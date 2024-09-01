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

use fltk::{prelude::*, button};
use crate::constants::CLEAR_SCOREBOARD;

pub fn clear_scoreboard_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut clear_scoreboard_btn = button::Button::default()
		.with_pos((screen_width * 23./54.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 4./27.) as i32, (screen_height * 1./16.) as i32)
		.with_label(CLEAR_SCOREBOARD);
	clear_scoreboard_btn.deactivate();
	clear_scoreboard_btn.hide();
	return clear_scoreboard_btn;
}