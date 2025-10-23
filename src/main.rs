use std::time::Instant;

use rs_service_cloud::app::{
    app_serve,
    util::{calculate_elapsed_duration, helper},
};
fn main() {
    let start_time = Instant::now();

    let app_name = env!("CARGO_PKG_NAME"); // Get
    let version = env!("CARGO_PKG_VERSION"); // Get the version from Cargo.toml

    let build_date = env!("BUILD_DATE");
    let build_time = env!("BUILD_TIME");
    let build_date_time = format!("{build_date} {build_time} UTC+7");

    let platform = helper::get_platform();
    let rustc_version = env!("RUSTC_VERSION");

    println!(
        "--- (Begin {app_name} v{version} | Built with {rustc_version} on {platform} at {build_date_time}) ---",
    );

    println!();

    // trace!("detailed tracing info");
    // debug!("debug info");
    // info!("relevant general info");
    // warn!("warning this program doesn't do much");
    // error!("error message here");

    app_serve::run();

    println!();

    let (days, hours, minutes, seconds, millis) = calculate_elapsed_duration(start_time);

    println!(
        "--- End ({days} days, {hours} hours, {minutes} minutes, {seconds} seconds, {millis} millis) ---"
    );
}
