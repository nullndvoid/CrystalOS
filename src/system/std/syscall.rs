/// THIS FILE IS ONLY FOR SPECIFIC CASES WHERE THE MAIN FUNCTION NEEDS DIRECT KERNEL INTERACTION

use crate::system::kernel::render::RENDERER;

pub fn terminal_mode_force() {
	RENDERER.lock().terminal_mode_force();
}