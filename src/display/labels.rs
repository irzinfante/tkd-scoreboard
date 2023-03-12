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

pub fn cheong_score_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_score_display_lbl = frame::Frame::default()
    	.with_pos(0, 0)
		.with_size((screen_width * 1./3.) as i32, (screen_height * 1./2.) as i32);
	cheong_score_display_lbl.set_frame(FrameType::FlatBox);
	cheong_score_display_lbl.set_color(Color::Blue);
	cheong_score_display_lbl.set_label_color(Color::White);
	cheong_score_display_lbl.hide();
	return cheong_score_display_lbl;
}

pub fn cheong_gam_jeon_count_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_gam_jeon_count_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 2./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 2./16.) as i32);
	cheong_gam_jeon_count_display_lbl.set_frame(FrameType::FlatBox);
	cheong_gam_jeon_count_display_lbl.set_color(Color::Blue.darker());
	cheong_gam_jeon_count_display_lbl.set_label_color(Color::White);
	cheong_gam_jeon_count_display_lbl.hide();
	return cheong_gam_jeon_count_display_lbl;
}

pub fn cheong_seung_display_lbl() -> frame::Frame {
	let mut cheong_round_seung_display_lbl = frame::Frame::default();
	cheong_round_seung_display_lbl.set_frame(FrameType::OFlatFrame);
	cheong_round_seung_display_lbl.set_color(Color::Blue);
	cheong_round_seung_display_lbl.hide();
	return cheong_round_seung_display_lbl;
}

pub fn hong_score_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_score_display_lbl = frame::Frame::default()
    	.with_pos((screen_width * 2./3.) as i32, 0)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./2.) as i32);
	hong_score_display_lbl.set_frame(FrameType::FlatBox);
	hong_score_display_lbl.set_color(Color::Red);
	hong_score_display_lbl.set_label_color(Color::White);
	hong_score_display_lbl.hide();
	return hong_score_display_lbl;
}

pub fn hong_gam_jeon_count_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_gam_jeon_count_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 5./9.) as i32, (screen_height * 2./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 2./16.) as i32);
	hong_gam_jeon_count_display_lbl.set_frame(FrameType::FlatBox);
	hong_gam_jeon_count_display_lbl.set_color(Color::Red.darker());
	hong_gam_jeon_count_display_lbl.set_label_color(Color::White);
	hong_gam_jeon_count_display_lbl.hide();
	return hong_gam_jeon_count_display_lbl;
}

pub fn hong_seung_display_lbl() -> frame::Frame {
	let mut hong_round_seung_display_lbl = frame::Frame::default();
	hong_round_seung_display_lbl.set_frame(FrameType::OFlatFrame);
	hong_round_seung_display_lbl.set_color(Color::Red);
	hong_round_seung_display_lbl.hide();
	return hong_round_seung_display_lbl;
}

pub fn round_rest_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, 0)
		.with_size((screen_width *  1./9.) as i32, (screen_height * 1./16.) as i32);
	round_display_lbl.set_frame(FrameType::FlatBox);
	round_display_lbl.set_color(Color::Black);
	round_display_lbl.set_label_color(Color::White);
	round_display_lbl.hide();
	return round_display_lbl;
}

pub fn round_number_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_number_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 1./16.) as i32)
		.with_size((screen_width *  1./9.) as i32, (screen_height * 3./16.) as i32);
	round_number_display_lbl.set_frame(FrameType::FlatBox);
	round_number_display_lbl.set_color(Color::Black);
	round_number_display_lbl.set_label_color(Color::White);
	round_number_display_lbl.hide();
	return round_number_display_lbl;
}

pub fn time_display_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut time_display_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 1./4.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./4.) as i32);
	time_display_lbl.set_frame(FrameType::FlatBox);
	time_display_lbl.hide();
	return time_display_lbl;
}