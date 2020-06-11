fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/nvim.ico");
        res.compile().expect("Could not attach exe icon");
    }

    #[cfg(target_os = "linux")]
    {
        use std::env;
        let cc = env::var("CC").unwrap();
        if let Ok(version) = process::Command::new(cc).arg("-dumpversion").output() {
            let local_ver = Version::from(version.stdout).unwrap();
            let affected_ver = Version::from("10").unwrap();

            if local_ver >= affected_ver {
                env::set_var("CFLAGS", "-fcommon -fPIE");
            }
        }
    }
}
