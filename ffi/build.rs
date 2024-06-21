fn main() -> miette::Result<()> {
    // Pulling PerfUtils code from git.
    let output = std::process::Command::new("git")
        .args(&[
            "clone",
            "https://github.com/PlatformLab/PerfUtils.git",
            "./PerfUtils",
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Submodule cloned successfully");
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to clone submodule: {}", error_message);
    }

    // Generate ffi related to TimeTrace.
    let path = std::path::PathBuf::from("./PerfUtils/src");
    let path2 = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path, &path2])
        .build()
        .unwrap();
    b.flag_if_supported("-std=c++14").compile("ffi");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Compile PerfUtils library.
    cc::Build::new()
        .cpp(true)
        .file("./PerfUtils/src/CacheTrace.cc")
        .file("./PerfUtils/src/Cycles.cc")
        .file("./PerfUtils/src/mkdir.cc")
        .file("./PerfUtils/src/Perf.cc")
        .file("./PerfUtils/src/Stats.cc")
        .file("./PerfUtils/src/TimeTrace.cc")
        .file("./PerfUtils/src/Util.cc")
        .compile("PerfUtils");

    Ok(())
}
