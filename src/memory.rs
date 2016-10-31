

pub trait Memory {
	 fn peek(&self, addr: usize) -> u8;
	 fn poke(&mut self, addr: usize, val: u8);

	 fn memcpy(&mut self, dest_addr: usize, source_addr: usize, len: usize, source: Option<&Memory>);
	 fn memset(&mut self, dest_addr: usize, val: u8, len: usize);
}
