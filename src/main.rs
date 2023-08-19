// use std::env::current_exe;
use std::{io::{self, Write}, process, thread, time};
use console::Term;

#[allow(unused_imports)]
use core::mem::drop;

#[derive(Debug)]
struct Position {
	x: usize,
	y: usize,
}

fn main() {
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

	let stdout = Term::buffered_stdout();

	const FPS: u64 = 1000 / 60;

	loop {
		clear_screen();
		move_cursor_up(1000);

		map[current_player_position.y][current_player_position.x] = '@';
		if previous_player_position.x != current_player_position.x || previous_player_position.y != current_player_position.y {
			map[previous_player_position.y][previous_player_position.x] = '+';
		}

		println!("WASD - movement; p - exit\n");

		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}
		
		let mut player_position = Position {
				x: current_player_position.x,
				y: current_player_position.y
		};

		if let Ok(character) = stdout.read_char() {
            match character {
                'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => player_position = move_object(player_position, character),
                'p' | 'P' => close_program(),
                _ => println!("{}", character),
            }
        }

		previous_player_position.x = current_player_position.x;
		previous_player_position.y = current_player_position.y;
		current_player_position = player_position;

		thread::sleep(time::Duration::from_millis(FPS));
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


fn move_object(mut current_pos: Position, direction: char) -> Position {
	match direction { // I use a function to add animations in future
		'w' | 'W' => current_pos.y -= 1,
		's' | 'S' => current_pos.y += 1,
		'd' | 'D' => current_pos.x += 1,
		'a' | 'A' => current_pos.x -= 1,
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