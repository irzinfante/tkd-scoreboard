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
use crate::constants::{DELETE_DATA, EXPORT_CSV, NEW_CONTEST};

pub fn new_contest_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut new_contest_btn = button::Button::default()
		.with_pos((screen_width * 13./90.) as i32, (screen_height * 28./48.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(NEW_CONTEST);
	new_contest_btn.deactivate();
	new_contest_btn.hide();
	return new_contest_btn;
}

pub fn export_data_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut export_data_btn = button::Button::default()
		.with_pos((screen_width * 35./40.) as i32, (screen_height * 2./14.) as i32)
		.with_size((screen_width * 4./40.) as i32, (screen_height * 4./63.) as i32)
		.with_label(EXPORT_CSV);
	export_data_btn.deactivate();
	export_data_btn.hide();
	return export_data_btn;
}

pub fn delete_data_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut delete_data_btn = button::Button::default()
		.with_pos((screen_width * 35./40.) as i32, (screen_height * 3./14.) as i32)
		.with_size((screen_width * 4./40.) as i32, (screen_height * 4./63.) as i32)
		.with_label(DELETE_DATA);
	delete_data_btn.deactivate();
	delete_data_btn.hide();
	return delete_data_btn;
}