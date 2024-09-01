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

use fltk::{prelude::*, enums::Color};
use crate::{Scoreboard, enums::State, enums::Winner};

pub mod labels;

pub fn seung_screen_lbl_dimensions(screen_width: f64, screen_height: f64) -> (f64, f64, i32) {
	let w = screen_width * 2./18.;
	let h = screen_height * 3./16.;
	let d = w.min(h);
	return ((w - d) / 2., ((h - d) / 2.), d as i32);
}

impl Scoreboard {
	pub fn show_screen(&mut self) {
		self.screen.hong_score_lbl.show();
		self.screen.hong_gam_jeon_count_lbl.show();
		
		self.screen.cheong_score_lbl.show();
		self.screen.cheong_gam_jeon_count_lbl.show();
	
		self.screen.round_rest_lbl.show();
		self.screen.round_number_lbl.show();
	
		self.screen.time_lbl.show();
	}
	
	pub fn update_screen(&mut self) {
		self.screen.hong_score_lbl.set_label(&self.hong_score.to_string());
		self.screen.hong_gam_jeon_count_lbl.set_label(&self.hong_gam_jeon_count.to_string());
		self.screen.cheong_score_lbl.set_label(&self.cheong_score.to_string());
		self.screen.cheong_gam_jeon_count_lbl.set_label(&self.cheong_gam_jeon_count.to_string());
		self.screen.time_lbl.set_label(&format!("{}:{:02}", (self.time/60.).trunc(), (self.time%60.).trunc()));
		
		match self.state {
			State::KeumanCondition => {
				self.screen.time_lbl.set_color(Color::White);
				self.screen.time_lbl.set_label_color(Color::Black);
			},
			State::Timeout => {
				if self.blink {
					self.screen.time_lbl.set_color(Color::White);
					self.screen.time_lbl.set_label_color(Color::Black);
				} else {
					self.screen.time_lbl.set_color(Color::Black);
					self.screen.time_lbl.set_label_color(Color::White);
				}
			},
			_ => {
				self.screen.time_lbl.set_color(Color::Black);
				self.screen.time_lbl.set_label_color(Color::White);
			}
		}
	}
	
	pub fn hide_screen(&mut self) {
		self.screen.cheong_score_lbl.hide();
		self.screen.cheong_gam_jeon_count_lbl.hide();
		self.screen.cheong_seung_lbl.hide();
		
		self.screen.hong_score_lbl.hide();
		self.screen.hong_gam_jeon_count_lbl.hide();
		self.screen.hong_seung_lbl.hide();
		
		self.screen.round_rest_lbl.hide();
		self.screen.round_number_lbl.hide();
		
		self.screen.time_lbl.hide();
	}
	
	pub fn show_round_winner_screen(&mut self) {
		match self.round_winner[(self.round_number - 1) as usize] {
			Winner::Cheong => {
				self.screen.cheong_seung_lbl.show();
			},
			Winner::Hong => {
				self.screen.hong_seung_lbl.show();
			},
			Winner::None => ()
		}
	}
	
	pub fn hide_round_winner_screen(&mut self) {
		match self.round_winner[(self.round_number - 1) as usize] {
			Winner::Cheong => {
				self.screen.cheong_seung_lbl.hide();
			},
			Winner::Hong => {
				self.screen.hong_seung_lbl.hide();
			},
			Winner::None => ()
		}
	}
}