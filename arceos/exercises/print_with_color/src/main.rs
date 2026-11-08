#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use axstd::os::arceos::modules::axlog::trace;
#[cfg(feature = "axstd")]
use axstd::println;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    trace!("trace test");
    println!("[WithColor]: Hello, Arceos!");
}
