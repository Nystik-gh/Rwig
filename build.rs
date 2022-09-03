// Example custom build script.
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    if let Ok(out_dir) = env::var("PATCH_OUT_DIR") {
        let patch_name = env::var("PATCH_NAME").unwrap_or("audio-patch".to_string());

        let profile = env::var("PROFILE").unwrap();

        let path_profile = Path::new(&profile);

        let target_path_32 = Path::new("target_32");
        let target_path = Path::new("target");

        let patch_path_32 = Path::new(&out_dir)
            .join(path_profile)
            .join(format!("{}_32.dll", patch_name));
        let patch_path = Path::new(&out_dir)
            .join(path_profile)
            .join(format!("{}.dll", patch_name));

        //println!("cargo:rerun-if-changed=build.rs");

        //println!("{}", &target_path_32.join(path_profile).join("patch.dll").to_str().unwrap());
        //println!("{}", &target_path.join(path_profile).join("patch.dll").to_str().unwrap());

        if !&target_path_32.exists() {
            Command::new("rustup")
                .current_dir("patch")
                .args(&[
                    "run",
                    "nightly-i686-pc-windows-msvc",
                    "cargo",
                    "build",
                    "--target-dir",
                    target_path_32.to_str().unwrap(),
                ])
                .status()
                .expect("failed to build patch 32");

            fs::remove_file(&patch_path_32).ok();
        }

        if !&patch_path_32.exists() {
            fs::create_dir_all(Path::new(&out_dir).join(path_profile)).ok();
            match fs::copy(
                Path::new("patch")
                    .join(&target_path_32)
                    .join(path_profile)
                    .join("patch.dll")
                    .to_str()
                    .unwrap(),
                &patch_path_32.to_str().unwrap(),
            ) {
                Ok(_) => println!("copy success"),
                Err(e) => println!(
                    "error: {:?}, {}",
                    e,
                    Path::new("patch")
                        .join(&target_path_32)
                        .join(path_profile)
                        .join("patch.dll")
                        .to_str()
                        .unwrap()
                ),
            };
        }

        if !&target_path_32.exists() {
            Command::new("rustup")
                .current_dir("patch")
                .args(&[
                    "run",
                    "nightly-x86_64-pc-windows-msvc",
                    "cargo",
                    "build",
                    "--target-dir",
                    target_path.to_str().unwrap(),
                ])
                .status()
                .expect("failed to build patch");

            fs::remove_file(&patch_path).ok();
        }

        if !&patch_path.exists() {
            fs::create_dir_all(Path::new(&out_dir).join(path_profile)).ok();
            match fs::copy(
                Path::new("patch")
                    .join(&target_path)
                    .join(path_profile)
                    .join("patch.dll")
                    .to_str()
                    .unwrap(),
                &patch_path.to_str().unwrap(),
            ) {
                Ok(_) => println!("copy success"),
                Err(e) => println!(
                    "error: {:?}, {}",
                    e,
                    Path::new("patch")
                        .join(&target_path)
                        .join(path_profile)
                        .join("patch.dll")
                        .to_str()
                        .unwrap()
                ),
            };
        }

        println!(
            "cargo:rerun-if-changed={}",
            &target_path_32.to_str().unwrap()
        );
        println!("cargo:rerun-if-changed={}", &patch_path.to_str().unwrap());

        println!("cargo:rerun-if-changed={}", &target_path.to_str().unwrap());
        println!(
            "cargo:rerun-if-changed={}",
            &target_path_32.to_str().unwrap()
        );
    }
}
