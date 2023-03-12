/**
 * Scoreboard program for taekwondo competition
 * Copyright (C) 2022-2023 Iker Ruiz de Infante Gonzalez <contact@irzinfante.eu>
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

use fltk::{prelude::*, input};

pub fn contest_number_input(screen_width: f64, screen_height: f64) -> input::IntInput {
	let mut contest_number_input = input::IntInput::default()
		.with_pos((screen_width * 17./80.) as i32, (screen_height * 17./48.) as i32)
		.with_size((screen_width * 2./81.) as i32, (screen_height * 1./32.) as i32);
	contest_number_input.hide();
	return contest_number_input;
}

pub fn round_time_input(screen_width: f64, screen_height: f64) -> input::IntInput {
	let mut round_time_input = input::IntInput::default()
		.with_pos((screen_width * 17./80.) as i32, (screen_height * 20./48.) as i32)
		.with_size((screen_width * 2./81.) as i32, (screen_height * 1./32.) as i32);
	round_time_input.hide();
	return round_time_input;
}

pub fn rest_time_input(screen_width: f64, screen_height: f64) -> input::IntInput {
	let mut rest_time_input = input::IntInput::default()
		.with_pos((screen_width * 17./80.) as i32, (screen_height * 23./48.) as i32)
		.with_size((screen_width * 2./81.) as i32, (screen_height * 1./32.) as i32);
	rest_time_input.hide();
	return rest_time_input;
}