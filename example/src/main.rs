use std::thread::sleep;
use std::time::Duration;
use timetrace_ffi::*;

fn main() {
    record("Start test");
    sleep(Duration::from_nanos(1_000_000_000));
    record("Do something");
    sleep(Duration::from_nanos(2_000_000_000));
    record("Finished test");
    print();
}
