pub mod std;
pub mod kernel;

pub fn init() {
	kernel::gdt::init();
	kernel::interrupts::init_idt();
	unsafe { kernel::interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
	kernel::sysinit::init().unwrap();
}