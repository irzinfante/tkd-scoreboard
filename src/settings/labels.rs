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

use fltk::{prelude::*, frame, enums::Align, enums::FrameType};
use crate::constants::{CONTEST_NUMBER, REST_TIME, ROUND_TIME, SECONDS};

pub fn contest_number_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_number_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./2.) as i32, (screen_height * 17./48.) as i32)
		.with_size(0, (screen_height * 1./32.) as i32)
		.with_align(Align::Left)
		.with_label(CONTEST_NUMBER);
	contest_number_lbl.set_frame(FrameType::FlatBox);
	contest_number_lbl.set_color(fltk::enums::Color::Red);
	contest_number_lbl.hide();
	return contest_number_lbl;
}

pub fn round_time_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_time_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./2.) as i32, (screen_height * 20./48.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./32.) as i32)
		.with_align(Align::Left)
		.with_label(ROUND_TIME);
	round_time_lbl.hide();
	return round_time_lbl;
}

pub fn round_time_seconds_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_time_seconds_lbl = frame::Frame::default()
		.with_pos((screen_width * 41./81.) as i32, (screen_height * 20./48.) as i32)
		.with_size((screen_width * 2./81.) as i32, (screen_height * 1./32.) as i32)
		.with_align(Align::Right)
		.with_label(SECONDS);
	round_time_seconds_lbl.hide();
	return round_time_seconds_lbl;
}

pub fn rest_time_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut rest_time_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./2.) as i32, (screen_height * 23./48.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./32.) as i32)
		.with_align(Align::Left)
		.with_label(REST_TIME);
	rest_time_lbl.hide();
	return rest_time_lbl;
}

pub fn rest_time_seconds_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut rest_time_seconds_lbl = frame::Frame::default()
		.with_pos((screen_width * 41./81.) as i32, (screen_height * 23./48.) as i32)
		.with_size((screen_width * 2./81.) as i32, (screen_height * 1./32.) as i32)
		.with_align(Align::Right)
		.with_label(SECONDS);
	rest_time_seconds_lbl.hide();
	return rest_time_seconds_lbl;
}