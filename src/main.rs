use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let current_time = SystemTime::now();
    match current_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!(
                "Current time in nanoseconds since UNIX_EPOCH: {:?}",
                duration.as_nanos()
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
