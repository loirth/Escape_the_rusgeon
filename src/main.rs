// use std::env::current_exe;
use std::{io::{self, Write}, process, thread, time};
use console::Term;

#[allow(unused_imports)]
use core::mem::drop;

mod sprites;
pub use crate::sprites::{functions, player::Player};

mod structs;
pub use crate::structs::*;

fn main() {
	let player_anim_frames = ['▲', '<', '▼', '>'];

	let w = '■'; // wall texture
	let a = ' '; // air texture
	let mut p = player_anim_frames[1]; // player texture, yes we have structure "player", but in map it's better to just write "p" instead of "player.texture"

	#[allow(unused_mut)]
	let mut map: Vec<Vec<char>> = vec![
	vec![w, w, w, w, w, w, w, w, w, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, p, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, a, a, a, a, a, a, a, a, w],
	vec![w, w, w, w, w, w, w, w, w, w],
	];

	let mut player = Player{anim_frames: player_anim_frames, texture: p, position: functions::get_position(map.clone(), p), direction: 'w'};
	let mut previous_player_position = Position {x: player.position.x, y: player.position.y};
	
	let stdout = Term::buffered_stdout();

	const FPS: u64 = 1000 / 60;

	loop {
		clear_screen();
		move_cursor_up(1000);

		// Map changing
		map[player.position.y][player.position.x] = p;
		/*if previous_player_position.x != player.position.x || previous_player_position.y != player.position.y {
			map[previous_player_position.y][previous_player_position.x] = a;
		}*/

		// UI
		println!("WASD - movement; p - exit\n");
		// Map printing
		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}
		
		if let Ok(character) = stdout.read_char() {
			match character {
				'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => {
					player.direction = character;
					player.walk();

					if !functions::check_for_obstacles(player.position.x, player.position.y, w, map.clone()) {
						player.position.x = previous_player_position.x;
						player.position.y = previous_player_position.y;
					}

					map[previous_player_position.y][previous_player_position.x] = a;//fill this tile with air, needed to avoid ïf statement
				}
				'p' | 'P' => close_program(),
				_ => println!("{}", character),
			}
		}

		previous_player_position.x = player.position.x;
		previous_player_position.y = player.position.y;

		p = player.texture;

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








