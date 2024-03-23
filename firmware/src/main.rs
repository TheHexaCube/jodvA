#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

#[inline(never)]
#[panic_handler]
fn skill_issue(_: &PanicInfo) -> ! {
    loop {}
}
