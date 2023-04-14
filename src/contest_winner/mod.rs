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

use fltk::{prelude::*, enums::Color};
use crate::{
	Scoreboard,
	enums::Winner,
	constants::{CHEONG_NL_SEUNG, CHEONG_SEUNG, HONG_NL_SEUNG, HONG_SEUNG}
};

pub mod labels;
pub mod buttons;

impl Scoreboard {
	pub fn show_contest_winner(&mut self, winner: Winner) {
		match winner {
			Winner::Cheong => {
				self.display.contest_winner_lbl.set_label(CHEONG_SEUNG);
				self.display.contest_winner_lbl.set_color(Color::Blue);
				self.screen.contest_winner_lbl.set_label(CHEONG_NL_SEUNG);
				self.screen.contest_winner_lbl.set_color(Color::Blue);
			},
			Winner::Hong => {
				self.display.contest_winner_lbl.set_label(HONG_SEUNG);
				self.display.contest_winner_lbl.set_color(Color::Red);
				self.screen.contest_winner_lbl.set_label(HONG_NL_SEUNG);
				self.screen.contest_winner_lbl.set_color(Color::Red);
			},
			Winner::None => ()
		}
		self.display.contest_winner_lbl.show();
		self.screen.contest_winner_lbl.show();
		
		self.controls.clear_scoreboard_btn.show();
		self.controls.clear_scoreboard_btn.activate();
	}
	
	pub fn hide_contest_winner(&mut self) {
		self.display.contest_winner_lbl.hide();
		self.screen.contest_winner_lbl.hide();
		
		self.controls.clear_scoreboard_btn.hide();
	}
}