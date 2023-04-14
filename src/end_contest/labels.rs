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

use fltk::{prelude::*, frame};
use crate::constants::CONTEST_WINNER;

pub fn end_contest_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut end_contest_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 7./24.) as i32, (screen_width * 1./24.) as i32)
		.with_label(CONTEST_WINNER);
	end_contest_display_lbl.hide();
	return end_contest_display_lbl;
}