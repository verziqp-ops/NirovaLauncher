use super::*;

// Legacy profile discovery remains covered for resource import compatibility.
fn find_local_java_folder(selected: &Path, id: &str, known: &[PathBuf]) -> Option<PathBuf> {
    std::iter::once(selected)
        .chain(known.iter().map(PathBuf::as_path))
        .find(|root| local_java_version_ready(root, id))
        .map(Path::to_path_buf)
}

#[test]
fn selected_jdk_reaches_external_preparers() {
    const CHILD: &str = "HORIZON_LAUNCHER_JAVA_TEST";
    if let Some(root) = std::env::var_os(CHILD) {
        let root = PathBuf::from(root);
        let home = java(25, true, &[], &root.join("missing.json"), &|_| {}).unwrap();
        assert_eq!(
            fs::canonicalize(&home).unwrap(),
            fs::canonicalize(root.join("jdk25")).unwrap()
        );
        // An older preparer invokes Java tools by name; both must now resolve to this JDK.
        for name in ["java", "javac"] {
            let mut cmd = crate::process::command(name);
            with_java(&mut cmd, &home);
            let output = cmd.arg("-version").output().unwrap();
            assert!(output.status.success());
            let text = String::from_utf8_lossy(&output.stdout).into_owned()
                + &String::from_utf8_lossy(&output.stderr);
            assert!(text.contains("25.0.1"), "{name}: {text}");
            assert_eq!(
                cmd.get_envs()
                    .find(|(key, _)| *key == "JAVA_HOME")
                    .unwrap()
                    .1,
                Some(home.as_os_str())
            );
        }
        return;
    }

    let root = std::env::temp_dir().join(format!("horizon-launcher-java-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let executable =
        |bin: &Path, name: &str| bin.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let probe = executable(&root, "probe");
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../crates/horizon-prepare/tests/support/java_probe.rs");
    let output = crate::process::command("rustc")
        .arg(source)
        .arg("-o")
        .arg(&probe)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for (home, compiler, runtime) in [
        ("jre21", None, "21.0.1"),
        ("jdk25", Some("25.0.1"), "25.0.1"),
    ] {
        let bin = root.join(home).join("bin");
        fs::create_dir_all(&bin).unwrap();
        for (name, version) in [("javac", compiler), ("java", Some(runtime))] {
            if let Some(version) = version {
                fs::copy(&probe, executable(&bin, name)).unwrap();
                fs::write(bin.join(format!("{name}.version")), version).unwrap();
            }
        }
    }
    let output = crate::process::command(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "backend::java_tests::selected_jdk_reaches_external_preparers",
            "--nocapture",
        ])
        .env(CHILD, &root)
        .env_remove("JAVA_HOME")
        .env(
            "PATH",
            std::env::join_paths([root.join("jre21/bin"), root.join("jdk25/bin")]).unwrap(),
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn local_guest_version_needs_both_profile_and_client_jar() {
    let root =
        std::env::temp_dir().join(format!("nirova-local-java-profile-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let version = root.join("versions").join("1.21.4");
    fs::create_dir_all(&version).unwrap();
    assert!(!local_java_version_ready(&root, "1.21.4"));
    fs::write(
        version.join("1.21.4.json"),
        r#"{"id":"1.21.4","mainClass":"net.minecraft.client.main.Main"}"#,
    )
    .unwrap();
    assert!(!local_java_version_ready(&root, "1.21.4"));
    fs::write(version.join("1.21.4.jar"), b"test jar").unwrap();
    assert!(local_java_version_ready(&root, "1.21.4"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn guest_discovers_an_installed_java_version_without_creating_game_files() {
    let root =
        std::env::temp_dir().join(format!("nirova-guest-autodiscovery-{}", std::process::id()));
    let selected = root.join("empty-choice");
    let installed = root.join("existing-minecraft");
    let version = installed.join("versions").join("1.21.4");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&selected).unwrap();
    fs::create_dir_all(&version).unwrap();
    fs::write(
        version.join("1.21.4.json"),
        r#"{"id":"1.21.4","mainClass":"net.minecraft.client.main.Main"}"#,
    )
    .unwrap();
    fs::write(
        version.join("1.21.4.jar"),
        b"existing locally installed jar",
    )
    .unwrap();

    assert_eq!(
        find_local_java_folder(&selected, "1.21.4", &[installed.clone()]),
        Some(installed)
    );
    assert_eq!(find_local_java_folder(&selected, "1.20.1", &[]), None);
    // Discovering a game must not fabricate a version in the selected folder.
    assert!(!selected.join("versions").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn java_version_download_folder_can_be_empty_and_is_persisted() {
    let root =
        std::env::temp_dir().join(format!("nirova-java-version-folder-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let empty = root.join("versions-go-here");
    let legacy = root.join("legacy-minecraft");
    let mut settings = Settings::default();

    let fallback = java_download_dir_for(&settings);
    assert_eq!(fallback, data_dir().join("minecraft"));

    fs::create_dir_all(legacy.join("versions")).unwrap();
    settings.minecraft_dir = Some(legacy.clone());
    assert_eq!(java_download_dir_for(&settings), legacy);

    // A newly chosen directory is authoritative without a versions/ subfolder.
    settings.java_versions_dir = Some(empty.clone());
    assert_eq!(java_download_dir_for(&settings), empty);
    assert!(!empty.join("versions").exists());
    let saved = serde_json::to_string(&settings).unwrap();
    let restored = Settings::parse(&saved);
    assert_eq!(restored.java_versions_dir.as_deref(), Some(empty.as_path()));
    assert_eq!(java_download_dir_for(&restored), empty);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn guest_version_download_rejects_unsafe_version_names_without_touching_disk() {
    let root = std::env::temp_dir().join(format!(
        "nirova-guest-version-safety-{}",
        std::process::id()
    ));
    let checksum = "0123456789abcdef0123456789abcdef01234567";
    let _ = fs::remove_dir_all(&root);
    for unsafe_id in ["", "../escape", "1.21/../other", "1.21\\escape"] {
        let err = download_version_for_guest_in(unsafe_id, checksum, &root, &|_| {}).unwrap_err();
        assert!(err.contains("Invalid Minecraft version ID"), "{err}");
    }
    let err = download_version_for_guest_in("1.21.4", "bad", &root, &|_| {}).unwrap_err();
    assert!(err.contains("invalid checksum"), "{err}");
    assert!(!root.exists());
}

#[test]
fn guest_installer_creates_a_verified_playable_java_version_from_public_manifest() {
    use std::collections::HashMap;
    use std::sync::atomic::AtomicBool;

    let root = std::env::temp_dir().join(format!(
        "nirova-guest-mojang-install-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let client = b"test Minecraft client bytes";
    let assets = br#"{"objects":{}}"#;
    let manifest = serde_json::json!({
        "id": "1.21.4",
        "mainClass": "net.minecraft.client.main.Main",
        "downloads": {
            "client": {
                "sha1": sources::sha1_hex(client),
                "size": client.len(),
                "url": "https://piston-data.mojang.com/guest-test.jar"
            }
        },
        "assetIndex": {
            "id": "guest-test",
            "sha1": sources::sha1_hex(assets),
            "url": "https://piston-meta.mojang.com/guest-assets.json"
        },
        "libraries": []
    })
    .to_string();
    let manifest_sha1 = sources::sha1_hex(manifest.as_bytes());
    let manifest_url =
        format!("https://piston-meta.mojang.com/v1/packages/{manifest_sha1}/1.21.4.json");
    let files = HashMap::from([
        (manifest_url, manifest.into_bytes()),
        (
            "https://piston-data.mojang.com/guest-test.jar".to_owned(),
            client.to_vec(),
        ),
        (
            "https://piston-meta.mojang.com/guest-assets.json".to_owned(),
            assets.to_vec(),
        ),
    ]);
    let asked = std::sync::Mutex::new(Vec::new());
    let fetch = |url: &str| {
        asked.lock().unwrap().push(url.to_owned());
        files
            .get(url)
            .cloned()
            .ok_or_else(|| format!("Mock does not have {url}"))
    };
    let cancel = AtomicBool::new(false);
    let install = || {
        download_version_for_guest_with(
            "1.21.4",
            &manifest_sha1,
            &root,
            "windows",
            &fetch,
            &cancel,
            &|_| {},
        )
    };
    assert_eq!(install().unwrap(), root);
    assert_eq!(
        fs::read(root.join("versions/1.21.4/1.21.4.jar")).unwrap(),
        client,
    );
    assert!(local_java_version_ready(&root, "1.21.4"));
    assert!(root.join("assets/indexes/guest-test.json").is_file());

    asked.lock().unwrap().clear();
    install().unwrap();
    assert!(
        asked.lock().unwrap().is_empty(),
        "Complete files should be reused"
    );
    fs::remove_dir_all(root).unwrap();
}
