// use std::env::current_exe;
use std::{io::{self, Write}, process, thread, time};
use console::Term;

#[allow(unused_imports)]
use core::mem::drop;

mod sprites;
pub use crate::sprites::player::Player;

mod level;
pub use crate::level::structs::*;
pub use crate::level::map::*;

fn main() {
	loop {
		let mut is_win_the_game = false;
		let coins_needed_for_win = 50;
		let player_anim_frames = ['▲', '<', '▼', '>'];

		let mut t: LevelTextures = LevelTextures{
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
			anim_frames: player_anim_frames,
			texture: t.p,
			position: get_position(t.p, &mut map),
			direction: 'w',
			coin_count: 0,
		};
		let mut previous_player_position = Position {x: player.position.x, y: player.position.y};
		generate_coins(3, &t, &mut map);
		
		let stdout = Term::buffered_stdout();

		const FPS: u64 = 1000 / 60;

		loop {
			clear_screen();
			move_cursor_up(1000);

			// MAP changing
			map[player.position.y][player.position.x] = t.p;

			// UI
			println!("WASD - movement\np - exit\n");
			if is_win_the_game {println!("YOU WIN THE GAME!\nPress r to restart.\n")}
			println!("Coins: {}", player.coin_count);
			print_map(&map);

			if let Ok(character) = stdout.read_char() {
				match character {
					'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => {
						player.direction = character;
						player.walk();

						match check_for_obstacles(player.position.x, player.position.y, &t, &mut map) {
							'w' => { 
								player.position.x = previous_player_position.x;
								player.position.y = previous_player_position.y;
							},
							'c' => {
								player.coin_count += 1;
								if player.coin_count >= coins_needed_for_win {is_win_the_game = true;}
								generate_coins(1, &t, &mut map);
							},
							_ => {},
						}
						map[previous_player_position.y][previous_player_position.x] = t.a; //fill this tile with air, needed to avoid ïf statement
					},
					'r' | 'R' => if is_win_the_game {break;},
					'p' | 'P' => close_program(),
					_ => println!("{}", character),
				}
			}

			previous_player_position.x = player.position.x;
			previous_player_position.y = player.position.y;

			t.p = player.texture;

			thread::sleep(time::Duration::from_millis(FPS));
		}
	}
}

/*fn generate_exits(MAP: Vec<Vec<char>>) {

}*/

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
