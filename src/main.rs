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
	let room_sizes = vec!([11,11], [4,6], [3,21], [5,5], [4,4], [20, 20], [7,13], [21, 21]);
	let mut i_for_room = 0;
	// main loop
	loop {
		let player_anim_frames = ['▲', '<', '▼', '>'];

		let textures: LevelTextures = LevelTextures{
			wall: '■',
			air: ' ',
			coin: '○', // can also be - ◌●○◙
			bomb: '◌',
			player: player_anim_frames[3],
		};


		if i_for_room >= room_sizes.len() { break; }
		let mut room: Vec<Vec<char>> = generate_room(room_sizes[i_for_room][0], room_sizes[i_for_room][1], &textures);
		//let player_position: Position = Position{x: room_sizes[i_for_room][0] / 2 - 1, y: room_sizes[i_for_room][1] / 2 - 1};
		//room[player_position.y][player_position.x] = textures.p;
		room[1][1] = textures.player;
		i_for_room += 1;

		let mut player = Player{
			position: get_position(textures.player, &room),
			direction: 'r',
			rotation_frames: player_anim_frames,
			texture: textures.player,
			collision_mask: vec!(['w', textures.wall], ['c', textures.coin], ['b', textures.bomb]),
			coin_count: 0,
			coins_needed_for_win: 20,
			is_win_the_game: false,
		};

		generate_coins(3, textures.coin, textures.air, &mut room);
		
		let stdout = Term::buffered_stdout();

		const FPS: u64 = 1000 / 60;
		// game loop
		loop {
			clear_screen();
			move_cursor_up(1000);

			// UI
			println!("WASD - movement\np - exit\n");
			if player.is_win_the_game {println!("YOU WIN THIS LEVEL!\nPress r to go to the next level.\n")}
			println!("Coins: {}", player.coin_count);
			print_map(&room);

			if let Ok(character) = stdout.read_char() {
				match character {
					'w' | 'a' | 's' | 'd' | 'W' | 'A' | 'S' | 'D' => {
						player.direction = character;
						player.walk(textures.air, &mut room);
					},
					'r' | 'R' => if player.is_win_the_game {break;},
					'p' | 'P' => close_program(),
					_ => println!("{}", character),
				}
			}

			thread::sleep(time::Duration::from_millis(FPS));
		}
	}
	clear_screen();
	move_cursor_up(1000);
	println!("Congratulations! You complete the game!")
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