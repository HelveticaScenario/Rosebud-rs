#[derive(Debug)]
pub enum ScreenMode {
	GameMode,
	EditorMode,
}

pub trait System {
	fn run(&mut self);
}
