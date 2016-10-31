#[macro_use]
use lua;

use ::std::ops::Range;

use config;

use runtime;

pub struct RuntimeGame {
	mem: runtime::RuntimeMem,
	_screen_range: Range<usize>,
	_palette_range: Range<usize>,
	l_state: lua::State,
}

impl Default for RuntimeGame {
	fn default() -> RuntimeGame {
		let mut l = lua::State::new();
		l.openlibs();
		let mut mem = [0u8; 512*1024];
		let palette = runtime::default_palette();
		let end = mem.len() - (config::SCREEN_HEIGHT * config::SCREEN_WIDTH);
		let begin = end - config::PALETTE_SIZE;
		for (i, val) in (&mut mem[begin..end]).iter_mut().enumerate() {
			*val = palette[i];
		}

		let mut r = RuntimeGame {
			mem: mem,
			_screen_range: (end..mem.len()),
			_palette_range: (begin..end),
			l_state: l,
		};
		r.push_to_lua();
		return r;
	}
}

impl<'a> runtime::Runtime<'a> for RuntimeGame {
	fn update(&mut self) {
		self.l_state.getglobal("update");
		if self.l_state.type_(-1) == Some(lua::Type::Function) {
			self.l_state.call(0, 0);
		}
	}

	fn draw(&mut self) {
		self.l_state.getglobal("draw");
		if self.l_state.type_(-1) == Some(lua::Type::Function) {
			self.l_state.call(0, 0);
		}
		// for (i, val) in (&mut self.mem[self._screen_range.clone()]).iter_mut().enumerate() {
		// 	*val = (i as u8)%16;
		// }
	}

	fn get_screen(&'a self) -> &'a [u8]{
		&self.mem[self._screen_range.clone()]
	}

	fn get_palette(&'a self)  -> &'a [u8] {
		&self.mem[self._palette_range.clone()]
	}

	fn update_mouse_state(&mut self, mouse_state: &runtime::MouseState) {

	}
}

impl RuntimeGame {
	fn push_to_lua(&mut self) {
        unsafe {
			self.push_function(RuntimeGame::poke, "poke");
			self.push_function(RuntimeGame::peek, "peek");
			// self.push_function(RuntimeGame::pset, "pset");
			// self.push_function(RuntimeGame::pget, "pget");
        }
    }

	unsafe fn push_function(&mut self, f: lua::CFunction, s: &str) {
		runtime::pushlightuserdata_typed(&mut self.l_state, &mut self.mem);
		self.l_state.pushcclosure(f, 1);
		self.l_state.setglobal(s);
	}

	lua_extern! {
        unsafe fn poke(L: &mut lua::ExternState) -> i32 {
            let mem: &mut runtime::RuntimeMem = runtime::tolightuserdata_typed::<runtime::RuntimeMem>(L, lua::upvalueindex(1)).as_mut().unwrap();
            let idx = L.tointeger(1);
            let val = L.tointeger(2);
            mem[idx as usize] = val as u8;
            0
        }

        unsafe fn peek(L: &mut lua::ExternState) -> i32 {
            let mem: &mut runtime::RuntimeMem = runtime::tolightuserdata_typed::<runtime::RuntimeMem>(L, lua::upvalueindex(1)).as_mut().unwrap();
			let idx = L.tointeger(1);
            let val = mem[idx as usize];
            L.pushinteger(val as isize);
            1
        }

		// unsafe fn pset(L: &mut lua::ExternState) -> i32 {
        //     let mem: &mut runtime::RuntimeMem = runtime::tolightuserdata_typed::<runtime::RuntimeMem>(L, lua::upvalueindex(1)).as_mut().unwrap();
        //     let x = L.tointeger(1);
        //     let y = L.tointeger(2);
        //     let val = L.tointeger(3);
        //     mem[idx as usize] = val as u8;
        //     0
        // }

        // unsafe fn pget(L: &mut lua::ExternState) -> i32 {
        //     let mem: &mut runtime::RuntimeMem = runtime::tolightuserdata_typed::<runtime::RuntimeMem>(L, lua::upvalueindex(1)).as_mut().unwrap();
		// 	let idx = L.tointeger(1);
        //     let val = mem[idx as usize];
        //     L.pushinteger(val as isize);
        //     1
        // }
    }
}

