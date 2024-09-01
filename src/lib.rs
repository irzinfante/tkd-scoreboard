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

use std::sync::{Arc, Mutex};
use fltk::{prelude::*, frame, input, button, group};
use enums::{State, Winner};
use constants::{REST, ROUND};

pub mod constants;
pub mod enums;
pub mod common;
pub mod settings;
pub mod display;
pub mod screen;
pub mod controls;
pub mod medical_timeout;
pub mod superiority_decision;
pub mod end_contest;
pub mod contest_winner;
pub mod data;

pub struct Settings {
	pub contest_number_lbl: frame::Frame,
	pub contest_number_input: input::IntInput,
	
	pub round_time_lbl: frame::Frame,
	pub round_time_input: input::IntInput,
	pub round_time_seconds_lbl: frame::Frame,
	
	pub rest_time_lbl: frame::Frame,
	pub rest_time_input: input::IntInput,
	pub rest_time_seconds_lbl: frame::Frame,
	
	pub new_contest_btn: button::Button,
	
	pub vertical_separator_lbl: frame::Frame,
	
	pub data_scroll: group::Scroll,
	pub export_data_btn: button::Button,
	pub delete_data_btn: button::Button
}

pub struct Display {
	pub cheong_score_lbl: frame::Frame,
	pub cheong_gam_jeon_count_lbl: frame::Frame,
	pub cheong_seung_lbl: frame::Frame,
	
	pub hong_score_lbl: frame::Frame,
	pub hong_gam_jeon_count_lbl: frame::Frame,
	pub hong_seung_lbl: frame::Frame,
	
	pub round_rest_lbl: frame::Frame,
	pub round_number_lbl: frame::Frame,
	
	pub time_lbl: frame::Frame,
	
	pub kye_shi_lbl: frame::Frame,
	pub kye_shi_time_lbl: frame::Frame,
	
	pub superiority_decision_lbl: frame::Frame,
	pub end_contest_lbl: frame::Frame,
	
	pub contest_winner_lbl: frame::Frame,
	
	pub contest_number_lbl: frame::Frame
}

pub struct Screen {
	pub hong_score_lbl: frame::Frame,
	pub hong_gam_jeon_count_lbl: frame::Frame,
	pub hong_seung_lbl: frame::Frame,
	
	pub cheong_score_lbl: frame::Frame,
	pub cheong_gam_jeon_count_lbl: frame::Frame,
	pub cheong_seung_lbl: frame::Frame,
	
	pub round_rest_lbl: frame::Frame,
	pub round_number_lbl: frame::Frame,
	
	pub time_lbl: frame::Frame,
	
	pub kye_shi_time_lbl: frame::Frame,
	
	pub contest_winner_lbl: frame::Frame,
	
	pub contest_number_lbl: frame::Frame
}

pub struct Controls {
	pub shi_jak_btn: button::Button,
	pub kye_sok_btn: button::Button,
	pub kal_yeo_btn: button::Button,
	pub kye_shi_btn: button::Button,
	pub keu_man_btn: button::Button,
	pub end_contest_btn: button::Button,
	
	pub cheong_plus_jirugi_btn: button::Button,
	pub cheong_minus_jirugi_btn: button::Button,
	pub cheong_plus_momtong_btn: button::Button,
	pub cheong_minus_momtong_btn: button::Button,
	pub cheong_plus_momdollyeo_momtong_btn: button::Button,
	pub cheong_minus_momdollyeo_momtong_btn: button::Button,
	pub cheong_plus_olgul_btn: button::Button,
	pub cheong_minus_olgul_btn: button::Button,
	pub cheong_plus_momdollyeo_olgul_btn: button::Button,
	pub cheong_minus_momdollyeo_olgul_btn: button::Button,
	pub cheong_plus_gam_jeon_btn: button::Button,
	pub cheong_minus_gam_jeon_btn: button::Button,
	
	pub hong_plus_gam_jeon_btn: button::Button,
	pub hong_minus_gam_jeon_btn: button::Button,
	pub hong_plus_jirugi_btn: button::Button,
	pub hong_minus_jirugi_btn: button::Button,
	pub hong_plus_momtong_btn: button::Button,
	pub hong_minus_momtong_btn: button::Button,
	pub hong_plus_momdollyeo_momtong_btn: button::Button,
	pub hong_minus_momdollyeo_momtong_btn: button::Button,
	pub hong_plus_olgul_btn: button::Button,
	pub hong_minus_olgul_btn: button::Button,
	pub hong_plus_momdollyeo_olgul_btn: button::Button,
	pub hong_minus_momdollyeo_olgul_btn: button::Button,
	
	pub plus_one_second_btn: button::Button,
	pub minus_one_second_btn: button::Button,
	
	pub kye_shi_cancel_btn: button::Button,
	
	pub cheong_superiority_decision_btn: button::Button,
	pub hong_superiority_decision_btn: button::Button,
	
	pub cheong_end_contest_btn: button::Button,
	pub hong_end_contest_btn: button::Button,
	pub resume_contest_btn: button::Button,
	
	pub clear_scoreboard_btn: button::Button
}

pub struct Data {
	pub contest_id: String,
	
	pub contest_number: String,
	pub round_time: String,
	pub rest_time: String,
	
	pub cheong_score: [String; 3],
	pub cheong_gamjeon: [String; 3],
	pub hong_score: [String; 3],
	pub hong_gamjeon: [String; 3],
	pub round_winner: [String; 3],
	
	pub contest_winner: String
}

pub struct Scoreboard {
	pub settings: Settings,
	pub display: Display,
	pub screen: Screen,
	pub controls: Controls,
	
	pub data: Data,
	
	pub state: State,
	pub previous_state: State,
	
	pub round_time: f32,
	pub rest_time: f32,
	
    pub cheong_score: u8,
    pub cheong_gam_jeon_count: u8,
    
    pub hong_score: u8,
    pub hong_gam_jeon_count: u8,
    
    pub gam_jeons_hash: u64,
    
    pub round_number: u8,
    pub round_winner: [Winner; 3],
    pub is_after_reevaluation: bool,
    
    pub time: f32,
    
    pub blink_time: f32,
    pub blink: bool,
    
    pub kye_shi_time: f32,
    
    pub cheong_jirugi_count: u8,
    pub cheong_momtong_count: u8,
    pub cheong_momdollyeo_momtong_count: u8,
    pub cheong_olgul_count: u8,
    pub cheong_momdollyeo_olgul_count: u8,
    
    pub hong_jirugi_count: u8,
    pub hong_momtong_count: u8,
    pub hong_momdollyeo_momtong_count: u8,
    pub hong_olgul_count: u8,
    pub hong_momdollyeo_olgul_count: u8
}

impl Scoreboard {
	pub fn change_state(&mut self, new_state: State) {
		self.previous_state = self.state;
		self.state = new_state;
	}
	
	pub fn initialize_round(&mut self) {
		self.display.round_rest_lbl.set_label(ROUND);
		self.screen.round_rest_lbl.set_label(ROUND);
		self.round_number += 1;
		self.display.round_number_lbl.set_label(&self.round_number.to_string());
		self.screen.round_number_lbl.set_label(&self.round_number.to_string());
		
		self.time = self.round_time;
	}
	
	fn initialize_rest(&mut self) {
		self.display.round_rest_lbl.set_label(REST);
		self.screen.round_rest_lbl.set_label(REST);
		self.display.round_number_lbl.set_label("");
		self.screen.round_number_lbl.set_label("");
		
		self.time = self.rest_time;
	}
	
	pub fn variate_time_round(&mut self, var: f32) {
		self.time = self.round_time.min(0f32.max(self.time + var));
	}
	
	pub fn variate_time_rest(&mut self, var: f32) {
		self.time = self.rest_time.min(0f32.max(self.time + var));
	}
	
	pub fn variate_kye_shi_time(&mut self, var: f32) {
		self.kye_shi_time = 60f32.min(0f32.max(self.kye_shi_time + var));
	}
	
	pub fn is_keu_man_conditions(&self) -> bool {
		// Time runs out
		if self.time <= 0f32 {
			return true;
		}
		
		// Difference of 12 points
		if self.cheong_score.abs_diff(self.hong_score) >= 12 {
			return true;
		}
		
		// Receive 5 gam-jeons
		if self.cheong_gam_jeon_count >= 5 || self.hong_gam_jeon_count >= 5 {
			return true;
		}
		
		return false;
	}
	
	pub fn round_winner(&self) -> Winner {
		// 5 gam-jeons
		if self.cheong_gam_jeon_count >= 5 && self.hong_gam_jeon_count < 5 {
			return Winner::Hong;
		}
		if self.hong_gam_jeon_count >= 5 && self.cheong_gam_jeon_count < 5 {
			return Winner::Cheong;
		}
		
		// Best puntuation
		if self.cheong_score > self.hong_score {
			return Winner::Cheong;
		}
		if self.cheong_score < self.hong_score {
			return Winner::Hong;
		}
		
		// More turning/spinning kicks points
		let cheong_momdollyeo_score = self.cheong_momdollyeo_momtong_count*4 + self.cheong_momdollyeo_olgul_count*5;
		let hong_momdollyeo_score = self.hong_momdollyeo_momtong_count*4 + self.hong_momdollyeo_olgul_count*5;
		if cheong_momdollyeo_score > hong_momdollyeo_score {
			return Winner::Cheong;
		}
		if cheong_momdollyeo_score < hong_momdollyeo_score {
			return Winner::Hong;
		}
		
		// More olgul points
		let cheong_olgul_score = self.cheong_olgul_count*3 + self.cheong_momdollyeo_olgul_count*5;
		let hong_olgul_score = self.hong_olgul_count*3 + self.hong_momdollyeo_olgul_count*5;
		if cheong_olgul_score > hong_olgul_score {
			return Winner::Cheong;
		}
		if cheong_olgul_score < hong_olgul_score {
			return Winner::Hong;
		}
		
		// More momtong points
		let cheong_momtong_score = self.cheong_momtong_count*2 + self.cheong_momdollyeo_momtong_count*4;
		let hong_momtong_score = self.hong_momtong_count*2 + self.hong_momdollyeo_momtong_count*4;
		if cheong_momtong_score > hong_momtong_score {
			return Winner::Cheong;
		}
		if cheong_momtong_score < hong_momtong_score {
			return Winner::Hong;
		}
		
		// More jirugi points
		if self.cheong_jirugi_count > self.hong_jirugi_count {
			return Winner::Cheong;
		}
		if self.cheong_jirugi_count < self.hong_jirugi_count {
			return Winner::Hong;
		}
		
		// More gam-jeon points
		if self.cheong_gam_jeon_count < self.hong_gam_jeon_count {
			return Winner::Cheong;
		}
		if self.cheong_gam_jeon_count > self.hong_gam_jeon_count {
			return Winner::Hong;
		}
		
		// More hits count (Suppressed as of v1.1.0)
		/*
		let cheong_hits_count = self.cheong_jirugi_count + self.cheong_momtong_count + self.cheong_momdollyeo_momtong_count
			+ self.cheong_olgul_count + self.cheong_momdollyeo_olgul_count;
		let hong_hits_count = self.hong_jirugi_count + self.hong_momtong_count + self.hong_momdollyeo_momtong_count
			+ self.hong_olgul_count + self.hong_momdollyeo_olgul_count;
		if cheong_hits_count > hong_hits_count {
			return Winner::Cheong;
		} else if cheong_hits_count != hong_hits_count {
			return Winner::Hong;
		}
		*/
		
		// Superiority decision
		return Winner::None;
	}
	
	pub fn decide_round_winner(&mut self) {
		let winner = self.round_winner();
		self.round_winner[(self.round_number - 1) as usize] = winner;
		match winner {
			Winner::None => {
				self.change_state(State::SuperiorityDecision);
				self.update_controls();
				self.show_superiority_decision();
			},
			_ => {
				self.is_contest_winner();
			}
		}
	}
	
	pub fn is_contest_winner(&mut self) {		
		let mut cheong_wins_count = 0;
		let mut hong_wins_count = 0;
		for winner in self.round_winner.iter() {
			match winner {
				Winner::Cheong => {
					cheong_wins_count += 1;
				},
				Winner::Hong => {
					hong_wins_count += 1;
				},
				Winner::None => ()
			}
		}
		if cheong_wins_count >= 2 {
			self.save_round_data();
			self.save_contest_winner_data(Winner::Cheong);
			self.change_state(State::ContestWinner);
			self.update_controls();
			self.show_contest_winner(Winner::Cheong);
			self.write_data();
			return;
		}
		if hong_wins_count >= 2 {
			self.save_round_data();
			self.save_contest_winner_data(Winner::Hong);
			self.change_state(State::ContestWinner);
			self.update_controls();
			self.show_contest_winner(Winner::Hong);
			self.write_data();
			return;
		}
		
		self.show_round_winner_display();
		self.show_round_winner_screen();
		
		if self.is_after_reevaluation {
			self.is_after_reevaluation = false;
			self.save_round_data();
			self.clear_punctuation();
			self.change_state(State::RestSecondPart);
			self.update_controls();
		} else {
			self.change_state(State::RestFirstPart);
			self.update_controls();
			self.initialize_rest();
			self.gam_jeons_hash = self.gam_jeons_hash();
		}
	}
	
	pub fn is_after_5_seconds(&mut self) {
		if self.rest_time - self.time >= 5f32 {
			if self.gam_jeons_hash != self.gam_jeons_hash() && self.is_reevaluation() {
				self.is_after_reevaluation = true;
				self.hide_round_winner_display();
				self.hide_round_winner_screen();
				self.decide_round_winner();
			} else {
				self.save_round_data();
				self.clear_punctuation();
				self.change_state(State::RestSecondPart);
				self.update_controls();
			}
		}
	}
	
	fn is_reevaluation(&self) -> bool {
		let round_winner = self.round_winner[(self.round_number - 1) as usize];
		let new_winner = self.round_winner();
		match (round_winner, new_winner) {
			(Winner::Cheong, Winner::Hong) | (Winner::Hong, Winner::Cheong) | (_, Winner::None) => true,
			_ => false
		}
	}
	
	pub fn time_ends(&mut self) {
		if self.time <= 0f32 {
			self.change_state(State::CallContestants);
			self.update_controls();
			
			self.initialize_round();
					
			self.update_display();
			self.update_screen();	
		}
	}
	
	pub fn clear_scoreboard(&mut self) {
		self.data.contest_id = String::new();
		
		self.data.contest_number = String::new();
		self.data.round_time = String::new();
		self.data.rest_time = String::new();
		
		self.data.cheong_score = [(); 3].map(|_| String::from("-"));
		self.data.cheong_gamjeon = [(); 3].map(|_| String::from("-"));
		self.data.hong_score = [(); 3].map(|_| String::from("-"));
		self.data.hong_gamjeon = [(); 3].map(|_| String::from("-"));
		self.data.round_winner = [(); 3].map(|_| String::from("-"));
		
		self.data.contest_winner = String::new();
		
		self.round_time = 0f32;
		self.rest_time = 0f32;
		
		self.cheong_score = 0;
		self.cheong_gam_jeon_count = 0;
		
		self.hong_score = 0;
		self.hong_gam_jeon_count = 0;
		
		self.gam_jeons_hash = 0;
		
		self.round_number = 0;
		self.round_winner = [Winner::None; 3];
		self.is_after_reevaluation = false;
		
		self.time = 0f32;
		
		self.blink_time = 0f32;
		self.blink = false;
		
		self.kye_shi_time = 0f32;
		
		self.cheong_jirugi_count = 0;
		self.cheong_momtong_count = 0;
		self.cheong_momdollyeo_momtong_count = 0;
		self.cheong_olgul_count = 0;
		self.cheong_momdollyeo_olgul_count = 0;
		
		self.hong_jirugi_count = 0;
		self.hong_momtong_count = 0;
		self.hong_momdollyeo_momtong_count = 0;
		self.hong_olgul_count = 0;
		self.hong_momdollyeo_olgul_count = 0;
	}
	
	fn clear_punctuation(&mut self) {
		self.cheong_score = 0;
		self.cheong_gam_jeon_count = 0;
		
		self.hong_score = 0;
		self.hong_gam_jeon_count = 0;
		
		self.gam_jeons_hash = 0;
		
		self.cheong_jirugi_count = 0;
		self.cheong_momtong_count = 0;
		self.cheong_momdollyeo_momtong_count = 0;
		self.cheong_olgul_count = 0;
		self.cheong_momdollyeo_olgul_count = 0;
		
		self.hong_jirugi_count = 0;
		self.hong_momtong_count = 0;
		self.hong_momdollyeo_momtong_count = 0;
		self.hong_olgul_count = 0;
		self.hong_momdollyeo_olgul_count = 0;
	}
}

#[derive(Clone)]
pub struct ScoreboardHandle {
    inner: Arc<Mutex<Scoreboard>>
}

impl ScoreboardHandle {
    pub fn new(scoreboard: Scoreboard) -> Self {
        Self {
            inner: Arc::new(Mutex::new(scoreboard))
        }
    }

    pub fn with_lock<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&mut Scoreboard) -> T,
    {
        let mut lock = self.inner.lock().unwrap();
        let result = func(&mut *lock);
        drop(lock);
        result
    }
}