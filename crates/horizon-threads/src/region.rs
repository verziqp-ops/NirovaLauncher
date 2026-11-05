//! Deterministic region-job executor.
//!
//! Jobs read immutable snapshots and return deltas. Workers never receive shared mutable world
//! state. Results and cross-region messages are ordered at the tick barrier. This executor is
//! infrastructure only: the live Game tick remains sequential until world ownership, RNG streams
//! and cross-region effects can use this protocol without changing vanilla ordering.

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

/// A region in one game dimension. One region contains 32 × 32 chunks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegionId {
    /// The game's stable dimension key, assigned by the caller.
    pub dimension: i32,
    pub x: i32,
    pub z: i32,
}

impl RegionId {
    pub const CHUNKS_PER_REGION: i32 = 32;

    /// Maps a chunk coordinate to its owning region. Euclidean division matters for negative
    /// coordinates: chunk -1 belongs to region -1, not region 0.
    pub fn from_chunk(dimension: i32, chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            dimension,
            x: chunk_x.div_euclid(Self::CHUNKS_PER_REGION),
            z: chunk_z.div_euclid(Self::CHUNKS_PER_REGION),
        }
    }
}

/// One immutable region snapshot for one tick. `order` is assigned by the caller from the
/// canonical sequential order, before any worker starts.
pub struct RegionJob<T> {
    pub tick: u64,
    pub order: u64,
    pub region: RegionId,
    pub snapshot: Arc<T>,
}

/// A worker's result. Results are returned in canonical `order`, not completion order.
#[derive(Debug, PartialEq, Eq)]
pub struct RegionResult<R> {
    pub tick: u64,
    pub order: u64,
    pub region: RegionId,
    pub output: R,
}

/// A cross-region effect emitted by a job. Its stable order is
/// `(tick, source_region, source_sequence, target_position)`. Messages are applied only after the
/// worker barrier, never by mutating another worker's snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionMessage<T> {
    pub tick: u64,
    pub source: RegionId,
    pub source_sequence: u64,
    pub target: RegionId,
    pub target_x: i32,
    pub target_y: i32,
    pub target_z: i32,
    pub payload: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegionScheduleError {
    MixedTicks,
    DuplicateJobOrder(u64),
    DuplicateMessageOrder {
        tick: u64,
        source: RegionId,
        source_sequence: u64,
    },
}

impl std::fmt::Display for RegionScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MixedTicks => write!(f, "a region batch contains more than one game tick"),
            Self::DuplicateJobOrder(order) => {
                write!(f, "region job order {order} is not unique")
            }
            Self::DuplicateMessageOrder {
                tick,
                source,
                source_sequence,
            } => write!(
                f,
                "region message order ({tick}, {source:?}, {source_sequence}) is not unique"
            ),
        }
    }
}

impl std::error::Error for RegionScheduleError {}

/// A fixed-size worker executor. It schedules immutable region snapshots concurrently and
/// returns outputs in the caller's canonical order.
pub struct RegionScheduler {
    workers: usize,
}

impl RegionScheduler {
    pub fn new(workers: usize) -> Self {
        Self {
            workers: workers.max(1),
        }
    }

    pub fn available() -> Self {
        Self::new(
            std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get),
        )
    }

    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn run<T, R, F>(
        &self,
        jobs: &[RegionJob<T>],
        run: F,
    ) -> Result<Vec<RegionResult<R>>, RegionScheduleError>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&RegionJob<T>) -> R + Sync,
    {
        if jobs.is_empty() {
            return Ok(Vec::new());
        }

        let tick = jobs[0].tick;
        if jobs.iter().any(|job| job.tick != tick) {
            return Err(RegionScheduleError::MixedTicks);
        }

        let mut order: Vec<usize> = (0..jobs.len()).collect();
        order.sort_by_key(|&index| (jobs[index].order, jobs[index].region));
        for pair in order.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            if jobs[left].order == jobs[right].order {
                return Err(RegionScheduleError::DuplicateJobOrder(jobs[left].order));
            }
        }

        let next = AtomicUsize::new(0);
        let results = Mutex::new(
            std::iter::repeat_with(|| None)
                .take(jobs.len())
                .collect::<Vec<Option<RegionResult<R>>>>(),
        );
        let worker_count = self.workers.min(jobs.len());

        std::thread::scope(|scope| {
            for _ in 0..worker_count {
                let next = &next;
                let order = &order;
                let results = &results;
                let run = &run;
                scope.spawn(move || loop {
                    let slot = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&index) = order.get(slot) else {
                        break;
                    };
                    let job = &jobs[index];
                    let result = RegionResult {
                        tick: job.tick,
                        order: job.order,
                        region: job.region,
                        output: run(job),
                    };
                    results.lock().expect("region result lock poisoned")[slot] = Some(result);
                });
            }
        });

        Ok(results
            .into_inner()
            .expect("region result lock poisoned")
            .into_iter()
            .map(|result| result.expect("every scheduled region job completes"))
            .collect())
    }
}

/// Orders cross-region messages at the tick barrier and rejects ambiguous sequence numbers.
pub fn order_messages<T>(
    mut messages: Vec<RegionMessage<T>>,
) -> Result<Vec<RegionMessage<T>>, RegionScheduleError> {
    messages.sort_by_key(|message| {
        (
            message.tick,
            message.source,
            message.source_sequence,
            message.target_x,
            message.target_y,
            message.target_z,
            message.target,
        )
    });
    for pair in messages.windows(2) {
        if pair[0].tick == pair[1].tick
            && pair[0].source == pair[1].source
            && pair[0].source_sequence == pair[1].source_sequence
        {
            return Err(RegionScheduleError::DuplicateMessageOrder {
                tick: pair[0].tick,
                source: pair[0].source,
                source_sequence: pair[0].source_sequence,
            });
        }
    }
    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    fn region(x: i32, z: i32) -> RegionId {
        RegionId {
            dimension: 0,
            x,
            z,
        }
    }

    fn job(tick: u64, order: u64, x: i32, z: i32, values: Vec<i32>) -> RegionJob<Vec<i32>> {
        RegionJob {
            tick,
            order,
            region: region(x, z),
            snapshot: Arc::new(values),
        }
    }

    #[test]
    fn negative_chunk_coordinates_use_euclidean_regions() {
        assert_eq!(RegionId::from_chunk(0, 0, 0), region(0, 0));
        assert_eq!(RegionId::from_chunk(0, 31, 31), region(0, 0));
        assert_eq!(RegionId::from_chunk(0, 32, 32), region(1, 1));
        assert_eq!(RegionId::from_chunk(0, -1, -1), region(-1, -1));
        assert_eq!(RegionId::from_chunk(0, -32, -32), region(-1, -1));
        assert_eq!(RegionId::from_chunk(0, -33, -33), region(-2, -2));
        assert_eq!(RegionId::from_chunk(-1, 0, 0).dimension, -1);
    }

    #[test]
    fn parallel_outputs_match_the_sequential_region_reference() {
        let jobs = [
            job(41, 3, 1, 0, vec![3, 4, 5]),
            job(41, 1, 0, 0, vec![1, 2]),
            job(41, 2, 0, 1, vec![6, 7, 8, 9]),
        ];
        let expected: Vec<_> = {
            let mut ordered: Vec<_> = jobs.iter().collect();
            ordered.sort_by_key(|job| job.order);
            ordered
                .into_iter()
                .map(|job| (job.tick, job.order, job.region, job.snapshot.iter().sum::<i32>()))
                .collect()
        };

        for workers in [1, 2, 4, 8] {
            let actual = RegionScheduler::new(workers)
                .run(&jobs, |job| job.snapshot.iter().sum::<i32>())
                .unwrap()
                .into_iter()
                .map(|result| (result.tick, result.order, result.region, result.output))
                .collect::<Vec<_>>();
            assert_eq!(actual, expected, "worker count {workers}");
        }
    }

    #[test]
    fn workers_really_run_independently() {
        let jobs = [job(1, 0, 0, 0, vec![1]), job(1, 1, 1, 0, vec![2])];
        let barrier = Barrier::new(2);
        let result = RegionScheduler::new(2)
            .run(&jobs, |_| {
                barrier.wait();
                1
            })
            .unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn completion_order_does_not_change_commit_order() {
        let jobs = [
            job(9, 30, 3, 0, vec![30]),
            job(9, 10, 1, 0, vec![10]),
            job(9, 20, 2, 0, vec![20]),
        ];
        let result = RegionScheduler::new(3)
            .run(&jobs, |job| {
                std::thread::sleep(std::time::Duration::from_millis(
                    3 * (30 - job.order) as u64,
                ));
                job.order
            })
            .unwrap();
        assert_eq!(
            result.iter().map(|r| r.order).collect::<Vec<_>>(),
            [10, 20, 30]
        );
    }

    #[test]
    fn messages_commit_in_canonical_source_order() {
        let messages = vec![
            RegionMessage {
                tick: 2,
                source: region(1, 0),
                source_sequence: 0,
                target: region(2, 0),
                target_x: 64,
                target_y: 5,
                target_z: 1,
                payload: "east",
            },
            RegionMessage {
                tick: 2,
                source: region(0, 0),
                source_sequence: 0,
                target: region(1, 0),
                target_x: 32,
                target_y: 5,
                target_z: 1,
                payload: "west",
            },
        ];
        let ordered = order_messages(messages).unwrap();
        assert_eq!(
            ordered.iter().map(|m| m.payload).collect::<Vec<_>>(),
            ["west", "east"]
        );
        let duplicate = vec![
            RegionMessage {
                tick: 1,
                source: region(0, 0),
                source_sequence: 7,
                target: region(1, 0),
                target_x: 32,
                target_y: 0,
                target_z: 0,
                payload: (),
            },
            RegionMessage {
                tick: 1,
                source: region(0, 0),
                source_sequence: 7,
                target: region(-1, 0),
                target_x: -1,
                target_y: 0,
                target_z: 0,
                payload: (),
            },
        ];
        assert_eq!(
            order_messages(duplicate),
            Err(RegionScheduleError::DuplicateMessageOrder {
                tick: 1,
                source: region(0, 0),
                source_sequence: 7
            })
        );
    }

    #[test]
    fn a_batch_cannot_mix_game_ticks_or_ambiguous_job_orders() {
        let mixed = [job(1, 0, 0, 0, vec![]), job(2, 1, 1, 0, vec![])];
        assert_eq!(
            RegionScheduler::new(2).run(&mixed, |_| ()),
            Err(RegionScheduleError::MixedTicks)
        );
        let duplicate = [job(1, 0, 0, 0, vec![]), job(1, 0, 1, 0, vec![])];
        assert_eq!(
            RegionScheduler::new(2).run(&duplicate, |_| ()),
            Err(RegionScheduleError::DuplicateJobOrder(0))
        );
    }
}
