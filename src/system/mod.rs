use crate::system::kernel::memory::BootInfoFrameAllocator;
use crate::system::kernel::{allocator, memory};
use bootloader::BootInfo;
use x86_64::VirtAddr;

mod kernel;
pub mod std;

pub fn init(boot_info: &'static BootInfo) {
    kernel::gdt::init();
    kernel::interrupts::init_idt();
    unsafe { kernel::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    kernel::sysinit::init().unwrap();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
}
