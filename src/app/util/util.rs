use std::time::Instant;

pub fn calculate_elapsed_duration(start_time: Instant) -> (u128, u128, u128, u128, u128) {
    let elapsed = start_time.elapsed();
    let millis = elapsed.as_millis();
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    (days, hours % 24, minutes % 60, seconds % 60, millis % 1000)
}
