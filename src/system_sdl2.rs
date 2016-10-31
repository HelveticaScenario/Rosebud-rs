use sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::event::WindowEventId;
use sdl2::keyboard::Keycode;

use config;
use system;
use runtime_editor;
use runtime_game;
use runtime::Runtime;

pub struct SystemSdl2<'a> {
	context: sdl2::Sdl,
	renderer: sdl2::render::Renderer<'a>,
	texture: sdl2::render::Texture,
	window_width: u32,
	window_height: u32,
	window_width_mult: u32,
	window_height_mult: u32,
	screen_mode: system::ScreenMode,
	game: runtime_game::RuntimeGame,
	editor: runtime_editor::RuntimeEditor,
}

impl<'a> SystemSdl2<'a> {
	pub fn new() -> SystemSdl2<'a> {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();

		let window = video_subsystem.window("Rosebud", config::SCREEN_WIDTH as u32 *3, config::SCREEN_HEIGHT as u32 *3)
			.position_centered()
			.resizable()
			.allow_highdpi()
			.opengl()
			.build()
			.unwrap();

		let (window_width, window_height) = window.drawable_size();
		let window_width_mult = window_width / config::SCREEN_WIDTH as u32;
		let window_height_mult = window_height / config::SCREEN_HEIGHT as u32;

		let renderer = window
			.renderer()
			.accelerated()
			.present_vsync()
			.build()
			.unwrap();

		video_subsystem.gl_set_swap_interval(1);
		let texture = renderer.create_texture_streaming(
			PixelFormatEnum::RGB24, config::SCREEN_WIDTH as u32, config::SCREEN_HEIGHT as u32).unwrap();

		SystemSdl2 {
			context: sdl_context,
			renderer: renderer,
			texture: texture,
			window_width: window_width,
			window_height: window_height,
			window_width_mult: window_width_mult,
			window_height_mult: window_height_mult,
			screen_mode: system::ScreenMode::GameMode,
			game: Default::default(),
			editor: Default::default()
		}
	}
}

impl<'a> system::System for SystemSdl2<'a> {
	fn run(&mut self) {
		let mut event_pump = self.context.event_pump().unwrap();
		let id = self.renderer.window().unwrap().id();
		let num = config::SCREEN_HEIGHT*config::SCREEN_WIDTH*3;

		'running: loop {
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} 
					| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running
					},
					Event::Window {window_id, win_event_id, ..} => {
						if window_id == id {
							match win_event_id {
								WindowEventId::SizeChanged => {
									let (window_width, window_height) = self.renderer.window().unwrap().drawable_size();
									self.window_height = window_height;
									self.window_width = window_width;
									self.window_width_mult = window_width / config::SCREEN_WIDTH as u32;
									self.window_height_mult = window_height / config::SCREEN_HEIGHT as u32;
								},
								_ => {}
							}
						}
					},
					_ => {}
				}
			}

			let (screen, palette) = match self.screen_mode {
				system::ScreenMode::GameMode => {
					self.game.update();
					self.game.draw();
					(self.game.get_screen(), self.game.get_palette())
				},
				system::ScreenMode::EditorMode => {
					self.editor.update();
					self.editor.draw();
					(self.editor.get_screen(), self.editor.get_palette())
				},
			};

			self.texture.with_lock(None, |buffer: &mut [u8], _: usize| {
				for (i, val) in screen.iter().enumerate() {
					let offset = i * 3;
					let p_offset = (val * 3) as usize;
					buffer[offset] = palette[p_offset];
					buffer[offset + 1] = palette[p_offset + 1];
					buffer[offset + 2] = palette[p_offset + 2];
				}
			}).unwrap();
			self.renderer.clear();
			
			let mult = if self.window_width_mult < self.window_height_mult {self.window_width_mult} else {self.window_height_mult};
			let rect = Rect::new(
				(self.window_width as i32 - (config::SCREEN_WIDTH as i32 * mult as i32)) / 2,
				(self.window_height as i32 - (config::SCREEN_HEIGHT as i32 * mult as i32)) / 2,
				config::SCREEN_WIDTH as u32 * mult as u32,
				config::SCREEN_HEIGHT as u32 * mult as u32,
			);

			self.renderer.copy(&self.texture, None, Some(rect)).unwrap();
			
			self.renderer.present();
		}
	}
}