#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use CrystalOS::std::tasks::{Executor, Task};
use CrystalOS::{print, print_log, printerr, println, println_log, std::syscall};
extern crate alloc;
use CrystalOS::user::bin::shell;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    syscall::terminal_mode_force();
    printerr!("{}", _info);
    CrystalOS::hlt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CrystalOS::test_panic_handler(info)
}


entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    CrystalOS::start(boot_info);

    #[cfg(test)]
    test_main();

    // runs the 'mainloop' of the OS;
    let mut executor = Executor::new();
    executor.spawn(Task::new(shell::command_handler()));
    loop {
        executor.try_run();
    }



    loop {}
}

