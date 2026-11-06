# NIROVA low-end performance profile

New profiles start with a conservative graphics configuration intended to reduce GPU and CPU load on older laptops.

| Setting | First-run value | Why |
|---|---:|---|
| Graphics preset | Fast | Avoids the heavier graphics preset |
| Render distance | 6 chunks | Reduces visible chunk count |
| Simulation distance | 5 chunks | Reduces active world simulation area |
| Maximum FPS | 60 | Caps unnecessary frame production |
| VSync | On | Lets the display pace frames |
| Clouds | Off | Removes cloud rendering |
| Cloud range | 64 | Smaller cloud work area if enabled later |
| Mipmap levels | 0 | Avoids mipmap generation/storage work |
| Entity distance | 0.5 | Reduces distant entity rendering |
| Entity shadows | Off | Removes entity shadow rendering |
| Biome blend | 0 | Avoids biome color blending work |
| Weather radius | 4 | Reduces weather effect area |

These are defaults for a fresh options file. Existing player settings are preserved; they are not overwritten.

## What this does not guarantee

- It does not guarantee that every old laptop can start the game. GPU feature support, graphics drivers, RAM, CPU and operating-system limits still matter.
- It is not an FPS benchmark. No performance gain is claimed until the same world, view distance, resolution and hardware are measured before and after.
- It does not reduce server tick cost or fix CPU-heavy world simulation. That requires separate profiling and safe simulation work.
- It does not replace a low-resolution mode, texture-memory budget, adaptive chunk loading or a tested integrated-GPU fallback. Those remain follow-up work.

## Validation plan

Benchmark at least one low-end integrated GPU laptop and one older CPU system. Record average FPS, 1% low FPS, frame time, memory use, chunk generation time and server MSPT. Compare the defaults against the prior profile in the same saved world and at the same resolution. Do not publish performance claims without the reports.
