// use std::env::current_exe;
use std::{io::{self, Write}, process, thread, time};
use console::Term;

#[allow(unused_imports)]
use core::mem::drop;

mod characters;
pub use crate::characters::sprite::Player;

mod level;
pub use crate::level::structs::*;
pub use crate::level::map::*;

fn main() {
	// main loop
	loop {
		let player_anim_frames = ['▲', '<', '▼', '>'];

		let t: LevelTextures = LevelTextures{
			w: '■',
			a: ' ',
			c: '○', // can also be - ◌●○◙
			b: '◌',
			p: player_anim_frames[3],
		};

		let mut map: Vec<Vec<char>> = vec![
		vec![t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.p, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.a, t.w],
		vec![t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w, t.w],
		];

		let mut player = Player{
			position: get_position(t.p, &mut map),
			direction: 'r',
			rotation_frames: player_anim_frames,
			texture: t.p,
			collision_mask: vec!(['w', t.w], ['c', t.c], ['b', t.b]),
			coin_count: 0,
			coins_needed_for_win: 50,
			is_win_the_game: false,
		};

		generate_coins(3, t.c, t.a, &mut map);
		
		let stdout = Term::buffered_stdout();

		const FPS: u64 = 1000 / 60;
		// game loop
		loop {
			clear_screen();
			move_cursor_up(1000);

			// UI
			println!("WASD - movement\np - exit\n");
			if player.is_win_the_game {println!("YOU WIN THE GAME!\nPress r to restart.\n")}
			println!("Coins: {}", player.coin_count);
			print_map(&map);

			if let Ok(character) = stdout.read_char() {
				match character {
					'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => {
						player.direction = character;
						player.walk(t.a, &mut map);
					},
					'r' | 'R' => if player.is_win_the_game {break;},
					'p' | 'P' => close_program(),
					_ => println!("{}", character),
				}
			}

			thread::sleep(time::Duration::from_millis(FPS));
		}
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