/**
 * Scoreboard program for taekwondo competition
 * Copyright (C) 2022-2024 Iker Ruiz de Infante Gonzalez <iker@irzinfante.dev>
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

use fltk::{prelude::*, frame, enums::Align};
use crate::constants::COPYRIGHT;

pub fn copyright(_screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut copyright = frame::Frame::default()
		.with_pos(0, screen_height as i32)
		.with_align(Align::RightBottom)
		.with_label(&format!("v{} - {}", env!("CARGO_PKG_VERSION"), COPYRIGHT));
	copyright.set_label_size(10);
	return copyright;
}

pub fn contest_number(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_number = frame::Frame::default()
		.with_pos(screen_width as i32, screen_height as i32)
		.with_align(Align::LeftBottom);
	contest_number.set_label_size(15);
	return contest_number;
}