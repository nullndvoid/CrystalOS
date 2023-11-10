#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use CrystalOS::kernel::tasks::{executor::Executor, Task};
use CrystalOS::{kernel, print, print_log, printerr, println, println_log};
extern crate alloc;
use CrystalOS::user::bin::shell;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::render::RENDERER.lock().terminal_mode_force();
    printerr!("{}", _info);
    CrystalOS::hlt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CrystalOS::test_panic_handler(info)
}

// some comment

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use CrystalOS::kernel::allocator;
    use CrystalOS::kernel::memory;
    use CrystalOS::kernel::memory::BootInfoFrameAllocator;

    CrystalOS::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");

    let mut executor = Executor::new();

    executor.spawn(Task::new(shell::command_handler()));

    loop {
        executor.try_run();
    }

    #[cfg(test)]
    test_main();

    loop {}
}
