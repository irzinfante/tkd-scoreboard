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
use crate::{Scoreboard, enums::State};

pub mod buttons;

impl Scoreboard {
	pub fn update_controls(&mut self) {
		match self.state {
			State::None | State::Settings | State::EndContest | State::ContestWinner => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.hide();
				self.controls.kal_yeo_btn.hide();
				self.controls.kye_shi_btn.hide();
				self.controls.keu_man_btn.hide();
				self.controls.end_contest_btn.hide();
				
				self.hide_scoring_controls();
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			},
			State::CallContestants => {
				self.controls.shi_jak_btn.show();
				self.controls.shi_jak_btn.activate();
				self.controls.kye_sok_btn.hide();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.deactivate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.deactivate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.deactivate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.activate();
				
				self.show_inactive_scoring_controls();
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			},
			State::Round => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.show();
				self.controls.kye_sok_btn.deactivate();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.activate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.deactivate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.deactivate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.deactivate();
				
				self.show_active_scoring_controls();
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			},
			State::Timeout => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.show();
				self.controls.kye_sok_btn.activate();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.deactivate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.activate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.deactivate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.activate();
				
				self.show_active_scoring_controls();
				
				self.controls.plus_one_second_btn.show();
				if self.time < self.round_time {
					self.controls.plus_one_second_btn.activate();
				} else {
					self.controls.plus_one_second_btn.deactivate();
				}
				self.controls.minus_one_second_btn.show();
				if self.time > 0f32 {
					self.controls.minus_one_second_btn.activate();
				} else {
					self.controls.minus_one_second_btn.deactivate();
				}
			},
			State::MedicalTimeout => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.show();
				self.controls.kye_sok_btn.deactivate();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.deactivate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.deactivate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.deactivate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.deactivate();
				
				self.hide_scoring_controls();
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			},
			State::KeumanCondition => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.show();
				self.controls.kye_sok_btn.deactivate();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.deactivate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.deactivate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.activate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.activate();
				
				self.show_active_scoring_controls();
				
				self.controls.plus_one_second_btn.show();
				if self.time < self.round_time {
					self.controls.plus_one_second_btn.activate();
				} else {
					self.controls.plus_one_second_btn.deactivate();
				}
				self.controls.minus_one_second_btn.show();
				if self.time > 0f32 {
					self.controls.minus_one_second_btn.activate();
				} else {
					self.controls.minus_one_second_btn.deactivate();
				}
			},
			State::SuperiorityDecision => {
				self.controls.shi_jak_btn.hide();
				self.controls.kye_sok_btn.hide();
				self.controls.kal_yeo_btn.hide();
				self.controls.kye_shi_btn.hide();
				self.controls.keu_man_btn.hide();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.activate();
				
				self.hide_scoring_controls();
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			},
			State::RestFirstPart | State::RestSecondPart => {
				self.controls.shi_jak_btn.show();
				self.controls.shi_jak_btn.deactivate();
				self.controls.kye_sok_btn.hide();
				self.controls.kal_yeo_btn.show();
				self.controls.kal_yeo_btn.deactivate();
				self.controls.kye_shi_btn.show();
				self.controls.kye_shi_btn.deactivate();
				self.controls.keu_man_btn.show();
				self.controls.keu_man_btn.deactivate();
				self.controls.end_contest_btn.show();
				self.controls.end_contest_btn.activate();
				
				self.show_inactive_scoring_controls();
				
				if self.cheong_gam_jeon_count < 5 {
					self.controls.cheong_plus_gam_jeon_btn.activate();
				} else {
					self.controls.cheong_plus_gam_jeon_btn.deactivate();
				}
				if self.cheong_gam_jeon_count > 0 {
					self.controls.cheong_minus_gam_jeon_btn.activate();
				} else {
					self.controls.cheong_minus_gam_jeon_btn.deactivate();
				}
				
				if self.hong_gam_jeon_count < 5 {
					self.controls.hong_plus_gam_jeon_btn.activate();
				} else {
					self.controls.hong_plus_gam_jeon_btn.deactivate();
				}
				if self.hong_gam_jeon_count > 0 {
					self.controls.hong_minus_gam_jeon_btn.activate();
				} else {
					self.controls.hong_minus_gam_jeon_btn.deactivate();
				}
				
				self.controls.plus_one_second_btn.hide();
				self.controls.minus_one_second_btn.hide();
			}
		}
	}
	
	fn show_scoring_controls(&mut self) {
		self.controls.cheong_plus_jirugi_btn.show();
		self.controls.cheong_minus_jirugi_btn.show();
		self.controls.cheong_plus_momtong_btn.show();
		self.controls.cheong_minus_momtong_btn.show();
		self.controls.cheong_plus_momdollyeo_momtong_btn.show();
		self.controls.cheong_minus_momdollyeo_momtong_btn.show();
		self.controls.cheong_plus_olgul_btn.show();
		self.controls.cheong_minus_olgul_btn.show();
		self.controls.cheong_plus_momdollyeo_olgul_btn.show();
		self.controls.cheong_minus_momdollyeo_olgul_btn.show();
		self.controls.cheong_plus_gam_jeon_btn.show();
		self.controls.cheong_minus_gam_jeon_btn.show();
		
		self.controls.hong_plus_gam_jeon_btn.show();
		self.controls.hong_minus_gam_jeon_btn.show();
		self.controls.hong_plus_jirugi_btn.show();
		self.controls.hong_minus_jirugi_btn.show();
		self.controls.hong_plus_momtong_btn.show();
		self.controls.hong_minus_momtong_btn.show();
		self.controls.hong_plus_momdollyeo_momtong_btn.show();
		self.controls.hong_minus_momdollyeo_momtong_btn.show();
		self.controls.hong_plus_olgul_btn.show();
		self.controls.hong_minus_olgul_btn.show();
		self.controls.hong_plus_momdollyeo_olgul_btn.show();
		self.controls.hong_minus_momdollyeo_olgul_btn.show();
	}
	
	fn show_inactive_scoring_controls(&mut self) {
		self.show_scoring_controls();
		
		self.controls.cheong_plus_jirugi_btn.deactivate();
		self.controls.cheong_minus_jirugi_btn.deactivate();
		self.controls.cheong_plus_momtong_btn.deactivate();
		self.controls.cheong_minus_momtong_btn.deactivate();
		self.controls.cheong_plus_momdollyeo_momtong_btn.deactivate();
		self.controls.cheong_minus_momdollyeo_momtong_btn.deactivate();
		self.controls.cheong_plus_olgul_btn.deactivate();
		self.controls.cheong_minus_olgul_btn.deactivate();
		self.controls.cheong_plus_momdollyeo_olgul_btn.deactivate();
		self.controls.cheong_minus_momdollyeo_olgul_btn.deactivate();
		self.controls.cheong_plus_gam_jeon_btn.deactivate();
		self.controls.cheong_minus_gam_jeon_btn.deactivate();
		
		self.controls.hong_plus_gam_jeon_btn.deactivate();
		self.controls.hong_minus_gam_jeon_btn.deactivate();
		self.controls.hong_plus_jirugi_btn.deactivate();
		self.controls.hong_minus_jirugi_btn.deactivate();
		self.controls.hong_plus_momtong_btn.deactivate();
		self.controls.hong_minus_momtong_btn.deactivate();
		self.controls.hong_plus_momdollyeo_momtong_btn.deactivate();
		self.controls.hong_minus_momdollyeo_momtong_btn.deactivate();
		self.controls.hong_plus_olgul_btn.deactivate();
		self.controls.hong_minus_olgul_btn.deactivate();
		self.controls.hong_plus_momdollyeo_olgul_btn.deactivate();
		self.controls.hong_minus_momdollyeo_olgul_btn.deactivate();
	}
	
	fn show_active_scoring_controls(&mut self) {
		self.show_scoring_controls();
		
		self.controls.cheong_plus_jirugi_btn.activate();
		if self.cheong_jirugi_count > 0 {
			self.controls.cheong_minus_jirugi_btn.activate();
		} else {
			self.controls.cheong_minus_jirugi_btn.deactivate();
		}
		
		self.controls.cheong_plus_momtong_btn.activate();
		if self.cheong_momtong_count > 0 {
			self.controls.cheong_minus_momtong_btn.activate();
		} else {
			self.controls.cheong_minus_momtong_btn.deactivate();
		}
		
		self.controls.cheong_plus_momdollyeo_momtong_btn.activate();
		if self.cheong_momdollyeo_momtong_count > 0 {
			self.controls.cheong_minus_momdollyeo_momtong_btn.activate();
		} else {
			self.controls.cheong_minus_momdollyeo_momtong_btn.deactivate();
		}
		
		self.controls.cheong_plus_olgul_btn.activate();
		if self.cheong_olgul_count > 0 {
			self.controls.cheong_minus_olgul_btn.activate();
		} else {
			self.controls.cheong_minus_olgul_btn.deactivate();
		}
		
		self.controls.cheong_plus_momdollyeo_olgul_btn.activate();
		if self.cheong_momdollyeo_olgul_count > 0 {
			self.controls.cheong_minus_momdollyeo_olgul_btn.activate();
		} else {
			self.controls.cheong_minus_momdollyeo_olgul_btn.deactivate();
		}
		
		if self.cheong_gam_jeon_count < 5 {
			self.controls.cheong_plus_gam_jeon_btn.activate();
		} else {
			self.controls.cheong_plus_gam_jeon_btn.deactivate();
		}
		if self.cheong_gam_jeon_count > 0 {
			self.controls.cheong_minus_gam_jeon_btn.activate();
		} else {
			self.controls.cheong_minus_gam_jeon_btn.deactivate();
		}
		
		if self.hong_gam_jeon_count < 5 {
			self.controls.hong_plus_gam_jeon_btn.activate();
		} else {
			self.controls.hong_plus_gam_jeon_btn.deactivate();
		}
		if self.hong_gam_jeon_count > 0 {
			self.controls.hong_minus_gam_jeon_btn.activate();
		} else {
			self.controls.hong_minus_gam_jeon_btn.deactivate();
		}
		
		self.controls.hong_plus_jirugi_btn.activate();
		if self.hong_jirugi_count > 0 {
			self.controls.hong_minus_jirugi_btn.activate();
		} else {
			self.controls.hong_minus_jirugi_btn.deactivate();
		}
		
		self.controls.hong_plus_momtong_btn.activate();
		if self.hong_momtong_count > 0 {
			self.controls.hong_minus_momtong_btn.activate();
		} else {
			self.controls.hong_minus_momtong_btn.deactivate();
		}
		
		self.controls.hong_plus_momdollyeo_momtong_btn.activate();
		if self.hong_momdollyeo_momtong_count > 0 {
			self.controls.hong_minus_momdollyeo_momtong_btn.activate();
		} else {
			self.controls.hong_minus_momdollyeo_momtong_btn.deactivate();
		}
		
		self.controls.hong_plus_olgul_btn.activate();
		if self.hong_olgul_count > 0 {
			self.controls.hong_minus_olgul_btn.activate();
		} else {
			self.controls.hong_minus_olgul_btn.deactivate();
		}
		
		self.controls.hong_plus_momdollyeo_olgul_btn.activate();
		if self.hong_momdollyeo_olgul_count > 0 {
			self.controls.hong_minus_momdollyeo_olgul_btn.activate();
		} else {
			self.controls.hong_minus_momdollyeo_olgul_btn.deactivate();
		}
	}
	
	fn hide_scoring_controls(&mut self) {
		self.controls.cheong_plus_jirugi_btn.hide();
		self.controls.cheong_minus_jirugi_btn.hide();
		
		self.controls.cheong_plus_momtong_btn.hide();
		self.controls.cheong_minus_momtong_btn.hide();
		
		self.controls.cheong_plus_momdollyeo_momtong_btn.hide();
		self.controls.cheong_minus_momdollyeo_momtong_btn.hide();
		
		self.controls.cheong_plus_olgul_btn.hide();
		self.controls.cheong_minus_olgul_btn.hide();
		
		self.controls.cheong_plus_momdollyeo_olgul_btn.hide();
		self.controls.cheong_minus_momdollyeo_olgul_btn.hide();
		
		self.controls.cheong_plus_gam_jeon_btn.hide();
		self.controls.cheong_minus_gam_jeon_btn.hide();
		
		self.controls.hong_plus_gam_jeon_btn.hide();
		self.controls.hong_minus_gam_jeon_btn.hide();
		
		self.controls.hong_plus_jirugi_btn.hide();
		self.controls.hong_minus_jirugi_btn.hide();
		
		self.controls.hong_plus_momtong_btn.hide();
		self.controls.hong_minus_momtong_btn.hide();
		
		self.controls.hong_plus_momdollyeo_momtong_btn.hide();
		self.controls.hong_minus_momdollyeo_momtong_btn.hide();
		
		self.controls.hong_plus_olgul_btn.hide();
		self.controls.hong_minus_olgul_btn.hide();
		
		self.controls.hong_plus_momdollyeo_olgul_btn.hide();
		self.controls.hong_minus_momdollyeo_olgul_btn.hide();
	}
}