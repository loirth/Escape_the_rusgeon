// use std::env::current_exe;
use std::{io::{self, Write}, process, thread, time};
use console::Term;
use rand::{thread_rng, Rng};

#[allow(unused_imports)]
use core::mem::drop;

mod sprites;
pub use crate::sprites::{functions, player::Player};

mod structs;
pub use crate::structs::*;

fn main() {
	let player_anim_frames = ['▲', '<', '▼', '>'];

	let textures = LevelTextures {
		wall: '■',
		air: ' ',
		coin: '○', // can also be - ◌●○◙
		bomb: '◌',
		player: player_anim_frames[3],
	};
	let w = textures.wall;
	let a = textures.air;
	let c = textures.coin;
	//let b = textures.bomb;
	let mut p = textures.player;

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

	let mut player = Player{
		anim_frames: player_anim_frames,
		texture: p,
		position: functions::get_position(&map, p),
		direction: 'w',
		coin_count: 0,
	};
	let mut previous_player_position = Position {x: player.position.x, y: player.position.y};
	generate_coins(3, textures.coin, textures.air, &mut map);
	
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
		println!("Coins: {}", player.coin_count);
		// Map printing
		for layer in 0..map.len() {
			println!("{}", map[layer].iter().collect::<String>());
		}
		
		if let Ok(character) = stdout.read_char() {
			match character {
				'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => {
					player.direction = character;
					player.walk();

					match functions::check_for_obstacles(player.position.x, player.position.y, textures.clone(), &map) {
						'w' => { 
							player.position.x = previous_player_position.x;
							player.position.y = previous_player_position.y;
						},
						'c' => {
							player.coin_count += 1;
							generate_coins(1, c, a, &mut map);
						}
						_ => {},
					}
					map[previous_player_position.y][previous_player_position.x] = a; //fill this tile with air, needed to avoid ïf statement
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

/*fn generate_exits(map: Vec<Vec<char>>) {

}*/

fn generate_coins(mut amount: u8, coin_texture: char, air_texture: char, map: &mut Vec<Vec<char>>) {
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
		while map[position.y][position.x] != air_texture {  // coins can spawn only on empty tile
			position.x = rng.gen_range(1..map.len() - 1);
			position.y = rng.gen_range(1..map.len() - 1);
		}
		map[position.y][position.x] = coin_texture;
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
