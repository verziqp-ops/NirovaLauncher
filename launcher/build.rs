//! Embeds Nirova's icon (`packaging/icons/nirova.ico`) in the Windows executables, as icon
//! resource 1: Explorer and the task bar show it, and the window takes it as its icon.
fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let ico = format!("{dir}/../packaging/icons/nirova.ico");
    println!("cargo:rerun-if-changed={ico}");
    println!("cargo:rerun-if-env-changed=NIROVA_LAUNCHER_VERSION");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon(&ico);
        let version = std::env::var("NIROVA_LAUNCHER_VERSION")
            .unwrap_or_else(|_| std::env::var("CARGO_PKG_VERSION").unwrap());
        resource.set("CompanyName", "NIROVA");
        resource.set("FileDescription", "NIROVA Launcher");
        resource.set("ProductName", "NIROVA Launcher");
        resource.set("OriginalFilename", "nirova-launcher.exe");
        resource.set("InternalName", "nirova-launcher");
        resource.set("FileVersion", &version);
        resource.set("ProductVersion", &version);
        if let Err(e) = resource.compile() {
            panic!("embedding {ico}: {e}");
        }
    }
}
