/**
 * Scoreboard program for taekwondo competition
 * Copyright (C) 2022 Iker Ruiz de Infante Gonzalez <contact@irzinfante.eu>
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

use fltk::{prelude::*, *};
use tkd_scoreboard::*;
use std::{thread, time::Duration};

fn main() {
    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();
    
    let mut main_win = window::Window::default()
    	.with_size(screen_width as i32, screen_height as i32)
    	.with_label("TKD Scoreboard");
    main_win.make_resizable(true);
	
	let display = Display {
		cheong_score_lbl: cheong_score_lbl(screen_width, screen_height),
		cheong_gam_jeon_count_lbl: cheong_gam_jeon_count_lbl(screen_width, screen_height),
		cheong_seung_lbl: cheong_seung_lbl(),
		
		hong_score_lbl: hong_score_lbl(screen_width, screen_height),
		hong_gam_jeon_count_lbl: hong_gam_jeon_count_lbl(screen_width, screen_height),
		hong_seung_lbl: hong_seung_lbl(),
		
		round_rest_lbl: round_rest_lbl(screen_width, screen_height),
		round_number_lbl: round_number_lbl(screen_width, screen_height),
		
		time_lbl: time_lbl(screen_width, screen_height),
		
		superiority_decision_lbl: superiority_decision_lbl(screen_width, screen_height),
		
		contest_winner_lbl: contest_winner_lbl(screen_width, screen_height)
	};
	
	let mut input_group = group::Group::default().with_size(screen_width as i32, screen_height as i32);
	let mut rount_time_lbl = round_time_lbl(screen_width, screen_height);
	let round_time_input = round_time_input(screen_width, screen_height);
	let mut round_time_seconds_lbl = round_time_seconds_lbl(screen_width, screen_height);
	let mut rest_time_lbl = rest_time_lbl(screen_width, screen_height);
	let rest_time_input = rest_time_input(screen_width, screen_height);
	let mut rest_time_seconds_lbl = rest_time_seconds_lbl(screen_width, screen_height);
	input_group.end();
	
	let controls = Controls {
		new_contest_btn: new_contest_btn(screen_width, screen_height),
		
		si_jak_btn: si_jak_btn(screen_width, screen_height),
		kye_sok_btn: kye_sok_btn(screen_width, screen_height),
		kal_yeo_btn: kal_yeo_btn(screen_width, screen_height),
		keu_man_btn: keu_man_btn(screen_width, screen_height),
		end_contest_btn: end_contest_btn(screen_width, screen_height),
		resume_contest_btn: resume_contest_btn(screen_width, screen_height),
		
		cheong_plus_jirugi_btn: cheong_plus_jirugi_btn(screen_width, screen_height),
		cheong_minus_jirugi_btn: cheong_minus_jirugi_btn(screen_width, screen_height),
		cheong_plus_momtong_btn: cheong_plus_momtong_btn(screen_width, screen_height),
		cheong_minus_momtong_btn: cheong_minus_momtong_btn(screen_width, screen_height),
		cheong_plus_momdollyeo_momtong_btn: cheong_plus_momdollyeo_momtong_btn(screen_width, screen_height),
		cheong_minus_momdollyeo_momtong_btn: cheong_minus_momdollyeo_momtong_btn(screen_width, screen_height),
		cheong_plus_olgul_btn: cheong_plus_olgul_btn(screen_width, screen_height),
		cheong_minus_olgul_btn: cheong_minus_olgul_btn(screen_width, screen_height),
		cheong_plus_momdollyeo_olgul_btn: cheong_plus_momdollyeo_olgul_btn(screen_width, screen_height),
		cheong_minus_momdollyeo_olgul_btn: cheong_minus_momdollyeo_olgul_btn(screen_width, screen_height),
		cheong_plus_gam_jeon_btn: cheong_plus_gam_jeon_btn(screen_width, screen_height),
		cheong_minus_gam_jeon_btn: cheong_minus_gam_jeon_btn(screen_width, screen_height),
		
		plus_1_second_btn: plus_1_second_btn(screen_width, screen_height),
		minus_1_second_btn: minus_1_second_btn(screen_width, screen_height),
		
		hong_plus_gam_jeon_btn: hong_plus_gam_jeon_btn(screen_width, screen_height),
		hong_minus_gam_jeon_btn: hong_minus_gam_jeon_btn(screen_width, screen_height),
		hong_plus_jirugi_btn: hong_plus_jirugi_btn(screen_width, screen_height),
		hong_minus_jirugi_btn: hong_minus_jirugi_btn(screen_width, screen_height),
		hong_plus_momtong_btn: hong_plus_momtong_btn(screen_width, screen_height),
		hong_minus_momtong_btn: hong_minus_momtong_btn(screen_width, screen_height),
		hong_plus_momdollyeo_momtong_btn: hong_plus_momdollyeo_momtong_btn(screen_width, screen_height),
		hong_minus_momdollyeo_momtong_btn: hong_minus_momdollyeo_momtong_btn(screen_width, screen_height),
		hong_plus_olgul_btn: hong_plus_olgul_btn(screen_width, screen_height),
		hong_minus_olgul_btn: hong_minus_olgul_btn(screen_width, screen_height),
		hong_plus_momdollyeo_olgul_btn: hong_plus_momdollyeo_olgul_btn(screen_width, screen_height),
		hong_minus_momdollyeo_olgul_btn: hong_minus_momdollyeo_olgul_btn(screen_width, screen_height),
		
		cheong_superiority_decision_btn: cheong_superiority_decision_btn(screen_width, screen_height),
		hong_superiority_decision_btn: hong_superiority_decision_btn(screen_width, screen_height)
	};
	
	copyright(screen_width, screen_height);
	
    main_win.end();
    main_win.show();
    
    let mut screen_win = window::Window::default()
    	.with_size(screen_width as i32, screen_height as i32);
    screen_win.make_resizable(true);
    
	let screen = Screen {
		hong_score_lbl: hong_score_screen_lbl(screen_width, screen_height),
		hong_gam_jeon_count_lbl: hong_gam_jeon_count_screen_lbl(screen_width, screen_height),
		hong_seung_lbl: hong_seung_screen_lbl(),
		
		cheong_score_lbl: cheong_score_screen_lbl(screen_width, screen_height),
		cheong_gam_jeon_count_lbl: cheong_gam_jeon_count_screen_lbl(screen_width, screen_height),
		cheong_seung_lbl: cheong_seung_screen_lbl(),
		
		round_rest_lbl: round_rest_screen_lbl(screen_width, screen_height),
		round_number_lbl: round_number_screen_lbl(screen_width, screen_height),
		
		time_lbl: time_screen_lbl(screen_width, screen_height),
		
		contest_winner_lbl: contest_winner_screen_lbl(screen_width, screen_height)
	};
	
	copyright(screen_width, screen_height);
	
    screen_win.end();
    screen_win.show();
    
    let scoreboard = ScoreboardHandle::new(
		Scoreboard {
			display,
			screen,
			controls,
			
			round_time: 0f32,
			rest_time: 0f32,
			
		    cheong_score: 0,
		    cheong_gam_jeon_count: 0,
		    
			hong_score: 0,
			hong_gam_jeon_count: 0,
		    
		    round_number: 0,
		    round_winner: [Winner::None; 3],
		    reevaluate: false,
		    
		    time: 0f32,
		    blink_time: 0f32,
		    
		    started: false,
		    time_running: false,
		    rest: false,
		    keu_man_superiority_decision: false,
		    end_contest: false,
		    
		    cheong_jirugi_count: 0,
		    cheong_momtong_count: 0,
		    cheong_momdollyeo_momtong_count: 0,
		    cheong_olgul_count: 0,
		    cheong_momdollyeo_olgul_count: 0,
		    
		    hong_jirugi_count: 0,
		    hong_momtong_count: 0,
		    hong_momdollyeo_momtong_count: 0,
		    hong_olgul_count: 0,
		    hong_momdollyeo_olgul_count: 0
		}
	);
    
	scoreboard.clone().with_lock(|share| {
		let scoreboard_new_contest_btn = scoreboard.clone();
		share.controls.new_contest_btn.set_callback(move |new_contest_btn| {
			scoreboard_new_contest_btn.with_lock(|share| {
				if valid_input(&round_time_input) && valid_input(&rest_time_input) {
					share.round_time = round_time_input.value().parse::<f32>().unwrap();
					share.rest_time = rest_time_input.value().parse::<f32>().unwrap();
					input_group.hide();
					
					share.display.time_lbl.set_color(enums::Color::Black);
					share.display.time_lbl.set_label_color(enums::Color::White);
					share.screen.time_lbl.set_color(enums::Color::Black);
					share.screen.time_lbl.set_label_color(enums::Color::White);
					share.variate_time(share.round_time);
					
					share.display.round_rest_lbl.set_label("ROUND");
					share.screen.round_rest_lbl.set_label("ROUND");
					share.new_round_display();
					
					share.display.cheong_seung_lbl.hide();
					share.display.hong_seung_lbl.hide();
					share.screen.cheong_seung_lbl.hide();
					share.screen.hong_seung_lbl.hide();
					
					share.display.contest_winner_lbl.hide();
					share.screen.contest_winner_lbl.hide();
					new_contest_btn.hide();
					
					share.controls.si_jak_btn.show();
					share.controls.si_jak_btn.activate();
					share.controls.kal_yeo_btn.show();
					share.controls.keu_man_btn.show();
					share.controls.end_contest_btn.show();
					share.controls.end_contest_btn.activate();
					
					share.controls.cheong_plus_jirugi_btn.deactivate();
					share.controls.cheong_plus_momtong_btn.deactivate();
					share.controls.cheong_plus_momdollyeo_momtong_btn.deactivate();
					share.controls.cheong_plus_olgul_btn.deactivate();
					share.controls.cheong_plus_momdollyeo_olgul_btn.deactivate();
					share.controls.cheong_plus_gam_jeon_btn.deactivate();
					share.controls.cheong_minus_jirugi_btn.deactivate();
					share.controls.cheong_minus_momtong_btn.deactivate();
					share.controls.cheong_minus_momdollyeo_momtong_btn.deactivate();
					share.controls.cheong_minus_olgul_btn.deactivate();
					share.controls.cheong_minus_momdollyeo_olgul_btn.deactivate();
					share.controls.cheong_minus_gam_jeon_btn.deactivate();
					
					share.controls.plus_1_second_btn.deactivate();
					share.controls.minus_1_second_btn.deactivate();
					
					share.controls.hong_plus_gam_jeon_btn.deactivate();
					share.controls.hong_plus_jirugi_btn.deactivate();
					share.controls.hong_plus_momtong_btn.deactivate();
					share.controls.hong_plus_momdollyeo_momtong_btn.deactivate();
					share.controls.hong_plus_olgul_btn.deactivate();
					share.controls.hong_plus_momdollyeo_olgul_btn.deactivate();
					share.controls.hong_minus_gam_jeon_btn.deactivate();
					share.controls.hong_minus_jirugi_btn.deactivate();
					share.controls.hong_minus_momtong_btn.deactivate();
					share.controls.hong_minus_momdollyeo_momtong_btn.deactivate();
					share.controls.hong_minus_olgul_btn.deactivate();
					share.controls.hong_minus_momdollyeo_olgul_btn.deactivate();
					
					share.show_screen_display();
					share.show_contest_controls();
				}
			});
		});
		
		let scoreboard_si_jak_btn = scoreboard.clone();
		share.controls.si_jak_btn.set_callback(move |si_jak_btn| {
			scoreboard_si_jak_btn.with_lock(|share| {
				share.started = true;
				share.time_running = true;
				
				si_jak_btn.deactivate();
				si_jak_btn.hide();
				share.controls.kye_sok_btn.show();
				share.controls.kal_yeo_btn.activate();
				share.controls.end_contest_btn.deactivate();
				
				share.controls.cheong_plus_jirugi_btn.activate();
				share.controls.cheong_plus_momtong_btn.activate();
				share.controls.cheong_plus_momdollyeo_momtong_btn.activate();
				share.controls.cheong_plus_olgul_btn.activate();
				share.controls.cheong_plus_momdollyeo_olgul_btn.activate();
				share.controls.cheong_plus_gam_jeon_btn.activate();
				
				share.controls.plus_1_second_btn.deactivate();
				share.controls.minus_1_second_btn.deactivate();
				
				share.controls.hong_plus_gam_jeon_btn.activate();
				share.controls.hong_plus_jirugi_btn.activate();
				share.controls.hong_plus_momtong_btn.activate();
				share.controls.hong_plus_momdollyeo_momtong_btn.activate();
				share.controls.hong_plus_olgul_btn.activate();
				share.controls.hong_plus_momdollyeo_olgul_btn.activate();
			});
		});
		
		let scoreboard_kye_sok_btn = scoreboard.clone();
		share.controls.kye_sok_btn.set_callback(move |kye_sok_btn| {
			scoreboard_kye_sok_btn.with_lock(|share| {
				share.time_running = true;
				share.display.time_lbl.set_color(enums::Color::Black);
				share.display.time_lbl.set_label_color(enums::Color::White);
				share.screen.time_lbl.set_color(enums::Color::Black);
				share.screen.time_lbl.set_label_color(enums::Color::White);
				kye_sok_btn.deactivate();
				share.controls.kal_yeo_btn.activate();
				share.controls.end_contest_btn.deactivate();
					
				share.controls.plus_1_second_btn.deactivate();
				share.controls.minus_1_second_btn.deactivate();
			});
		});
		
		let scoreboard_kal_yeo_btn = scoreboard.clone();
		share.controls.kal_yeo_btn.set_callback(move |kal_yeo_btn| {
			scoreboard_kal_yeo_btn.with_lock(|share| {
				share.time_running = false;
				share.blink_time = 0f32;
				kal_yeo_btn.deactivate();
				share.controls.plus_1_second_btn.activate();
				share.controls.minus_1_second_btn.activate();
				share.controls.kye_sok_btn.activate();
				share.controls.end_contest_btn.activate();
			});
		});
		
		let scoreboard_keu_man_btn = scoreboard.clone();
		share.controls.keu_man_btn.set_callback(move |keu_man_btn| {
			scoreboard_keu_man_btn.with_lock(|share| {
				let winner = share.round_winner();
				if let Winner::None = winner {
					share.hide_contest_controls();
					share.started = false;
					keu_man_btn.deactivate();
					share.show_superiority_decision_controls(false);
				} else {
					share.decide_winner(winner);
				}
			});
		});
		
		let scoreboard_end_contest_btn = scoreboard.clone();
		share.controls.end_contest_btn.set_callback(move |_end_contest_btn| {
			scoreboard_end_contest_btn.with_lock(|share| {
				share.end_contest();
			});
		});
		
		let scoreboard_resume_contest_btn = scoreboard.clone();
		share.controls.resume_contest_btn.set_callback(move |resume_contest_btn| {
			scoreboard_resume_contest_btn.with_lock(|share| {
				if share.started && share.is_termination_condition() {
					share.controls.keu_man_btn.activate();
				} else if share.started && !share.keu_man_superiority_decision {
					share.controls.kye_sok_btn.activate();
				} else if share.rest {
					share.time_running = true;
				} else {
					share.controls.si_jak_btn.activate();
				}
				if share.keu_man_superiority_decision {
					share.show_superiority_decision_controls(false);
				} else {
					share.hide_superiority_decision_controls();
					share.show_contest_controls();
				}
				share.end_contest = false;
				resume_contest_btn.deactivate();
				resume_contest_btn.hide();
				share.controls.end_contest_btn.activate();
				share.controls.end_contest_btn.show();
			});
		});
		
		let scoreboard_cheong_plus_jirugi_btn = scoreboard.clone();
		share.controls.cheong_plus_jirugi_btn.set_callback(move |_cheong_plus_jirugi_btn| {
			scoreboard_cheong_plus_jirugi_btn.with_lock(|share| {
				share.variate_cheong_score(1);
				share.cheong_jirugi_count += 1;
				share.controls.cheong_minus_jirugi_btn.activate();
			});
		});
		
		let scoreboard_cheong_minus_jirugi_btn = scoreboard.clone();
		share.controls.cheong_minus_jirugi_btn.set_callback(move |cheong_minus_jirugi_btn| {
			scoreboard_cheong_minus_jirugi_btn.with_lock(|share| {
				share.variate_cheong_score(-1);
				share.cheong_jirugi_count -= 1;
				if share.cheong_jirugi_count == 0 {
					cheong_minus_jirugi_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_plus_momtong_btn = scoreboard.clone();
		share.controls.cheong_plus_momtong_btn.set_callback(move |_cheong_plus_momtong_btn| {
			scoreboard_cheong_plus_momtong_btn.with_lock(|share| {
				share.variate_cheong_score(2);
				share.cheong_momtong_count += 1;
				share.controls.cheong_minus_momtong_btn.activate();
			});
		});
		
		let scoreboard_cheong_minus_momtong_btn = scoreboard.clone();
		share.controls.cheong_minus_momtong_btn.set_callback(move |cheong_minus_momtong_btn| {
			scoreboard_cheong_minus_momtong_btn.with_lock(|share| {
				share.variate_cheong_score(-2);
				share.cheong_momtong_count -= 1;
				if share.cheong_momtong_count == 0 {
					cheong_minus_momtong_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_plus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.cheong_plus_momdollyeo_momtong_btn.set_callback(move |_cheong_plus_momdollyeo_momtong_btn| {
			scoreboard_cheong_plus_momdollyeo_momtong_btn.with_lock(|share| {
				share.variate_cheong_score(4);
				share.cheong_momdollyeo_momtong_count += 1;
				share.controls.cheong_minus_momdollyeo_momtong_btn.activate();
			});
		});
		
		let scoreboard_cheong_minus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.cheong_minus_momdollyeo_momtong_btn.set_callback(move |cheong_minus_momdollyeo_momtong_btn| {
			scoreboard_cheong_minus_momdollyeo_momtong_btn.with_lock(|share| {
				share.variate_cheong_score(-4);
				share.cheong_momdollyeo_momtong_count -= 1;
				if share.cheong_momdollyeo_momtong_count == 0 {
					cheong_minus_momdollyeo_momtong_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_plus_olgul_btn = scoreboard.clone();
		share.controls.cheong_plus_olgul_btn.set_callback(move |_cheong_plus_olgul_btn| {
			scoreboard_cheong_plus_olgul_btn.with_lock(|share| {
				share.variate_cheong_score(3);
				share.cheong_olgul_count += 1;
				share.controls.cheong_minus_olgul_btn.activate();
			});
		});
		
		let scoreboard_cheong_minus_olgul_btn = scoreboard.clone();
		share.controls.cheong_minus_olgul_btn.set_callback(move |cheong_minus_olgul_btn| {
			scoreboard_cheong_minus_olgul_btn.with_lock(|share| {
				share.variate_cheong_score(-3);
				share.cheong_olgul_count -= 1;
				if share.cheong_olgul_count == 0 {
					cheong_minus_olgul_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_plus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.cheong_plus_momdollyeo_olgul_btn.set_callback(move |_cheong_plus_momdollyeo_olgul_btn| {
			scoreboard_cheong_plus_momdollyeo_olgul_btn.with_lock(|share| {
				share.variate_cheong_score(5);
				share.cheong_momdollyeo_olgul_count += 1;
				share.controls.cheong_minus_momdollyeo_olgul_btn.activate();
			});
		});
		
		let scoreboard_cheong_minus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.cheong_minus_momdollyeo_olgul_btn.set_callback(move |cheong_minus_momdollyeo_olgul_btn| {
			scoreboard_cheong_minus_momdollyeo_olgul_btn.with_lock(|share| {
				share.variate_cheong_score(-5);
				share.cheong_momdollyeo_olgul_count -= 1;
				if share.cheong_momdollyeo_olgul_count == 0 {
					cheong_minus_momdollyeo_olgul_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_plus_gam_jeon_btn = scoreboard.clone();
		share.controls.cheong_plus_gam_jeon_btn.set_callback(move |cheong_plus_gam_jeon_btn| {
			scoreboard_cheong_plus_gam_jeon_btn.with_lock(|share| {
				share.variate_cheong_gam_jeon_count(1);
				share.variate_hong_score(1);
				share.controls.cheong_minus_gam_jeon_btn.activate();
				if share.rest {
					share.reevaluate = true;
				}
				if share.cheong_gam_jeon_count > 4 {
					cheong_plus_gam_jeon_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_minus_gam_jeon_btn = scoreboard.clone();
		share.controls.cheong_minus_gam_jeon_btn.set_callback(move |cheong_minus_gam_jeon_btn| {
			scoreboard_cheong_minus_gam_jeon_btn.with_lock(|share| {
				share.variate_cheong_gam_jeon_count(-1);
				share.variate_hong_score(-1);
				share.controls.cheong_plus_gam_jeon_btn.activate();
				if share.cheong_gam_jeon_count == 0 {
					cheong_minus_gam_jeon_btn.deactivate();
				}
			});
		});
		
		let scoreboard_plus_1_second_btn = scoreboard.clone();
		share.controls.plus_1_second_btn.set_callback(move |_plus_1_second_btn| {
			scoreboard_plus_1_second_btn.with_lock(|share| {
				share.variate_time(1f32);
			});
		});
		
		let scoreboard_minus_1_second_btn = scoreboard.clone();
		share.controls.minus_1_second_btn.set_callback(move |_minus_1_second_btn| {
			scoreboard_minus_1_second_btn.with_lock(|share| {
				share.variate_time(-1f32);
			});
		});
		
		let scoreboard_hong_plus_gam_jeon_btn = scoreboard.clone();
		share.controls.hong_plus_gam_jeon_btn.set_callback(move |hong_plus_gam_jeon_btn| {
			scoreboard_hong_plus_gam_jeon_btn.with_lock(|share| {
				share.variate_hong_gam_jeon_count(1);
				share.variate_cheong_score(1);
				share.controls.hong_minus_gam_jeon_btn.activate();
				if share.rest {
					share.reevaluate = true;
				}
				if share.hong_gam_jeon_count > 4 {
					hong_plus_gam_jeon_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_minus_gam_jeon_btn = scoreboard.clone();
		share.controls.hong_minus_gam_jeon_btn.set_callback(move |hong_minus_gam_jeon_btn| {
			scoreboard_hong_minus_gam_jeon_btn.with_lock(|share| {
				share.variate_hong_gam_jeon_count(-1);
				share.variate_cheong_score(-1);
				share.controls.hong_plus_gam_jeon_btn.activate();
				if share.hong_gam_jeon_count == 0 {
					hong_minus_gam_jeon_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_plus_jirugi_btn = scoreboard.clone();
		share.controls.hong_plus_jirugi_btn.set_callback(move |_hong_plus_jirugi_btn| {
			scoreboard_hong_plus_jirugi_btn.with_lock(|share| {
				share.variate_hong_score(1);
				share.hong_jirugi_count += 1;
				share.controls.hong_minus_jirugi_btn.activate();
			});
		});
		
		let scoreboard_hong_minus_jirugi_btn = scoreboard.clone();
		share.controls.hong_minus_jirugi_btn.set_callback(move |hong_minus_jirugi_btn| {
			scoreboard_hong_minus_jirugi_btn.with_lock(|share| {
				share.variate_hong_score(-1);
				share.hong_jirugi_count -= 1;
				if share.hong_jirugi_count == 0 {
					hong_minus_jirugi_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_plus_momtong_btn = scoreboard.clone();
		share.controls.hong_plus_momtong_btn.set_callback(move |_hong_plus_momtong_btn| {
			scoreboard_hong_plus_momtong_btn.with_lock(|share| {
				share.variate_hong_score(2);
				share.hong_momtong_count += 1;
				share.controls.hong_minus_momtong_btn.activate();
			});
		});
		
		let scoreboard_hong_minus_momtong_btn = scoreboard.clone();
		share.controls.hong_minus_momtong_btn.set_callback(move |hong_minus_momtong_btn| {
			scoreboard_hong_minus_momtong_btn.with_lock(|share| {
				share.variate_hong_score(-2);
				share.hong_momtong_count -= 1;
				if share.hong_momtong_count == 0 {
					hong_minus_momtong_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_plus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.hong_plus_momdollyeo_momtong_btn.set_callback(move |_hong_plus_momdollyeo_momtong_btn| {
			scoreboard_hong_plus_momdollyeo_momtong_btn.with_lock(|share| {
				share.variate_hong_score(4);
				share.hong_momdollyeo_momtong_count += 1;
				share.controls.hong_minus_momdollyeo_momtong_btn.activate();
			});
		});
		
		let scoreboard_hong_minus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.hong_minus_momdollyeo_momtong_btn.set_callback(move |hong_minus_momdollyeo_momtong_btn| {
			scoreboard_hong_minus_momdollyeo_momtong_btn.with_lock(|share| {
				share.variate_hong_score(-4);
				share.hong_momdollyeo_momtong_count -= 1;
				if share.hong_momdollyeo_momtong_count == 0 {
					hong_minus_momdollyeo_momtong_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_plus_olgul_btn = scoreboard.clone();
		share.controls.hong_plus_olgul_btn.set_callback(move |_hong_plus_olgul_btn| {
			scoreboard_hong_plus_olgul_btn.with_lock(|share| {
				share.variate_hong_score(3);
				share.hong_olgul_count += 1;
				share.controls.hong_minus_olgul_btn.activate();
			});
		});
		
		let scoreboard_hong_minus_olgul_btn = scoreboard.clone();
		share.controls.hong_minus_olgul_btn.set_callback(move |hong_minus_olgul_btn| {
			scoreboard_hong_minus_olgul_btn.with_lock(|share| {
				share.variate_hong_score(-3);
				share.hong_olgul_count -= 1;
				if share.hong_olgul_count == 0 {
					hong_minus_olgul_btn.deactivate();
				}
			});
		});
		
		let scoreboard_hong_plus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.hong_plus_momdollyeo_olgul_btn.set_callback(move |_hong_plus_momdollyeo_olgul_btn| {
			scoreboard_hong_plus_momdollyeo_olgul_btn.with_lock(|share| {
				share.variate_hong_score(5);
				share.hong_momdollyeo_olgul_count += 1;
				share.controls.hong_minus_momdollyeo_olgul_btn.activate();
			});
		});
		
		let scoreboard_hong_minus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.hong_minus_momdollyeo_olgul_btn.set_callback(move |hong_minus_momdollyeo_olgul_btn| {
			scoreboard_hong_minus_momdollyeo_olgul_btn.with_lock(|share| {
				share.variate_hong_score(-5);
				share.hong_momdollyeo_olgul_count -= 1;
				if share.hong_momdollyeo_olgul_count == 0 {
					hong_minus_momdollyeo_olgul_btn.deactivate();
				}
			});
		});
		
		let scoreboard_cheong_superiority_decision_btn = scoreboard.clone();
		share.controls.cheong_superiority_decision_btn.set_callback(move |_cheong_superiority_decision_btn| {
			scoreboard_cheong_superiority_decision_btn.with_lock(|share| {
				share.decide_winner(Winner::Cheong);
			})
		});
		
		let scoreboard_hong_superiority_decision_btn = scoreboard.clone();
		share.controls.hong_superiority_decision_btn.set_callback(move |_hong_superiority_decision_btn| {
			scoreboard_hong_superiority_decision_btn.with_lock(|share| {
				share.decide_winner(Winner::Hong);
			})
		});
	});	
    
    let scoreboard_resize = scoreboard.clone();
    main_win.resize_callback(move |_win, _x, _y, w, h| {
		let (x, y, d) = seung_lbl_dimensions(w as f64, h as f64);
		
		scoreboard_resize.with_lock(|share| {
			rount_time_lbl.set_label_size(scale_size(25., w as f64, h as f64));
			round_time_seconds_lbl.set_label_size(scale_size(15., w as f64, h as f64));
			rest_time_lbl.set_label_size(scale_size(25., w as f64, h as f64));
			rest_time_seconds_lbl.set_label_size(scale_size(15., w as f64, h as f64));
			
			share.display.cheong_score_lbl.set_label_size(scale_size(400., w as f64, h as f64));
			share.display.cheong_gam_jeon_count_lbl.set_label_size(scale_size(90., w as f64, h as f64));
			share.display.cheong_seung_lbl.set_pos(((w as f64) * 1./3. + x) as i32, y as i32);
			share.display.cheong_seung_lbl.set_size(d, d);
			
			share.display.hong_score_lbl.set_label_size(scale_size(400., w as f64, h as f64));
			share.display.hong_gam_jeon_count_lbl.set_label_size(scale_size(90., w as f64, h as f64));
			share.display.hong_seung_lbl.set_pos(((w as f64) * 5./9. + x) as i32, y as i32);
			share.display.hong_seung_lbl.set_size(d, d);
			
			share.display.round_rest_lbl.set_label_size(scale_size(40., w as f64, h as f64));
			share.display.round_number_lbl.set_label_size(scale_size(150., w as f64, h as f64));
			share.display.time_lbl.set_label_size(scale_size(225., w as f64, h as f64));
			
			share.controls.new_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.si_jak_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.kye_sok_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.kal_yeo_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.keu_man_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.end_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.resume_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.display.contest_winner_lbl.set_label_size(scale_size(120., w as f64, h as f64));
			
			share.controls.cheong_plus_jirugi_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_JIRUGI).unwrap()));
			share.controls.cheong_minus_jirugi_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_JIRUGI).unwrap()));
			share.controls.cheong_plus_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMTONG).unwrap()));
			share.controls.cheong_minus_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMTONG).unwrap()));
			share.controls.cheong_plus_momdollyeo_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMDOLLYEO_MOMTONG).unwrap()));
			share.controls.cheong_minus_momdollyeo_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMDOLLYEO_MOMTONG).unwrap()));
			share.controls.cheong_plus_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_OLGUL).unwrap()));
			share.controls.cheong_minus_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_OLGUL).unwrap()));
			share.controls.cheong_plus_momdollyeo_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMDOLLYEO_OLGUL).unwrap()));
			share.controls.cheong_minus_momdollyeo_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMDOLLYEO_OLGUL).unwrap()));
			share.controls.cheong_plus_gam_jeon_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.cheong_minus_gam_jeon_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.controls.plus_1_second_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.minus_1_second_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.controls.hong_plus_gam_jeon_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.hong_minus_gam_jeon_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.hong_plus_jirugi_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_JIRUGI).unwrap()));
			share.controls.hong_minus_jirugi_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_JIRUGI).unwrap()));
			share.controls.hong_plus_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMTONG).unwrap()));
			share.controls.hong_minus_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMTONG).unwrap()));
			share.controls.hong_plus_momdollyeo_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMDOLLYEO_MOMTONG).unwrap()));
			share.controls.hong_minus_momdollyeo_momtong_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMDOLLYEO_MOMTONG).unwrap()));
			share.controls.hong_plus_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_OLGUL).unwrap()));
			share.controls.hong_minus_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_OLGUL).unwrap()));
			share.controls.hong_plus_momdollyeo_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(PLUS_MOMDOLLYEO_OLGUL).unwrap()));
			share.controls.hong_minus_momdollyeo_olgul_btn.set_image_scaled(Some(image::SvgImage::from_data(MINUS_MOMDOLLYEO_OLGUL).unwrap()));
			
			share.display.superiority_decision_lbl.set_label_size(scale_size(40., w as f64, h as f64));
			share.controls.cheong_superiority_decision_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.hong_superiority_decision_btn.set_label_size(scale_size(25., w as f64, h as f64));
		});
	});
	
	let screen_resize = scoreboard.clone();
    screen_win.resize_callback(move |_win, _x, _y, w, h| {
		let (x, y, d) = seung_screen_lbl_dimensions(w as f64, h as f64);
		
		screen_resize.with_lock(|share| {
			share.screen.hong_score_lbl.set_label_size(scale_size(475., w as f64, h as f64));
			share.screen.hong_gam_jeon_count_lbl.set_label_size(scale_size(125., w as f64, h as f64));
			share.screen.hong_seung_lbl.set_pos(((w as f64) * 4./18. + x) as i32, y as i32);
			share.screen.hong_seung_lbl.set_size(d, d);
			
			share.screen.cheong_score_lbl.set_label_size(scale_size(475., w as f64, h as f64));
			share.screen.cheong_gam_jeon_count_lbl.set_label_size(scale_size(125., w as f64, h as f64));
			share.screen.cheong_seung_lbl.set_pos(((w as f64) * 12./18. + x) as i32, y as i32);
			share.screen.cheong_seung_lbl.set_size(d, d);
			
			share.screen.round_rest_lbl.set_label_size(scale_size(75., w as f64, h as f64));
			share.screen.round_number_lbl.set_label_size(scale_size(175., w as f64, h as f64));
			share.screen.time_lbl.set_label_size(scale_size(275., w as f64, h as f64));
			
			share.screen.contest_winner_lbl.set_label_size(scale_size(100., w as f64, h as f64));
		});
	});

    while app.wait() {
		if !main_win.shown() || !screen_win.shown() {
			panic!("Exit!");
		}
		thread::sleep(Duration::from_millis(100));
		scoreboard.with_lock(|share| {
			if share.started {
				if share.is_termination_condition() && !share.end_contest {
					share.time_running = false;
					
					share.controls.kye_sok_btn.deactivate();
					share.controls.kal_yeo_btn.deactivate();
					share.controls.keu_man_btn.activate();
					share.controls.end_contest_btn.activate();
					
					share.controls.plus_1_second_btn.activate();
					share.controls.minus_1_second_btn.activate();
					
					share.display.time_lbl.set_color(enums::Color::White);
					share.display.time_lbl.set_label_color(enums::Color::Black);
					share.screen.time_lbl.set_color(enums::Color::White);
					share.screen.time_lbl.set_label_color(enums::Color::Black);
					share.variate_time(0f32);
				} else {
					if share.time_running {
						share.variate_time(-0.1);
					} else if !share.is_termination_condition() {
						if !share.end_contest {
							share.controls.kye_sok_btn.activate();
							share.controls.keu_man_btn.deactivate();
						}
						if share.blink_time >= 0.5 {
							if share.display.time_lbl.color() == enums::Color::White {
								share.display.time_lbl.set_color(enums::Color::Black);
								share.display.time_lbl.set_label_color(enums::Color::White);
								share.screen.time_lbl.set_color(enums::Color::Black);
								share.screen.time_lbl.set_label_color(enums::Color::White);
							} else {
								share.display.time_lbl.set_color(enums::Color::White);
								share.display.time_lbl.set_label_color(enums::Color::Black);
								share.screen.time_lbl.set_color(enums::Color::White);
								share.screen.time_lbl.set_label_color(enums::Color::Black);
							}
							share.blink_time -= 0.5;
						} else {
							share.blink_time += 0.1;
						}
						share.variate_time(0f32);
					}
				}
			} else if share.rest {
				if share.time_running {
					share.variate_time(-0.1);
					if ((share.rest_time - share.time)*10.).trunc() / 10. == 5f32 {
						if share.reevaluate {
							let winner = share.round_winner();
							if let Winner::None = winner {
								share.hide_contest_controls();
								share.time_running = false;
								share.show_superiority_decision_controls(false);
							} else {
								share.decide_winner(winner);
							}
						} else {
							share.new_round_display();
						}
					} else if share.time == 0f32 {
						share.display.round_rest_lbl.set_label("ROUND");
						share.display.round_number_lbl.set_label(&share.round_number.to_string());
						share.screen.round_rest_lbl.set_label("ROUND");
						share.screen.round_number_lbl.set_label(&share.round_number.to_string());
						share.variate_time(share.round_time);
						share.controls.si_jak_btn.activate();
						
						share.rest = false;
						share.started = false;
						share.time_running = false;
					}
				}
			}
		});
	}
}

fn valid_input(input: &input::IntInput) -> bool {
	match input.value().parse::<f32>() {
		Ok(value) => {
			if value <= 0f32 {
				return false;
			} else {
				return true;
			}
		},
		Err(_) => false
	}
}

fn scale_size(size: f64, width: f64, height: f64) -> i32 {
	(size * (width.powf(2.) + height.powf(2.)).sqrt() / 2000.) as i32
}

fn seung_lbl_dimensions(screen_width: f64, screen_height: f64) -> (f64, f64, i32) {
	let w = screen_width * 1./9.;
	let h = screen_height * 1./8.;
	let d = w.min(h);
	return ((w - d) / 2., ((h - d) / 2.), d as i32);
}

fn seung_screen_lbl_dimensions(screen_width: f64, screen_height: f64) -> (f64, f64, i32) {
	let w = screen_width * 2./18.;
	let h = screen_height * 3./16.;
	let d = w.min(h);
	return ((w - d) / 2., ((h - d) / 2.), d as i32);
}

fn cheong_score_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_score_lbl = frame::Frame::default()
    	.with_pos(0, 0)
		.with_size((screen_width * 1./3.) as i32, (screen_height * 1./2.) as i32);
	cheong_score_lbl.set_frame(enums::FrameType::FlatBox);
	cheong_score_lbl.set_color(enums::Color::Blue);
	cheong_score_lbl.set_label_color(enums::Color::White);
	return cheong_score_lbl;
}

fn cheong_gam_jeon_count_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_gam_jeon_count_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 2./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 2./16.) as i32);
	cheong_gam_jeon_count_lbl.set_frame(enums::FrameType::FlatBox);
	cheong_gam_jeon_count_lbl.set_color(enums::Color::Blue.darker());
	cheong_gam_jeon_count_lbl.set_label_color(enums::Color::White);
	return cheong_gam_jeon_count_lbl;
}

fn cheong_seung_lbl() -> frame::Frame {
	let mut cheong_round_seung_lbl = frame::Frame::default();
	cheong_round_seung_lbl.set_frame(enums::FrameType::OFlatFrame);
	cheong_round_seung_lbl.set_color(enums::Color::Blue);
	cheong_round_seung_lbl.hide();
	return cheong_round_seung_lbl;
}

fn hong_score_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_score_lbl = frame::Frame::default()
    	.with_pos((screen_width * 2./3.) as i32, 0)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./2.) as i32);
	hong_score_lbl.set_frame(enums::FrameType::FlatBox);
	hong_score_lbl.set_color(enums::Color::Red);
	hong_score_lbl.set_label_color(enums::Color::White);
	return hong_score_lbl;
}

fn hong_gam_jeon_count_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_gam_jeon_count_lbl = frame::Frame::default()
		.with_pos((screen_width * 5./9.) as i32, (screen_height * 2./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 2./16.) as i32);
	hong_gam_jeon_count_lbl.set_frame(enums::FrameType::FlatBox);
	hong_gam_jeon_count_lbl.set_color(enums::Color::Red.darker());
	hong_gam_jeon_count_lbl.set_label_color(enums::Color::White);
	return hong_gam_jeon_count_lbl;
}

fn hong_seung_lbl() -> frame::Frame {
	let mut hong_round_seung_lbl = frame::Frame::default();
	hong_round_seung_lbl.set_frame(enums::FrameType::OFlatFrame);
	hong_round_seung_lbl.set_color(enums::Color::Red);
	hong_round_seung_lbl.hide();
	return hong_round_seung_lbl;
}

fn round_rest_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, 0)
		.with_size((screen_width *  1./9.) as i32, (screen_height * 1./16.) as i32);
	round_lbl.set_frame(enums::FrameType::FlatBox);
	round_lbl.set_color(enums::Color::Black);
	round_lbl.set_label_color(enums::Color::White);
	return round_lbl;
}

fn round_number_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_number_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 1./16.) as i32)
		.with_size((screen_width *  1./9.) as i32, (screen_height * 3./16.) as i32);
	round_number_lbl.set_frame(enums::FrameType::FlatBox);
	round_number_lbl.set_color(enums::Color::Black);
	round_number_lbl.set_label_color(enums::Color::White);
	return round_number_lbl;
}

fn time_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut time_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 1./4.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 1./4.) as i32);
	time_lbl.set_frame(enums::FrameType::FlatBox);
	time_lbl.set_color(enums::Color::Black);
	time_lbl.set_label_color(enums::Color::White);
	return time_lbl;
}

fn hong_score_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_score_screen_lbl = frame::Frame::default()
    	.with_pos(0, (screen_height * 3./16.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 12./16.) as i32);
	hong_score_screen_lbl.set_frame(enums::FrameType::FlatBox);
	hong_score_screen_lbl.set_color(enums::Color::Red);
	hong_score_screen_lbl.set_label_color(enums::Color::White);
	hong_score_screen_lbl.hide();
	return hong_score_screen_lbl;
}

fn hong_gam_jeon_count_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut hong_gam_jeon_count_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./18.) as i32, 0)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 3./16.) as i32);
	hong_gam_jeon_count_screen_lbl.set_frame(enums::FrameType::FlatBox);
	hong_gam_jeon_count_screen_lbl.set_color(enums::Color::Red.darker());
	hong_gam_jeon_count_screen_lbl.set_label_color(enums::Color::White);
	hong_gam_jeon_count_screen_lbl.hide();
	return hong_gam_jeon_count_screen_lbl;
}

fn hong_seung_screen_lbl() -> frame::Frame {
	let mut hong_round_seung_screen_lbl = frame::Frame::default();
	hong_round_seung_screen_lbl.set_frame(enums::FrameType::OFlatFrame);
	hong_round_seung_screen_lbl.set_color(enums::Color::Red);
	hong_round_seung_screen_lbl.hide();
	return hong_round_seung_screen_lbl;
}

fn cheong_score_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_score_screen_lbl = frame::Frame::default()
    	.with_pos((screen_width * 2./3.) as i32, (screen_height * 3./16.) as i32)
		.with_size((screen_width * 1./3.) as i32, (screen_height * 12./16.) as i32);
	cheong_score_screen_lbl.set_frame(enums::FrameType::FlatBox);
	cheong_score_screen_lbl.set_color(enums::Color::Blue);
	cheong_score_screen_lbl.set_label_color(enums::Color::White);
	cheong_score_screen_lbl.hide();
	return cheong_score_screen_lbl;
}

fn cheong_gam_jeon_count_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut cheong_gam_jeon_count_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 15./18.) as i32, 0)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 3./16.) as i32);
	cheong_gam_jeon_count_screen_lbl.set_frame(enums::FrameType::FlatBox);
	cheong_gam_jeon_count_screen_lbl.set_color(enums::Color::Blue.darker());
	cheong_gam_jeon_count_screen_lbl.set_label_color(enums::Color::White);
	cheong_gam_jeon_count_screen_lbl.hide();
	return cheong_gam_jeon_count_screen_lbl;
}

fn cheong_seung_screen_lbl() -> frame::Frame {
	let mut cheong_round_seung_screen_lbl = frame::Frame::default();
	cheong_round_seung_screen_lbl.set_frame(enums::FrameType::OFlatFrame);
	cheong_round_seung_screen_lbl.set_color(enums::Color::Blue);
	cheong_round_seung_screen_lbl.hide();
	return cheong_round_seung_screen_lbl;
}

fn round_rest_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 15./36.) as i32, (screen_height * 1./32.) as i32)
		.with_size((screen_width *  3./18.) as i32, (screen_height * 3./32.) as i32);
	round_screen_lbl.set_frame(enums::FrameType::FlatBox);
	round_screen_lbl.set_color(enums::Color::Black);
	round_screen_lbl.set_label_color(enums::Color::White);
	round_screen_lbl.hide();
	return round_screen_lbl;
}

fn round_number_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_number_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 15./36.) as i32, (screen_height * 4./32.) as i32)
		.with_size((screen_width *  3./18.) as i32, (screen_height * 7./32.) as i32);
	round_number_screen_lbl.set_frame(enums::FrameType::FlatBox);
	round_number_screen_lbl.set_color(enums::Color::Black);
	round_number_screen_lbl.set_label_color(enums::Color::White);
	round_number_screen_lbl.hide();
	return round_number_screen_lbl;
}

fn time_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut time_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 6./16.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 5./16.) as i32);
	time_screen_lbl.set_frame(enums::FrameType::FlatBox);
	time_screen_lbl.set_color(enums::Color::Black);
	time_screen_lbl.set_label_color(enums::Color::White);
	time_screen_lbl.hide();
	return time_screen_lbl;
}

fn round_time_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_time_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 28./48.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./32.) as i32)
		.with_label("Round time:");
	round_time_lbl.set_frame(enums::FrameType::FlatBox);
	return round_time_lbl;
}

fn round_time_input(screen_width: f64, screen_height: f64) -> input::IntInput {
	let mut round_time_input = input::IntInput::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 10./16.) as i32)
		.with_size((screen_width * 5./81.) as i32, (screen_height * 1./32.) as i32);
	round_time_input.set_value("120");
	return round_time_input;
}

fn round_time_seconds_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut round_time_seconds_lbl = frame::Frame::default()
		.with_pos((screen_width * 41./81.) as i32, (screen_height * 10./16.) as i32)
		.with_size((screen_width * 4./81.) as i32, (screen_height * 1./32.) as i32)
		.with_label("seconds");
	round_time_seconds_lbl.set_frame(enums::FrameType::FlatBox);
	return round_time_seconds_lbl;
}

fn rest_time_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut rest_time_lbl = frame::Frame::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 34./48.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./32.) as i32)
		.with_label("Rest time:");
	rest_time_lbl.set_frame(enums::FrameType::FlatBox);
	return rest_time_lbl;
}

fn rest_time_input(screen_width: f64, screen_height: f64) -> input::IntInput {
	let mut rest_time_input = input::IntInput::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 12./16.) as i32)
		.with_size((screen_width * 5./81.) as i32, (screen_height * 1./32.) as i32);
	rest_time_input.set_value("60");
	return rest_time_input;
}

fn rest_time_seconds_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut rest_time_seconds_lbl = frame::Frame::default()
		.with_pos((screen_width * 41./81.) as i32, (screen_height * 12./16.) as i32)
		.with_size((screen_width * 4./81.) as i32, (screen_height * 1./32.) as i32)
		.with_label("seconds");
	rest_time_seconds_lbl.set_frame(enums::FrameType::FlatBox);
	return rest_time_seconds_lbl;
}

fn new_contest_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let new_contest_btn = button::Button::default()
		.with_pos((screen_width * 4./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("New contest");
	return new_contest_btn;
}

fn si_jak_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut si_jak_btn = button::Button::default()
		.with_pos((screen_width * 1./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("Shi-jak");
	si_jak_btn.hide();
	return si_jak_btn;
}

fn kye_sok_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut kye_sok_btn = button::Button::default()
		.with_pos((screen_width * 1./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("Kye-sok");
	kye_sok_btn.deactivate();
	kye_sok_btn.hide();
	return kye_sok_btn;
}

fn kal_yeo_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut kal_yeo_btn = button::Button::default()
		.with_pos((screen_width * 3./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("Kal-yeo");
	kal_yeo_btn.deactivate();
	kal_yeo_btn.hide();
	return kal_yeo_btn;
}

fn keu_man_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut keu_man_btn = button::Button::default()
		.with_pos((screen_width * 5./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("Keu-man");
	keu_man_btn.deactivate();
	keu_man_btn.hide();
	return keu_man_btn;
}

fn end_contest_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut end_contest_btn = button::Button::default()
		.with_pos((screen_width * 7./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("End contest");
	end_contest_btn.hide();
	return end_contest_btn;
}

fn resume_contest_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut resume_contest_btn = button::Button::default()
		.with_pos((screen_width * 7./9.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label("Resume");
	resume_contest_btn.hide();
	return resume_contest_btn;
}

fn contest_winner_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_winner_lbl = frame::Frame::default()
		.with_pos((screen_width * 9./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 15./24.) as i32, (screen_width * 3./24.) as i32);
	contest_winner_lbl.set_frame(enums::FrameType::FlatBox);
	contest_winner_lbl.set_label_color(enums::Color::White);
	return contest_winner_lbl;
}

fn contest_winner_screen_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut contest_winner_screen_lbl = frame::Frame::default()
		.with_pos((screen_width * 1./3.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width *  1./3.) as i32, (screen_height * 4./16.) as i32);
	contest_winner_screen_lbl.set_frame(enums::FrameType::FlatBox);
	contest_winner_screen_lbl.set_label_color(enums::Color::White);
	return contest_winner_screen_lbl;
}

fn cheong_plus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 2./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_jirugi_btn.set_color(enums::Color::Blue);
	cheong_plus_jirugi_btn.hide();
	return cheong_plus_jirugi_btn;
}

fn cheong_minus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 2./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_jirugi_btn.set_color(enums::Color::Blue);
	cheong_minus_jirugi_btn.hide();
	return cheong_minus_jirugi_btn;
}

fn cheong_plus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 5./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momtong_btn.set_color(enums::Color::Blue);
	cheong_plus_momtong_btn.hide();
	return cheong_plus_momtong_btn;
}

fn cheong_minus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 5./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momtong_btn.set_color(enums::Color::Blue);
	cheong_minus_momtong_btn.hide();
	return cheong_minus_momtong_btn;
}

fn cheong_plus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 8./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momdollyeo_momtong_btn.set_color(enums::Color::Blue);
	cheong_plus_momdollyeo_momtong_btn.hide();
	return cheong_plus_momdollyeo_momtong_btn;
}

fn cheong_minus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 8./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momdollyeo_momtong_btn.set_color(enums::Color::Blue);
	cheong_minus_momdollyeo_momtong_btn.hide();
	return cheong_minus_momdollyeo_momtong_btn;
}

fn cheong_plus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 11./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_olgul_btn.set_color(enums::Color::Blue);
	cheong_plus_olgul_btn.hide();
	return cheong_plus_olgul_btn;
}

fn cheong_minus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 11./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_olgul_btn.set_color(enums::Color::Blue);
	cheong_minus_olgul_btn.hide();
	return cheong_minus_olgul_btn;
}

fn cheong_plus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 14./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momdollyeo_olgul_btn.set_color(enums::Color::Blue);
	cheong_plus_momdollyeo_olgul_btn.hide();
	return cheong_plus_momdollyeo_olgul_btn;
}

fn cheong_minus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 14./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momdollyeo_olgul_btn.set_color(enums::Color::Blue);
	cheong_minus_momdollyeo_olgul_btn.hide();
	return cheong_minus_momdollyeo_olgul_btn;
}

fn cheong_plus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("+ Gam-jeon");
	cheong_plus_gam_jeon_btn.set_color(enums::Color::Blue);
	cheong_plus_gam_jeon_btn.set_label_color(enums::Color::White);
	cheong_plus_gam_jeon_btn.hide();
	return cheong_plus_gam_jeon_btn;
}

fn cheong_minus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("- Gam-jeon");
	cheong_minus_gam_jeon_btn.set_color(enums::Color::Blue);
	cheong_minus_gam_jeon_btn.set_label_color(enums::Color::White);
	cheong_minus_gam_jeon_btn.hide();
	return cheong_minus_gam_jeon_btn;
}

fn plus_1_second_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut plus_1_second_btn = button::Button::default()
		.with_pos((screen_width * 23./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32)
		.with_label("+ 1s");
	plus_1_second_btn.hide();
	return plus_1_second_btn;
}

fn minus_1_second_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut minus_1_second_btn = button::Button::default()
		.with_pos((screen_width * 23./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32)
		.with_label("- 1s");
	minus_1_second_btn.hide();
	return minus_1_second_btn;
}

fn hong_plus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("+ Gam-jeon");
	hong_plus_gam_jeon_btn.set_color(enums::Color::Red);
	hong_plus_gam_jeon_btn.set_label_color(enums::Color::White);
	hong_plus_gam_jeon_btn.hide();
	return hong_plus_gam_jeon_btn;
}

fn hong_minus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("- Gam-jeon");
	hong_minus_gam_jeon_btn.set_color(enums::Color::Red);
	hong_minus_gam_jeon_btn.set_label_color(enums::Color::White);
	hong_minus_gam_jeon_btn.hide();
	return hong_minus_gam_jeon_btn;
}

fn hong_plus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 32./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_jirugi_btn.set_color(enums::Color::Red);
	hong_plus_jirugi_btn.hide();
	return hong_plus_jirugi_btn;
}

fn hong_minus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 32./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_jirugi_btn.set_color(enums::Color::Red);
	hong_minus_jirugi_btn.hide();
	return hong_minus_jirugi_btn;
}

fn hong_plus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 35./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momtong_btn.set_color(enums::Color::Red);
	hong_plus_momtong_btn.hide();
	return hong_plus_momtong_btn;
}

fn hong_minus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 35./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momtong_btn.set_color(enums::Color::Red);
	hong_minus_momtong_btn.hide();
	return hong_minus_momtong_btn;
}

fn hong_plus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 38./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momdollyeo_momtong_btn.set_color(enums::Color::Red);
	hong_plus_momdollyeo_momtong_btn.hide();
	return hong_plus_momdollyeo_momtong_btn;
}

fn hong_minus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 38./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momdollyeo_momtong_btn.set_color(enums::Color::Red);
	hong_minus_momdollyeo_momtong_btn.hide();
	return hong_minus_momdollyeo_momtong_btn;
}

fn hong_plus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 41./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_olgul_btn.set_color(enums::Color::Red);
	hong_plus_olgul_btn.hide();
	return hong_plus_olgul_btn;
}

fn hong_minus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 41./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_olgul_btn.set_color(enums::Color::Red);
	hong_minus_olgul_btn.hide();
	return hong_minus_olgul_btn;
}

fn hong_plus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 44./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momdollyeo_olgul_btn.set_color(enums::Color::Red);
	hong_plus_momdollyeo_olgul_btn.hide();
	return hong_plus_momdollyeo_olgul_btn;
}

fn hong_minus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 44./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momdollyeo_olgul_btn.set_color(enums::Color::Red);
	hong_minus_momdollyeo_olgul_btn.hide();
	return hong_minus_momdollyeo_olgul_btn;
}

fn superiority_decision_lbl(screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut superiority_decision_lbl = frame::Frame::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 7./24.) as i32, (screen_width * 1./24.) as i32);
	superiority_decision_lbl.set_frame(enums::FrameType::FlatBox);
	superiority_decision_lbl.hide();
	return superiority_decision_lbl;
}

fn cheong_superiority_decision_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_superiority_decision_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("Cheong");
	cheong_superiority_decision_btn.set_color(enums::Color::Blue);
	cheong_superiority_decision_btn.set_label_color(enums::Color::White);
	cheong_superiority_decision_btn.hide();
	return cheong_superiority_decision_btn;
}

fn hong_superiority_decision_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_superiority_decision_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label("Hong");
	hong_superiority_decision_btn.set_color(enums::Color::Red);
	hong_superiority_decision_btn.set_label_color(enums::Color::White);
	hong_superiority_decision_btn.hide();
	return hong_superiority_decision_btn;
}

fn copyright(_screen_width: f64, screen_height: f64) -> frame::Frame {
	let mut copyright = frame::Frame::default()
		.with_pos(0, screen_height as i32)
		.with_align(enums::Align::RightBottom)
		.with_label("Copyright (C) 2022 Iker Ruiz de Infante Gonzalez <contact@@irzinfante.eu>");
	copyright.set_label_size(10);
	return copyright;
}