#![allow(non_snake_case)]
#![feature(libc, link_args)]
extern crate sdl2;
extern crate rand;
#[macro_use]
extern crate lua;
extern crate libc;

#[cfg(all(target_os="macos", target_arch="x86_64"))]
#[link_args = "-pagezero_size 10000 -image_base 100000000"]
extern "system" {}

use std::ffi::CString;

use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use std::mem;

use std::thread;

mod system;
use system::System;
mod system_sdl2;

mod runtime;
mod runtime_game;
mod runtime_editor;

mod memory;
mod memory_cartridge;
mod memory_runtime;

mod config;

mod cartridge;

// struct Memory([u8;16]);

// impl Memory {
//     fn new() -> Memory {
//         Memory([0;16])
//     }

//     fn push_to_lua(&mut self, L: &mut lua::State) {
//         unsafe {
//             pushlightuserdata_typed(L, self);
//             L.pushcclosure(Memory::poke, 1);
//             L.setglobal("poke");

//             pushlightuserdata_typed(L, self);
//             L.pushcclosure(Memory::peek, 1);
//             L.setglobal("peek");
//         }
//     }

//     lua_extern! {
//         unsafe fn poke(L: &mut lua::ExternState) -> i32 {
//             let mem: &mut Memory = tolightuserdata_typed::<Memory>(L, lua::upvalueindex(1)).as_mut().unwrap();
//             let idx = L.tointeger(1);
//             let val = L.tointeger(2);
//             mem.0[idx as usize] = val as u8;
//             0
//         }

//         unsafe fn peek(L: &mut lua::ExternState) -> i32 {
//             let mem: &mut Memory = tolightuserdata_typed::<Memory>(L, lua::upvalueindex(1)).as_mut().unwrap();
//             let idx = L.tointeger(1);
//             let val = mem.0[idx as usize];
//             L.pushinteger(val as isize);
//             1
//         }
//     }
// }



fn main() {

    // let mut L = lua::State::new();
    // L.open_table();
    // L.open_string();
    // L.open_math();
    // L.open_base();
    // L.pushnil(); // push the first key
    // while L.next(lua::GLOBALSINDEX) {
    //     println!("{} - {}", L.describe(-2), L.typename(-1));
    //     L.pop(1); // remove the value, keep the key
    // }
    // // L.settop(0);
    // let mut m = Memory::new();
    // m.push_to_lua(&mut L);
    // let r = L.dostring(r#"
    // while true do
    //     poke(0,0)
    // end
    // "#);
    // assert_eq!(r, true);
    // L.getglobal("foo");
    // println!("{:?}", L.type_(-1) == Some(lua::Type::Function));
    
    // L.call(0, 1);
    // println!("{:?} {:?}", m.0[0], m.0[1]);

    
    let mut system = system_sdl2::SystemSdl2::new();
    system.run();



//      unsafe {
//         let L : *mut lua::lua_State = lua::luaL_newstate();
//         lua::luaL_openlibs(L);
//         let status = lua::luaL_loadstring(L, CString::new(
// r#"io.write("The table the script received has:\n");
// x = 0
// for i = 1, #foo do
//   print(i, foo[i])
//   x = x + foo[i]
// end
// io.write("Returning data back to C\n");
// return x"#
//         ).unwrap().as_ptr());
//         if status != 0 {
//             println!("Couldn't load file.");
//             panic!("Couldn't load file.");
//         }
//         lua::lua_newtable(L);
//         for i in 1i32 .. 6 {
//             lua::lua_pushnumber(L, i as f64);
//             lua::lua_pushnumber(L, (i * 2) as f64);
//             lua::lua_rawset(L, -3);
//         }
//         lua::lua_setglobal(L,  CString::new("foo").unwrap().as_ptr());
//         let result = lua::lua_pcall(L, 0, lua::LUA_MULTRET, 0);
//         if result != 0 {
//             println!("Failed to run script.");
//             panic!("Failed to run script.");
//         }
//         let sum: lua::lua_Number = lua::lua_tonumber(L, -1);
//         println!("Script returned: {}", sum);
//         lua::lua_pop(L, 1);
//         lua::lua_close(L);

//     }
}




