use rand::{thread_rng, Rng};
use crate::{Position, LevelTextures};

pub fn print_map(map: &Vec<Vec<char>>) {
		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}
}

pub fn get_position(object: char, map: &Vec<Vec<char>>) -> Position {
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

pub fn check_for_obstacles(x: usize, y: usize, textures: &LevelTextures, map: &Vec<Vec<char>>) -> char {
	return if map[y][x] == textures.wall { 'w' } else if map[y][x] == textures.coin { 'c' } else {' '};
}

// Generators

pub fn generate_coins(mut amount: u8, coin_texture: char, air_texture: char, map: &mut Vec<Vec<char>>) {
	let mut rng = thread_rng();
	let mut coin_positions = Vec::new();
	while amount > 0 {
		let y_pos: usize = rng.gen_range(1..map.len() - 1);
		let x_pos: usize = rng.gen_range(1..map[y_pos].len() - 1);
		Vec::push(&mut coin_positions, Position{x: x_pos, y: y_pos});
		amount -= 1;
	}

	for mut position in coin_positions.into_iter() {
		let mut max_recursion = 100;
		while map[position.y][position.x] != air_texture && max_recursion > 0 {  // coins can spawn only on empty tile
			position.y = rng.gen_range(1..map.len() - 1);
			position.x = rng.gen_range(1..map[position.y].len() - 1);
			max_recursion -= 1;
		}
		map[position.y][position.x] = coin_texture;
	}
}

pub fn generate_room(mut width: usize, mut height: usize, textures: &LevelTextures) -> Vec<Vec<char>> {
	let min_size = 4;
	if height < min_size { height = min_size; };
	if width < min_size { width = min_size; };
	let mut map: Vec<Vec<char>> = Vec::new();
	Vec::push(&mut map, create_vector(width, textures, true));
	height -= 1;
	while height > 1 {
		Vec::push(&mut map, create_vector(width, textures, false));
		height -= 1;
	}
	Vec::push(&mut map, create_vector(width, textures, true));
	map
}

fn create_vector(mut lenght: usize, textures: &LevelTextures, only_walls: bool) -> Vec<char> {
	let mut vec = Vec::new();
	Vec::push(&mut vec, textures.wall);
	lenght -= 1;
	let air_texture = if only_walls {textures.wall} else {textures.air};
	while lenght > 1 {
		Vec::push(&mut vec, air_texture);
		lenght -= 1;
	}
	Vec::push(&mut vec, textures.wall);
	vec
}