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

use fltk::{prelude::*, input};
use crate::{
	Scoreboard,
	constants::{DEFAULT_CONTEST_NUMBER, DEFAULT_ROUND_TIME, DEFAULT_REST_TIME}
};

pub mod labels;
pub mod inputs;
pub mod buttons;

fn valid_input(input: &input::IntInput) -> bool {
	match input.value().parse::<u8>() {
		Ok(value) => {
			if value > 0u8 {
				return true;
			}
		},
		Err(_) => ()
	}
	return false;
}

impl Scoreboard {
	pub fn show_settings(&mut self) {
		self.settings.contest_number_lbl.show();
		self.settings.contest_number_input.show();
		
		self.settings.round_time_lbl.show();
		self.settings.round_time_input.show();
		self.settings.round_time_seconds_lbl.show();
		
		self.settings.rest_time_lbl.show();
		self.settings.rest_time_input.show();
		self.settings.rest_time_seconds_lbl.show();
		
		self.settings.new_contest_btn.show();
		
		self.settings.vertical_separator_lbl.show();
	}
	
	pub fn hide_settings(&mut self) {
		self.settings.contest_number_lbl.hide();
		self.settings.contest_number_input.hide();
		
		self.settings.round_time_lbl.hide();
		self.settings.round_time_input.hide();
		self.settings.round_time_seconds_lbl.hide();
		
		self.settings.rest_time_lbl.hide();
		self.settings.rest_time_input.hide();
		self.settings.rest_time_seconds_lbl.hide();
		
		self.settings.new_contest_btn.hide();
		
		self.settings.vertical_separator_lbl.hide();
	}
	
	pub fn set_default_settings(&mut self) {
		self.settings.contest_number_input.set_value(DEFAULT_CONTEST_NUMBER);
		self.settings.round_time_input.set_value(DEFAULT_ROUND_TIME);
		self.settings.rest_time_input.set_value(DEFAULT_REST_TIME);
	}
	
	pub fn increment_contest_number_settings(&mut self) {
		match self.settings.contest_number_input.value().parse::<u8>() {
			Ok(value) => {
				if value < u8::MAX {
					self.settings.contest_number_input.set_value(&(value  + 1).to_string());
					return;
				}
			},
			Err(_) => ()
		}
		self.settings.contest_number_input.set_value(DEFAULT_CONTEST_NUMBER);
	}
	
	pub fn validate_settings(&mut self) {
		if !(
			valid_input(&self.settings.round_time_input) &&
			valid_input(&self.settings.rest_time_input) &&
			valid_input(&self.settings.contest_number_input)
		) {
			self.settings.new_contest_btn.deactivate();
		} else {
			self.settings.new_contest_btn.activate();
		}
	}
}