use crate::structs::*;
/*pub fn move_object(mut current_pos: Position, direction: char) -> Position {
	match direction { // I use a function to add animations in future
		'w' | 'W' => current_pos.y -= 1,
		's' | 'S' => current_pos.y += 1,
		'd' | 'D' => current_pos.x += 1,
		'a' | 'A' => current_pos.x -= 1,
		_ => return current_pos,
	};
	return current_pos;
}*/

pub fn get_position(map: &Vec<Vec<char>>, object: char) -> Position {
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

pub fn check_for_obstacles(x: usize, y: usize, textures: LevelTextures, map: &Vec<Vec<char>>) -> char {
	//println!("{}, {}", textures.wall, textures.coin);
	return if map[y][x] == textures.wall { 'w' } else if map[y][x] == textures.coin { 'c' } else {' '};
}
