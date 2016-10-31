use std::num::Wrapping;


pub struct State {
    mem: [u8; 0x80000],
    rand_seed: u64,
}

impl State {
    pub fn new(seed: u64) -> State {
        State {
            mem: [0u8; 0x80000],
            rand_seed: seed
        }
    }

    pub fn poke(&mut self, addr: usize, val: u8) {
        self.mem[addr] = val;
    }

    pub fn peek(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    pub fn memcpy(&mut self, dest_addr: usize, source_addr: usize, len: usize) {
        let mut copy: Vec<u8> = Vec::with_capacity(len);
        for (i, item) in copy[0..len].iter_mut().enumerate() {
            *item = self.mem[source_addr + i];
        }
        for (i, item) in self.mem[dest_addr .. (dest_addr + len)].iter_mut().enumerate() {
            *item = copy[i];
        }
    }

    pub fn memset(&mut self, dest_addr: usize, val: u8, len: usize) {
        for item in &mut self.mem[dest_addr..(dest_addr+len)] {
            *item = val;
        }
    }

    pub fn pset(&mut self, x: u8, y: u8, c: &Color) {
        let x = x as usize;
        let y = y as usize;
        let idx = (x * 3) + (y * 256 * 3) + 0x50000;
        self.poke(idx    , c.0);
        self.poke(idx + 1, c.1);
        self.poke(idx + 2, c.2);
    }

    pub fn xorshift64star(&mut self) -> u64 {
        self.rand_seed ^= self.rand_seed >> 12; // a
        self.rand_seed ^= self.rand_seed << 25; // b
        self.rand_seed ^= self.rand_seed >> 27; // c
        return (Wrapping(self.rand_seed) * Wrapping(2685821657736338717u64)).0;
    }

    // pub fn rnd() -> 
}

pub struct StateWrapper {
    state: State
}

impl StateWrapper {
    pub fn new(seed: u64) -> StateWrapper {
        StateWrapper {
            state: State::new(seed)
        }
    }

    pub fn get_screen_slice(&self) -> &[u8] {
        &self.state.mem[0x50000 .. 0x80000]
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut State {
        &mut self.state 
    }

}

pub struct Color(pub u8, pub u8, pub u8);