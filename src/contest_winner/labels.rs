/**
 * Scoreboard program for taekwondo competition
 * Copyright (C) 2022-2023 Iker Ruiz de Infante Gonzalez <iker@irzinfante.eu>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use fltk::{prelude::*, frame, enums::FrameType, enums::Color};

pub fn contest_winner_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_winner_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 9./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 15./24.) as i32, (screen_width * 3./24.) as i32);
	contest_winner_display_lbl.set_frame(FrameType::FlatBox);
	contest_winner_display_lbl.set_label_color(Color::White);
	contest_winner_display_lbl.hide();
	return contest_winner_display_lbl;
}

pub fn contest_winner_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_winner_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 4./16.) as i32);
	contest_winner_screen_lbl.set_frame(FrameType::FlatBox);
	contest_winner_screen_lbl.set_label_color(Color::White);
	contest_winner_screen_lbl.hide();
	return contest_winner_screen_lbl;
}