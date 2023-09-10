use crate::level::{structs::Position};
use crate::generate_coins;

pub struct Player {
	pub position: Position,
	pub direction: char,
	/*  textures   */
	pub rotation_frames: [char; 4], // 0 - w(up), 1 - a(left), 2 - s(down), 3 - d(right)
	pub texture: char,
	/*  collision  */
	pub collision_mask: Vec<[char; 2]>, // with what sprites will collide this one
	/* "inventory" */
	pub coin_count: u32,
	pub coins_needed_for_win: u32,
	pub is_win_the_game: bool,
}


impl Player {
	pub fn walk(&mut self, air_texture: char, map: &mut Vec<Vec<char>>) {
		let mut position_x = self.position.x;
		let mut position_y = self.position.y;
		match self.direction { // I use a function to add animations in future
  /*up*/		'w' | 'W' => {
				position_y -= 1;
				self.texture = self.rotation_frames[0];
			},
  /*left*/		'a' | 'A' => {
				position_x -= 1;
				self.texture = self.rotation_frames[1];
			},
  /*down*/		's' | 'S' => {
				position_y += 1;
				self.texture = self.rotation_frames[2];
			},
  /*right*/		'd' | 'D' => {
				position_x += 1;
				self.texture = self.rotation_frames[3];
			},
			_ => {},
		}

		if !self.is_collide(map[position_y][position_x], self.collision_mask.clone()) {
			map[self.position.y][self.position.x] = air_texture;

			if self.is_collide_with_coin(map[position_y][position_x]) {
				self.coin_count += 1;
				if self.coin_count >= self.coins_needed_for_win {self.is_win_the_game = true;};
				generate_coins(1, map[position_y][position_x], ' ', map);
			}

			self.position.x = position_x;
			self.position.y = position_y;
		}
		map[self.position.y][self.position.x] = self.texture;
	}

	fn is_collide_with_coin(&self, tile: char) -> bool {
		for collider in self.collision_mask.iter() {
			if tile == collider[1] {
				if collider[0] == 'c' { return true; }
			}
		}
		return false;
	}
}


impl Sprite for Player {
	fn is_collide(&mut self, tile: char, _collision_mask: Vec<[char; 2]>) -> bool {
		for collider in self.collision_mask.iter() {
			if tile == collider[1] {
				match collider[0] {
					'w' => return true,
					_ => {},
				}
			}
		}
		return false;
	}
}


trait Sprite {
	fn is_collide(&mut self, tile: char, collision_mask: Vec<[char; 2]>) -> bool {
		for collider in collision_mask.iter() {
			if tile == collider[1] {
				match collider[0] {
					'w' => return true,
					_ => {},
				}
			}
		}
		return false;
	}
}