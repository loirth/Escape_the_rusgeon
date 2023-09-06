use rand::{thread_rng, Rng};
use crate::{Position, LevelTextures};

pub fn print_map(map: &Vec<Vec<char>>) {
		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}

}

pub fn get_position(object: char, map: &mut Vec<Vec<char>>) -> Position {
	let mut position = Position {
		x: 0,
		y: 0,
	};

	for layer in 0..map.len() {
		for item in 0..map[layer].len() {
			if map[layer][item] == object {
				position.x = item;
				position.y = layer;
			}
		}
	}
	return position;
}

pub fn check_for_obstacles(x: usize, y: usize, textures: &LevelTextures, map: &mut Vec<Vec<char>>) -> char {
	// if map[y][x] == textures.w {return 'w';}
	// 	textures.coin => return 'c',
	// 	_ => {},
	// }
	return if map[y][x] == textures.w { 'w' } else if map[y][x] == textures.c { 'c' } else {' '};
}

// Generators

pub  fn generate_coins(mut amount: u8, textures: &LevelTextures, map: &mut Vec<Vec<char>>) {
	let mut rng = thread_rng();
	let mut coin_positions = Vec::new();
	while amount > 0 {
		let y_pos: usize = rng.gen_range(1..map.len() - 1);
		let x_pos: usize = rng.gen_range(1..map[y_pos].len() - 1);
		Vec::push(&mut coin_positions, Position{x: x_pos, y: y_pos});
		amount -= 1;
	}

	for mut position in coin_positions.into_iter() {
		/*if rng.gen_range(0..5) == 3 {  bombs implementation, but I will do it tomorrow
			todo!();
		}*/
		while map[position.y][position.x] != textures.a {  // coins can spawn only on empty tile
			position.y = rng.gen_range(1..map.len() - 1);
			position.x = rng.gen_range(1..map.len() - 1);
		}
		map[position.y][position.x] = textures.c;
	}
}