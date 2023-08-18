// use std::env::current_exe;
use std::io::{self, Write};
use std::process;

#[allow(unused_imports)]
use core::mem::drop;

#[derive(Debug)]
struct Position {
	x: usize,
	y: usize,
}

fn main() {
	#[allow(unused_mut)]
	let mut map: Vec<Vec<char>> = vec![
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '@', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	vec!['+', '+', '+', '+', '+', '+', '+', '+', '+', '+'],
	];

	let mut current_player_position = get_position(map.clone(), '@');
	let mut previous_player_position = Position {x: current_player_position.x, y: current_player_position.y};

	loop {
		let mut user_input: String = String::new();

		clear_screen();
		move_cursor_up(1000);

		map[current_player_position.y][current_player_position.x] = '@';
		if previous_player_position.x != current_player_position.x || previous_player_position.y != current_player_position.y {
			map[previous_player_position.y][previous_player_position.x] = '+';
		}

		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}

		println!("Enter direction or |w|a|s|d| or \"exit\"");
		
		io::stdin()
			.read_line(&mut user_input)
			.expect("Failed to read line");

		let mut player_position = Position {
				x: current_player_position.x,
				y: current_player_position.y
		};

		match user_input.to_lowercase().trim() {
			"exit" => close_program(),
			"up" | "down" | "left" | "right" | "w" | "s" | "a" | "d" => player_position = move_object(player_position, user_input.to_lowercase().trim()),
			_ => {
				println!("Wrong function, please try again!");
				continue;
			},
		}

		previous_player_position.x = current_player_position.x;
		previous_player_position.y = current_player_position.y;
		current_player_position = player_position;
	}
}

fn clear_screen() {
	print!("\x1b[2J");
	io::stdout().flush().unwrap();
}

fn move_cursor_up(layer: i16) {
	print!("\x1b[{layer}A");
	io::stdout().flush().unwrap();
}

fn close_program() {
	println!("Goodbye, see you later!");
	process::exit(0);
}


fn move_object(mut current_pos: Position, direction: &str) -> Position {
	match direction {
		"up" | "w" => current_pos.y -= 1,
		"down" | "s" => current_pos.y += 1,
		"right" | "d" => current_pos.x += 1,
		"left" | "a" => current_pos.x -= 1,
		_ => return current_pos,
	};
	return current_pos;
}


fn get_position(map: Vec<Vec<char>>, object: char) -> Position {
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