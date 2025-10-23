fn main() {
    // println!(
    //     "cargo:rustc-env=BUILD_DATE={}",
    //     chrono::Utc::now()
    //         .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap())
    //         .format("%Y-%m-%d")
    // );
    // println!(
    //     "cargo:rustc-env=BUILD_TIME={}",
    //     chrono::Utc::now()
    //         .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap())
    //         .format("%H:%M:%S")
    // );

    // println!("cargo:rustc-env=RUSTC_VERSION={}", "1.90.0");
}

// fn get_rustc_version() -> String {
//     let output = std::process::Command::new("rustc")
//         .arg("--version")
//         .output()
//         .expect("Failed to execute rustc command");

//     if output.status.success() {
//         let version = String::from_utf8_lossy(&output.stdout);

//         version
//             .split_whitespace()
//             .take(2)
//             .collect::<Vec<_>>()
//             .join(" ")
//     } else {
//         "Unknown".to_string()
//     }
// }
