use config;

use runtime;

pub struct RuntimeEditor {
	mem: [u8; 1024*64]
}

impl Default for RuntimeEditor {
	fn default() -> RuntimeEditor {
		 RuntimeEditor {
			 mem: [0u8; 1024*64],
		 }
	}
}

impl<'a> runtime::Runtime<'a> for RuntimeEditor {
	fn update(&mut self) {

	}

	fn draw(&mut self) {

	}

	fn get_screen(&'a self) -> &'a [u8]{
		&self.mem
	}

	fn get_palette(&'a self)  -> &'a [u8] {
		&self.mem
	}

	fn update_mouse_state(&mut self, mouse_state: &runtime::MouseState) {

	}
}
