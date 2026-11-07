//! Source-pinned native bootstrap shipped with the launcher, independent of online updates.
use crate::backend::{self, Release};
use std::path::{Path, PathBuf};

pub const PACK: &str = "nirova-runtime.pack";
const MANIFEST: &str = include_str!("../runtime-manifest.json");
fn manifest() -> &'static serde_json::Value {
    static VALUE: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
    VALUE.get_or_init(|| serde_json::from_str(MANIFEST).expect("compiled runtime manifest"))
}

pub struct Info {
    pub tag: String,
    pub name: String,
    pub asset: String,
    pub sha256: String,
    pub size: u64,
}

pub fn info() -> Option<Info> {
    let manifest = manifest();
    let platform = manifest["platforms"].as_object()?.values().find(|p| {
        p["os"].as_str() == Some(std::env::consts::OS)
            && p["arch"].as_str() == Some(std::env::consts::ARCH)
    })?;
    Some(Info {
        tag: if option_env!("NIROVA_BUNDLED_RUNTIME_SIGNED") == Some("true") {
            format!("{}-signed", manifest["tag"].as_str()?)
        } else {
            manifest["tag"].as_str()?.into()
        },
        name: manifest["name"].as_str()?.into(),
        asset: platform["asset"].as_str()?.into(),
        sha256: option_env!("NIROVA_BUNDLED_RUNTIME_SHA256")
            .unwrap_or(platform["sha256"].as_str()?)
            .into(),
        size: option_env!("NIROVA_BUNDLED_RUNTIME_SIZE")
            .and_then(|s| s.parse().ok())
            .unwrap_or(platform["size"].as_u64()?),
    })
}

pub fn path_for(exe: &Path) -> PathBuf {
    let parent = backend::app_bundle(exe)
        .map(|a| a.join("Contents/Resources"))
        .unwrap_or_else(|| exe.parent().unwrap_or(Path::new(".")).to_path_buf());
    if let Some(info) = info() {
        let addressed = parent.join(format!("nirova-runtime-{}.pack", info.sha256));
        if addressed.is_file() {
            return addressed;
        }
    }
    parent.join(PACK)
}

pub fn path() -> PathBuf {
    path_for(backend::exe_path())
}
pub fn is_bundled(tag: &str) -> bool {
    let tag = if option_env!("NIROVA_BUNDLED_RUNTIME_SIGNED") == Some("true") {
        tag.strip_suffix("-signed")
    } else {
        Some(tag)
    };
    manifest()["tag"].as_str() == tag
}

pub fn release() -> Option<Release> {
    let info = info()?;
    path().is_file().then(|| Release {
        tag: info.tag,
        name: info.name,
        body: String::new(),
        date: "26.3 · Rust".into(),
        asset: Some(("bundled:nirova-runtime.pack".into(), info.size)),
        prerelease: true,
        nightly: None,
    })
}

/// Local bootstrap remains usable while GitHub is slow, offline or has no public game releases.
pub fn merge(remote: Result<Vec<Release>, String>) -> Result<Vec<Release>, String> {
    let Some(local) = release() else {
        return remote;
    };
    let mut all = remote.unwrap_or_default();
    if !all.iter().any(|r| r.tag == local.tag) {
        all.push(local);
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_bootstrap_matches_the_resource_protocol_and_is_found_in_each_package_layout() {
        let m: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
        assert_eq!(m["minecraft"], horizon_core::version::VERSION_ID);
        assert_eq!(m["platforms"].as_object().unwrap().len(), 4);
        for p in m["platforms"].as_object().unwrap().values() {
            assert_eq!(p["sha256"].as_str().unwrap().len(), 64);
            assert!(p["size"].as_u64().unwrap() > 1_000_000);
        }
        assert_eq!(
            path_for(Path::new("/tmp/NIROVA.app/Contents/MacOS/nirova-launcher")),
            Path::new("/tmp/NIROVA.app/Contents/Resources/nirova-runtime.pack")
        );
        assert_eq!(
            path_for(Path::new("/tmp/bin/nirova-launcher")),
            Path::new("/tmp/bin/nirova-runtime.pack")
        );
    }
}
