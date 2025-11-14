use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("external/sqlite-vec").exists() {
        println!("cargo:warning=Initializing sqlite-vec submodule...");
        let status = Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .status()
            .expect("Failed to initialize submodules");
        if !status.success() {
            panic!(
                "Failed to initialize submodules, git exited with code {}",
                status.code().unwrap()
            );
        }
    }

    let sqlite3_flags = match Command::new("pkg-config")
        .current_dir("external/sqlite-vec")
        .args(["sqlite3", "--libs", "--cflags"])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            println!("cargo:warning=pkg-config failed, using fallback flags");
            if cfg!(target_os = "macos") {
                "-lsqlite3".to_string()
            } else {
                panic!("Unable to find sqlite3 libraries")
            }
        }
    };

    println!("cargo:warning=Using SQLite flags: {}", sqlite3_flags);
    println!("cargo:warning=Building sqlite-vec extension...");
    let make_status = Command::new("make")
        .current_dir("external/sqlite-vec")
        .args([
            "loadable",
            format!("CFLAGS+=\"{}\"", sqlite3_flags).as_str(),
        ])
        .status()
        .expect("Failed to run make");

    if !make_status.success() {
        panic!("Make command failed to build sqlite-vec");
    }

    println!("cargo:rustc-link-search=native=external/sqlite-vec");
    println!("cargo:rerun-if-changed=external/sqlite-vec");
}
