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

use fltk::prelude::*;
use crate::Scoreboard;

pub mod labels;
pub mod buttons;

impl Scoreboard {
	pub fn show_superiority_decision(&mut self) {
		self.display.superiority_decision_lbl.show();
		
		self.controls.cheong_superiority_decision_btn.show();
		self.controls.cheong_superiority_decision_btn.activate();
		
		self.controls.hong_superiority_decision_btn.show();
		self.controls.hong_superiority_decision_btn.activate();
	}
	
	pub fn hide_superiority_decision(&mut self) {
		self.display.superiority_decision_lbl.hide();
		
		self.controls.cheong_superiority_decision_btn.hide();
		
		self.controls.hong_superiority_decision_btn.hide();
	}
}