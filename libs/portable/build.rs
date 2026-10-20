fn main() {
    #[cfg(windows)]
    {
        use std::io::Write;

        println!("cargo:rerun-if-env-changed=RUSTDESK_PORTABLE_REQUIRE_ADMIN");
        println!("cargo:rerun-if-changed=manifest-admin.xml");
        let manifest = if std::env::var_os("RUSTDESK_PORTABLE_REQUIRE_ADMIN").is_some() {
            "manifest-admin.xml"
        } else {
            "../../res/manifest.xml"
        };
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file(manifest);
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
