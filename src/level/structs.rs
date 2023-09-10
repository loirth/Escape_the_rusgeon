#[derive(Debug, Copy, Clone)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LevelTextures { // all textures that can be in the level
	pub wall: char,
	pub air: char,
	pub coin: char,
	pub bomb: char,
	pub player: char,
}