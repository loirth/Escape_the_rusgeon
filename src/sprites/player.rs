use crate::structs::Position;

pub struct Player {
	pub anim_frames: [char; 4], // 0 - w(up), 1 - a(left), 2 - s(down), 3 - d(right)
	pub texture: char,
	pub position: Position,
	pub direction: char,
	pub coin_count: u32,
}

impl Player {
	pub fn walk(&mut self) {
		match self.direction { // I use a function to add animations in future
			'w' | 'W' => {
				self.position.y -= 1;
				self.texture = self.anim_frames[0];
			},
			'a' | 'A' => {
				self.position.x -= 1;
				self.texture = self.anim_frames[1];
			},
			's' | 'S' => {
				self.position.y += 1;
				self.texture = self.anim_frames[2];
			},
			'd' | 'D' => {
				self.position.x += 1;
				self.texture = self.anim_frames[3];
			},
			_ => {},
		}
	}
}