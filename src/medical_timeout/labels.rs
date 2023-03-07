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

use fltk::{prelude::*, frame, enums::FrameType, enums::Color};
use crate::constants::KYE_SHI;

pub fn kye_shi_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut kye_shi_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 4./8.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./8.) as i32)
		.with_label(KYE_SHI);
	kye_shi_display_lbl.set_frame(FrameType::FlatBox);
	kye_shi_display_lbl.hide();
	return kye_shi_display_lbl;
}

pub fn kye_shi_time_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut kye_shi_time_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 5./8.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./8.) as i32);
	kye_shi_time_display_lbl.set_frame(FrameType::FlatBox);
	kye_shi_time_display_lbl.hide();
	return kye_shi_time_display_lbl;
}

pub fn kye_shi_time_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut kye_shi_time_screen_lbl = frame::Frame::default()
		.with_pos(0, (screen_height * 1./4.) as i32)
		.with_size(screen_width as i32, (screen_height * 2./4.) as i32);
	kye_shi_time_screen_lbl.set_frame(FrameType::FlatBox);
	kye_shi_time_screen_lbl.set_color(Color::White);
	kye_shi_time_screen_lbl.set_label_color(Color::Black);
	kye_shi_time_screen_lbl.hide();
	return kye_shi_time_screen_lbl;
}