
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;


lazy_static! {
	pub static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = unsafe {
			SerialPort::new(0x3F8)
		};
		serial_port.init();
		Mutex::new(serial_port)
	};
}

#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		SERIAL1.lock().write_fmt(args).expect("unable to print to serial!")
	})

}
pub fn serial_reply(chr: char) -> char {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	let mut chr_return: char = 'X';

	interrupts::without_interrupts(|| {
		SERIAL1.lock().send(chr as u8);
		chr_return = SERIAL1.lock().receive() as char ;
	});

	chr_return
}

