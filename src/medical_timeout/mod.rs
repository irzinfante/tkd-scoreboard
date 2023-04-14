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

use fltk::prelude::*;
use crate::Scoreboard;

pub mod labels;
pub mod buttons;

impl Scoreboard {
	pub fn show_medical_timeout(&mut self) {
		self.kye_shi_time = 60f32;
		
		self.display.kye_shi_lbl.show();
		self.display.kye_shi_time_lbl.show();
		
		self.screen.kye_shi_time_lbl.show();
		
		self.controls.kye_shi_cancel_btn.show();
		self.controls.kye_shi_cancel_btn.activate();
	}
	
	pub fn update_medical_timeout(&mut self) {
		self.display.kye_shi_time_lbl.set_label(&format!("{:}", self.kye_shi_time.trunc()));
		self.screen.kye_shi_time_lbl.set_label(&format!("{:}", self.kye_shi_time.trunc()));
	}
	
	pub fn hide_medical_timeout(&mut self) {
		self.kye_shi_time = 0f32;
		
		self.display.kye_shi_lbl.hide();
		self.display.kye_shi_time_lbl.hide();
		
		self.screen.kye_shi_time_lbl.hide();
		
		self.controls.kye_shi_cancel_btn.hide();
	}
}