use autocxx::prelude::*;

include_cpp! {
    // C++ headers we want to include.
    #include "include.h"

    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)

    // What types and functions we want to generate.
    generate!("timetrace_set_output_filename")
    generate!("timetrace_print")
    generate!("timetrace_record_with_args")
    generate!("timetrace_record")
    generate!("timetrace_set_keepoldevents")
}

use crate::ffi::*;
use std::ffi::CString;

/// Convert Rust's static str to a raw pointer.
fn convert_str_to_ptr(format: &'static str) -> *const i8 {
    let c_string = CString::new(format).expect("CString::new failed");
    c_string.into_raw()
}

/// Set the log output file. If not set, calling print will print on standard output.
pub fn set_output_filename(format: &'static str) {
    let c_ptr = convert_str_to_ptr(format);
    unsafe {
        timetrace_set_output_filename(c_ptr);
    }
}

/// The recording function of timetrace will record the time of the call and print it
/// when calling the print function. In addition to string markers, this function also
/// supports adding additional parameters to record additional information.
pub fn record_with_args(format: &'static str, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
    let c_ptr = convert_str_to_ptr(format);
    unsafe {
        timetrace_record_with_args(c_ptr, arg0, arg1, arg2, arg3);
    }
}

/// The recording function of timetrace will record the time of the call and print it
/// when calling the print function.
pub fn record(format: &'static str) {
    let c_ptr = convert_str_to_ptr(format);
    unsafe {
        timetrace_record(c_ptr);
    }
}

/// Print the results of all records.
pub fn print() {
    timetrace_print();
    println!()
}

/// The timetrace uses a buffer to record events, and this function sets the keep flag
/// to indicate whether to print the old values saved in the buffer before.
pub fn set_keepoldevents(keep: bool) {
    timetrace_set_keepoldevents(keep);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        set_keepoldevents(true);
        set_output_filename("timetrace_wrapper_test.log");
        record("Start of execution");
        let mut _sum: u64 = 0;
        for i in 0..(1 << 20) {
            _sum += i;
        }
        record_with_args("This record with args", 0, 1, 2, 3);
        record("End of a counting loop");
        record("Hello world");
        print();
    }
}
