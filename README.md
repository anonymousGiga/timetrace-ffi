# timetrace-ffi
timetrace-ffi is the encapsulation of [TimeTrace in the C++ version](https://github.com/PlatformLab/PerfUtils) under Rust.

# Usage

1. Add the following to your Cargo.toml:
```
[dependencies]
timetrace-ffi = { git= "https://github.com/megaeth-labs/timetrace-ffi.git" }
```

2. Instrument your code like this: 
```
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
```

