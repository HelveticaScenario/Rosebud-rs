use config;

use lua;

use ::std::mem;

pub type RuntimeMem = [u8; 512*1024];

pub unsafe fn pushlightuserdata_typed<T>(L: &mut lua::State, ud: *mut T) {
    L.pushlightuserdata(mem::transmute(ud))
}

pub unsafe fn tolightuserdata_typed<T>(L: &mut lua::ExternState, idx: i32) -> *mut T {
    assert!(L.islightuserdata(idx));
    mem::transmute(L.touserdata(idx))
}

pub fn default_palette() -> [u8; config::PALETTE_SIZE] {
	let mut p = [0u8; config::PALETTE_SIZE];
	p[0 *3] =   0; p[0 *3 + 1] =   0; p[0 *3 + 2] =   0; // black
	p[1 *3] =  29; p[1 *3 + 1] =  43; p[1 *3 + 2] =  83; // dark-blue
	p[2 *3] = 126; p[2 *3 + 1] =  37; p[2 *3 + 2] =  83; // dark-purple
	p[3 *3] =   0; p[3 *3 + 1] = 135; p[3 *3 + 2] =  81; // dark-green
	p[4 *3] = 171; p[4 *3 + 1] =  82; p[4 *3 + 2] =  54; // brown
	p[5 *3] =  95; p[5 *3 + 1] =  87; p[5 *3 + 2] =  79; // dark-gray
	p[6 *3] = 194; p[6 *3 + 1] = 195; p[6 *3 + 2] = 199; // light-gray
	p[7 *3] = 255; p[7 *3 + 1] = 241; p[7 *3 + 2] = 232; // white
	p[8 *3] = 255; p[8 *3 + 1] =   0; p[8 *3 + 2] =  77; // red
	p[9 *3] = 255; p[9 *3 + 1] = 164; p[9 *3 + 2] =   0; // orange
	p[10*3] = 255; p[10*3 + 1] = 236; p[10*3 + 2] =  39; // yellow
	p[11*3] =   0; p[11*3 + 1] = 228; p[11*3 + 2] =  54; // green
	p[12*3] =  41; p[12*3 + 1] = 173; p[12*3 + 2] = 255; // blue
	p[13*3] = 131; p[13*3 + 1] = 118; p[13*3 + 2] = 156; // indigo
	p[14*3] = 255; p[14*3 + 1] = 119; p[14*3 + 2] = 168; // pink
	p[15*3] = 255; p[15*3 + 1] = 204; p[15*3 + 2] = 170; // peach
	return p;
}

#[derive(Debug)]
pub struct MouseState {
	x: Option<u16>,
	y: Option<u16>,
	rel_x: u16,
	rel_y: u16,
	left_btn_down: bool,
	middle_btn_down: bool,
	right_btn_down: bool,
	x1_btn_down: bool,
	x2_btn_down: bool,
}

pub trait Runtime<'a> {
	fn update(&mut self);
	fn draw(&mut self);
	fn get_screen(&'a self) -> &'a [u8];
	fn get_palette(&'a self) -> &'a [u8];
	fn update_mouse_state(&mut self, mouse_state: &MouseState);
}
