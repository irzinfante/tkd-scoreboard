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

use fltk::{prelude::*, button, enums::Color};
use crate::constants::{CHEONG, HONG};

pub fn cheong_superiority_decision_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_superiority_decision_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(CHEONG);
	cheong_superiority_decision_btn.set_color(Color::Blue);
	cheong_superiority_decision_btn.set_label_color(Color::White);
	cheong_superiority_decision_btn.deactivate();
	cheong_superiority_decision_btn.hide();
	return cheong_superiority_decision_btn;
}

pub fn hong_superiority_decision_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_superiority_decision_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(HONG);
	hong_superiority_decision_btn.set_color(Color::Red);
	hong_superiority_decision_btn.set_label_color(Color::White);
	hong_superiority_decision_btn.deactivate();
	hong_superiority_decision_btn.hide();
	return hong_superiority_decision_btn;
}