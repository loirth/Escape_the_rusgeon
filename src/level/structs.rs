#[derive(Copy, Clone)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LevelTextures { // all textures that can be in the level
	pub w: char,
	pub a: char,
	pub c: char,
	pub b: char,
	pub p: char,
}