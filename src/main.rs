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

use std::{thread, time::Duration};
use fltk::{prelude::*, app, window, image};
use tkd_scoreboard::*;
use crate::{
	constants::{
		TKD_SCOREBOARD_SCOREBOARD, ICON, TKD_SCOREBOARD_SCREEN,
		PLUS_JIRUGI, PLUS_MOMTONG, PLUS_MOMDOLLYEO_MOMTONG, PLUS_OLGUL, PLUS_MOMDOLLYEO_OLGUL,
		MINUS_JIRUGI, MINUS_MOMTONG, MINUS_MOMDOLLYEO_MOMTONG, MINUS_OLGUL, MINUS_MOMDOLLYEO_OLGUL
	},
	enums::{State, Winner},
	common::{*, labels::*},
	settings::{labels::*, inputs::*, buttons::*},
	display::{*, labels::*},
	screen::{*, labels::*},
	controls::buttons::*,
	medical_timeout::{labels::*, buttons::*},
	superiority_decision::{labels::*, buttons::*},
	end_contest::{labels::*, buttons::*},
	contest_winner::{labels::*, buttons::*},
	data::groups::*
};

fn main() {
    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();
    
    let mut main_win = window::Window::default()
    	.with_size(screen_width as i32, screen_height as i32)
    	.with_label(TKD_SCOREBOARD_SCOREBOARD);
    main_win.make_resizable(true);
    main_win.set_icon(Some(image::SvgImage::from_data(ICON).unwrap()));
	
	let settings = Settings {
		contest_number_lbl: contest_number_lbl(screen_width, screen_height),
		contest_number_input: contest_number_input(screen_width, screen_height),
		
		round_time_lbl: round_time_lbl(screen_width, screen_height),
		round_time_input: round_time_input(screen_width, screen_height),
		round_time_seconds_lbl: round_time_seconds_lbl(screen_width, screen_height),
		
		rest_time_lbl: rest_time_lbl(screen_width, screen_height),
		rest_time_input: rest_time_input(screen_width, screen_height),
		rest_time_seconds_lbl: rest_time_seconds_lbl(screen_width, screen_height),
		
		new_contest_btn: new_contest_btn(screen_width, screen_height),
		
		vertical_separator_lbl: vertical_separator_lbl(screen_width, screen_height),
		
		data_scroll: scroll_group(screen_width, screen_height),
		export_data_btn: export_data_btn(screen_width, screen_height),
		delete_data_btn: delete_data_btn(screen_width, screen_height)
	};
	
	let display = Display {
		cheong_score_lbl: cheong_score_display_lbl(screen_width, screen_height),
		cheong_gam_jeon_count_lbl: cheong_gam_jeon_count_display_lbl(screen_width, screen_height),
		cheong_seung_lbl: cheong_seung_display_lbl(),
		
		hong_score_lbl: hong_score_display_lbl(screen_width, screen_height),
		hong_gam_jeon_count_lbl: hong_gam_jeon_count_display_lbl(screen_width, screen_height),
		hong_seung_lbl: hong_seung_display_lbl(),
		
		round_rest_lbl: round_rest_display_lbl(screen_width, screen_height),
		round_number_lbl: round_number_display_lbl(screen_width, screen_height),
		
		time_lbl: time_display_lbl(screen_width, screen_height),
		
		kye_shi_lbl: kye_shi_display_lbl(screen_width, screen_height),
		kye_shi_time_lbl: kye_shi_time_display_lbl(screen_width, screen_height),
		
		superiority_decision_lbl: superiority_decision_display_lbl(screen_width, screen_height),
		end_contest_lbl: end_contest_display_lbl(screen_width, screen_height),
		
		contest_winner_lbl: contest_winner_display_lbl(screen_width, screen_height),
		
		contest_number_lbl: contest_number(screen_width, screen_height)
	};
	
	let controls = Controls {
		shi_jak_btn: shi_jak_btn(screen_width, screen_height),
		kye_sok_btn: kye_sok_btn(screen_width, screen_height),
		kal_yeo_btn: kal_yeo_btn(screen_width, screen_height),
		kye_shi_btn: kye_shi_btn(screen_width, screen_height),
		kye_shi_cancel_btn: kye_shi_cancel_btn(screen_width, screen_height),
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
		
		plus_one_second_btn: plus_one_second_btn(screen_width, screen_height),
		minus_one_second_btn: minus_one_second_btn(screen_width, screen_height),
		
		cheong_superiority_decision_btn: cheong_superiority_decision_btn(screen_width, screen_height),
		hong_superiority_decision_btn: hong_superiority_decision_btn(screen_width, screen_height),
		
		cheong_end_contest_btn: cheong_end_contest_btn(screen_width, screen_height),
		hong_end_contest_btn: hong_end_contest_btn(screen_width, screen_height),
		
		clear_scoreboard_btn: clear_scoreboard_btn(screen_width, screen_height)
	};
	
	copyright(screen_width, screen_height);
	
    main_win.end();
    main_win.show();
    
    let mut screen_win = window::Window::default()
    	.with_size(screen_width as i32, screen_height as i32)
    	.with_label(TKD_SCOREBOARD_SCREEN);
    screen_win.make_resizable(true);
    screen_win.set_icon(Some(image::SvgImage::from_data(ICON).unwrap()));
    
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
		
		kye_shi_time_lbl: kye_shi_time_screen_lbl(screen_width, screen_height),
		
		contest_winner_lbl: contest_winner_screen_lbl(screen_width, screen_height),
		
		contest_number_lbl: contest_number(screen_width, screen_height)
	};
	
	copyright(screen_width, screen_height);
	
    screen_win.end();
    screen_win.show();
    
    let data = Data {
		contest_id: String::new(),
		
		contest_number: String::new(),
		round_time: String::new(),
		rest_time: String::new(),
		
		cheong_score: [(); 3].map(|_| String::from("-")),
		cheong_gamjeon: [(); 3].map(|_| String::from("-")),
		hong_score: [(); 3].map(|_| String::from("-")),
		hong_gamjeon: [(); 3].map(|_| String::from("-")),
		round_winner: [(); 3].map(|_| String::from("-")),
		
		contest_winner: Winner::None.to_string()
	};
    
    let scoreboard = ScoreboardHandle::new(
		Scoreboard {
			settings,
			display,
			screen,
			controls,
			
			data,
			
			state: State::None,
			previous_state: State::None,
			
			round_time: 0f32,
			rest_time: 0f32,
			
			cheong_score: 0,
			cheong_gam_jeon_count: 0,
			
			hong_score: 0,
			hong_gam_jeon_count: 0,
			
			gam_jeons_hash: 0,
			
			round_number: 0,
			round_winner: [Winner::None; 3],
			is_after_reevaluation: false,
			
			time: 0f32,
			
			blink_time: 0f32,
			blink: false,
			
			kye_shi_time: 0f32,
			
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
    
    let scoreboard_resize = scoreboard.clone();
    main_win.resize_callback(move |_win, _x, _y, w, h| {
		let (x, y, d) = seung_display_lbl_dimensions(w as f64, h as f64);
		
		scoreboard_resize.with_lock(|share| {
			share.settings.contest_number_lbl.set_label_size(scale_size(25., w as f64, h as f64));
			share.settings.contest_number_input.set_text_size(scale_size(17., w as f64, h as f64));
			share.settings.round_time_lbl.set_label_size(scale_size(25., w as f64, h as f64));
			share.settings.round_time_input.set_text_size(scale_size(17., w as f64, h as f64));
			share.settings.round_time_seconds_lbl.set_label_size(scale_size(17., w as f64, h as f64));
			share.settings.rest_time_lbl.set_label_size(scale_size(25., w as f64, h as f64));
			share.settings.rest_time_input.set_text_size(scale_size(17., w as f64, h as f64));
			share.settings.rest_time_seconds_lbl.set_label_size(scale_size(17., w as f64, h as f64));
			share.settings.new_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.settings.data_scroll.hscrollbar().deactivate();
			share.settings.export_data_btn.set_label_size(scale_size(20., w as f64, h as f64));
			share.settings.delete_data_btn.set_label_size(scale_size(20., w as f64, h as f64));
			
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
			
			share.display.time_lbl.set_label_size(scale_size(215., w as f64, h as f64));
			
			share.display.kye_shi_lbl.set_label_size(scale_size(40., w as f64, h as f64));
			share.display.kye_shi_time_lbl.set_label_size(scale_size(125., w as f64, h as f64));
			share.controls.kye_shi_cancel_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.display.superiority_decision_lbl.set_label_size(scale_size(40., w as f64, h as f64));
			share.controls.cheong_superiority_decision_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.hong_superiority_decision_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.display.end_contest_lbl.set_label_size(scale_size(40., w as f64, h as f64));
			share.controls.cheong_end_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.hong_end_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.resume_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.display.contest_winner_lbl.set_label_size(scale_size(120., w as f64, h as f64));
			share.controls.clear_scoreboard_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
			share.controls.shi_jak_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.kye_sok_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.kal_yeo_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.kye_shi_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.keu_man_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.end_contest_btn.set_label_size(scale_size(25., w as f64, h as f64));
			
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
			
			share.controls.plus_one_second_btn.set_label_size(scale_size(25., w as f64, h as f64));
			share.controls.minus_one_second_btn.set_label_size(scale_size(25., w as f64, h as f64));
		});
	});
    main_win.resize(main_win.x(), main_win.y(), main_win.width(), main_win.height());
	
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
			
			share.screen.time_lbl.set_label_size(scale_size(250., w as f64, h as f64));
			
			share.screen.kye_shi_time_lbl.set_label_size(scale_size(400., w as f64, h as f64));
			
			share.screen.contest_winner_lbl.set_label_size(scale_size(100., w as f64, h as f64));
		});
	});
    screen_win.resize(screen_win.x(), screen_win.y(), screen_win.width(), screen_win.height());
    
	scoreboard.clone().with_lock(|share| {
		let scoreboard_new_contest_btn = scoreboard.clone();
		share.settings.new_contest_btn.set_callback(move |_new_contest_btn| {
			scoreboard_new_contest_btn.with_lock(|share| {
				share.round_time = share.settings.round_time_input.value().parse::<f32>().unwrap();
				share.rest_time = share.settings.rest_time_input.value().parse::<f32>().unwrap();
				
				share.show_contest_number();
				
				share.hide_settings();
				share.hide_data();
				share.change_state(State::CallContestants);
				
				share.initialize_round();
				share.initialize_contest_data();
				
				share.show_display();
				share.update_display();
				
				share.show_screen();
				share.update_screen();
				
				share.update_controls();
			});
		});
		
		let scoreboard_export_data_btn = scoreboard.clone();
		share.settings.export_data_btn.set_callback(move |_export_data_btn| {
			scoreboard_export_data_btn.with_lock(|share| {
				share.export_data();
			});
		});
		
		let scoreboard_delete_data_btn = scoreboard.clone();
		share.settings.delete_data_btn.set_callback(move |_delete_data_btn| {
			scoreboard_delete_data_btn.with_lock(|share| {
				share.delete_data();
				share.update_data();
			});
		});
		
		let scoreboard_shi_jak_btn = scoreboard.clone();
		share.controls.shi_jak_btn.set_callback(move |_shi_jak_btn| {
			scoreboard_shi_jak_btn.with_lock(|share| {
				share.change_state(State::Round);
				share.update_controls();
			});
		});
		
		let scoreboard_kye_sok_btn = scoreboard.clone();
		share.controls.kye_sok_btn.set_callback(move |_kye_sok_btn| {
			scoreboard_kye_sok_btn.with_lock(|share| {
				share.change_state(State::Round);
				share.update_controls();
			});
		});
		
		let scoreboard_kal_yeo_btn = scoreboard.clone();
		share.controls.kal_yeo_btn.set_callback(move |_kal_yeo_btn| {
			scoreboard_kal_yeo_btn.with_lock(|share| {
				share.change_state(State::Timeout);
				share.update_controls();
			});
		});
		
		let scoreboard_kye_shi_btn = scoreboard.clone();
		share.controls.kye_shi_btn.set_callback(move |_kye_shi_btn| {
			scoreboard_kye_shi_btn.with_lock(|share| {
				share.change_state(State::MedicalTimeout);
				share.update_controls();
				
				share.show_medical_timeout();
				share.update_medical_timeout();
			});
		});
		
		let scoreboard_keu_man_btn = scoreboard.clone();
		share.controls.keu_man_btn.set_callback(move |_keu_man_btn| {
			scoreboard_keu_man_btn.with_lock(|share| {
				share.decide_round_winner();
			});
		});
		
		let scoreboard_end_contest_btn = scoreboard.clone();
		share.controls.end_contest_btn.set_callback(move |_end_contest_btn| {
			scoreboard_end_contest_btn.with_lock(|share| {
				match share.state {
					State::SuperiorityDecision => share.hide_superiority_decision(),
					_ => ()
				}
				share.change_state(State::EndContest);
				share.update_controls();
				share.show_end_contest();
			});
		});
		
		let scoreboard_cheong_plus_jirugi_btn = scoreboard.clone();
		share.controls.cheong_plus_jirugi_btn.set_callback(move |_cheong_plus_jirugi_btn| {
			scoreboard_cheong_plus_jirugi_btn.with_lock(|share| {
				share.cheong_score += 1;
				share.cheong_jirugi_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_jirugi_btn = scoreboard.clone();
		share.controls.cheong_minus_jirugi_btn.set_callback(move |_cheong_minus_jirugi_btn| {
			scoreboard_cheong_minus_jirugi_btn.with_lock(|share| {
				share.cheong_score -= 1;
				share.cheong_jirugi_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_plus_momtong_btn = scoreboard.clone();
		share.controls.cheong_plus_momtong_btn.set_callback(move |_cheong_plus_momtong_btn| {
			scoreboard_cheong_plus_momtong_btn.with_lock(|share| {
				share.cheong_score += 2;
				share.cheong_momtong_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_momtong_btn = scoreboard.clone();
		share.controls.cheong_minus_momtong_btn.set_callback(move |_cheong_minus_momtong_btn| {
			scoreboard_cheong_minus_momtong_btn.with_lock(|share| {
				share.cheong_score -= 2;
				share.cheong_momtong_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_plus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.cheong_plus_momdollyeo_momtong_btn.set_callback(move |_cheong_plus_momdollyeo_momtong_btn| {
			scoreboard_cheong_plus_momdollyeo_momtong_btn.with_lock(|share| {
				share.cheong_score += 4;
				share.cheong_momdollyeo_momtong_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.cheong_minus_momdollyeo_momtong_btn.set_callback(move |_cheong_minus_momdollyeo_momtong_btn| {
			scoreboard_cheong_minus_momdollyeo_momtong_btn.with_lock(|share| {
				share.cheong_score -= 4;
				share.cheong_momdollyeo_momtong_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_plus_olgul_btn = scoreboard.clone();
		share.controls.cheong_plus_olgul_btn.set_callback(move |_cheong_plus_olgul_btn| {
			scoreboard_cheong_plus_olgul_btn.with_lock(|share| {
				share.cheong_score += 3;
				share.cheong_olgul_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_olgul_btn = scoreboard.clone();
		share.controls.cheong_minus_olgul_btn.set_callback(move |_cheong_minus_olgul_btn| {
			scoreboard_cheong_minus_olgul_btn.with_lock(|share| {
				share.cheong_score -= 3;
				share.cheong_olgul_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_plus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.cheong_plus_momdollyeo_olgul_btn.set_callback(move |_cheong_plus_momdollyeo_olgul_btn| {
			scoreboard_cheong_plus_momdollyeo_olgul_btn.with_lock(|share| {
				share.cheong_score += 5;
				share.cheong_momdollyeo_olgul_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.cheong_minus_momdollyeo_olgul_btn.set_callback(move |_cheong_minus_momdollyeo_olgul_btn| {
			scoreboard_cheong_minus_momdollyeo_olgul_btn.with_lock(|share| {
				share.cheong_score -= 5;
				share.cheong_momdollyeo_olgul_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_plus_gam_jeon_btn = scoreboard.clone();
		share.controls.cheong_plus_gam_jeon_btn.set_callback(move |_cheong_plus_gam_jeon_btn| {
			scoreboard_cheong_plus_gam_jeon_btn.with_lock(|share| {
				share.cheong_gam_jeon_count += 1;
				share.hong_score += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_minus_gam_jeon_btn = scoreboard.clone();
		share.controls.cheong_minus_gam_jeon_btn.set_callback(move |_cheong_minus_gam_jeon_btn| {
			scoreboard_cheong_minus_gam_jeon_btn.with_lock(|share| {
				share.cheong_gam_jeon_count -= 1;
				share.hong_score -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_gam_jeon_btn = scoreboard.clone();
		share.controls.hong_plus_gam_jeon_btn.set_callback(move |_hong_plus_gam_jeon_btn| {
			scoreboard_hong_plus_gam_jeon_btn.with_lock(|share| {
				share.hong_gam_jeon_count += 1;
				share.cheong_score += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_gam_jeon_btn = scoreboard.clone();
		share.controls.hong_minus_gam_jeon_btn.set_callback(move |_hong_minus_gam_jeon_btn| {
			scoreboard_hong_minus_gam_jeon_btn.with_lock(|share| {
				share.hong_gam_jeon_count -= 1;
				share.cheong_score -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_jirugi_btn = scoreboard.clone();
		share.controls.hong_plus_jirugi_btn.set_callback(move |_hong_plus_jirugi_btn| {
			scoreboard_hong_plus_jirugi_btn.with_lock(|share| {
				share.hong_score += 1;
				share.hong_jirugi_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_jirugi_btn = scoreboard.clone();
		share.controls.hong_minus_jirugi_btn.set_callback(move |_hong_minus_jirugi_btn| {
			scoreboard_hong_minus_jirugi_btn.with_lock(|share| {
				share.hong_score -= 1;
				share.hong_jirugi_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_momtong_btn = scoreboard.clone();
		share.controls.hong_plus_momtong_btn.set_callback(move |_hong_plus_momtong_btn| {
			scoreboard_hong_plus_momtong_btn.with_lock(|share| {
				share.hong_score += 2;
				share.hong_momtong_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_momtong_btn = scoreboard.clone();
		share.controls.hong_minus_momtong_btn.set_callback(move |_hong_minus_momtong_btn| {
			scoreboard_hong_minus_momtong_btn.with_lock(|share| {
				share.hong_score -= 2;
				share.hong_momtong_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.hong_plus_momdollyeo_momtong_btn.set_callback(move |_hong_plus_momdollyeo_momtong_btn| {
			scoreboard_hong_plus_momdollyeo_momtong_btn.with_lock(|share| {
				share.hong_score += 4;
				share.hong_momdollyeo_momtong_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_momdollyeo_momtong_btn = scoreboard.clone();
		share.controls.hong_minus_momdollyeo_momtong_btn.set_callback(move |_hong_minus_momdollyeo_momtong_btn| {
			scoreboard_hong_minus_momdollyeo_momtong_btn.with_lock(|share| {
				share.hong_score -= 4;
				share.hong_momdollyeo_momtong_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_olgul_btn = scoreboard.clone();
		share.controls.hong_plus_olgul_btn.set_callback(move |_hong_plus_olgul_btn| {
			scoreboard_hong_plus_olgul_btn.with_lock(|share| {
				share.hong_score += 3;
				share.hong_olgul_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_olgul_btn = scoreboard.clone();
		share.controls.hong_minus_olgul_btn.set_callback(move |_hong_minus_olgul_btn| {
			scoreboard_hong_minus_olgul_btn.with_lock(|share| {
				share.hong_score -= 3;
				share.hong_olgul_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_plus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.hong_plus_momdollyeo_olgul_btn.set_callback(move |_hong_plus_momdollyeo_olgul_btn| {
			scoreboard_hong_plus_momdollyeo_olgul_btn.with_lock(|share| {
				share.hong_score += 5;
				share.hong_momdollyeo_olgul_count += 1;
				share.update_controls();
			});
		});
		
		let scoreboard_hong_minus_momdollyeo_olgul_btn = scoreboard.clone();
		share.controls.hong_minus_momdollyeo_olgul_btn.set_callback(move |_hong_minus_momdollyeo_olgul_btn| {
			scoreboard_hong_minus_momdollyeo_olgul_btn.with_lock(|share| {
				share.hong_score -= 5;
				share.hong_momdollyeo_olgul_count -= 1;
				share.update_controls();
			});
		});
		
		let scoreboard_plus_one_second_btn = scoreboard.clone();
		share.controls.plus_one_second_btn.set_callback(move |_plus_one_second_btn| {
			scoreboard_plus_one_second_btn.with_lock(|share| {
				share.variate_time_round(1f32);
				share.update_controls();
			});
		});
		
		let scoreboard_minus_one_second_btn = scoreboard.clone();
		share.controls.minus_one_second_btn.set_callback(move |_minus_one_second_btn| {
			scoreboard_minus_one_second_btn.with_lock(|share| {
				share.variate_time_round(-1f32);
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_superiority_decision_btn = scoreboard.clone();
		share.controls.cheong_superiority_decision_btn.set_callback(move |_cheong_superiority_decision_btn| {
			scoreboard_cheong_superiority_decision_btn.with_lock(|share| {
				share.hide_superiority_decision();
				share.round_winner[(share.round_number - 1) as usize] = Winner::Cheong;
				share.is_contest_winner();
			});
		});
		
		let scoreboard_hong_superiority_decision_btn = scoreboard.clone();
		share.controls.hong_superiority_decision_btn.set_callback(move |_hong_superiority_decision_btn| {
			scoreboard_hong_superiority_decision_btn.with_lock(|share| {
				share.hide_superiority_decision();
				share.round_winner[(share.round_number - 1) as usize] = Winner::Hong;
				share.is_contest_winner();
			});
		});
		
		let scoreboard_kye_shi_cancel_btn = scoreboard.clone();
		share.controls.kye_shi_cancel_btn.set_callback(move |_kye_shi_cancel_btn| {
			scoreboard_kye_shi_cancel_btn.with_lock(|share| {
				share.hide_medical_timeout();
				share.change_state(State::Timeout);
				share.update_controls();
			});
		});
		
		let scoreboard_cheong_end_contest_btn = scoreboard.clone();
		share.controls.cheong_end_contest_btn.set_callback(move |_cheong_end_contest_btn| {
			scoreboard_cheong_end_contest_btn.with_lock(|share| {
				share.hide_end_contest();
				share.change_state(State::ContestWinner);
				share.show_contest_winner(Winner::Cheong);
				share.save_round_data();
				share.save_contest_winner_data(Winner::Cheong);
				share.write_data();
			});
		});
		
		let scoreboard_hong_end_contest_btn = scoreboard.clone();
		share.controls.hong_end_contest_btn.set_callback(move |_hong_end_contest_btn| {
			scoreboard_hong_end_contest_btn.with_lock(|share| {
				share.hide_end_contest();
				share.change_state(State::ContestWinner);
				share.show_contest_winner(Winner::Hong);
				share.save_round_data();
				share.save_contest_winner_data(Winner::Hong);
				share.write_data();
			});
		});
		
		let scoreboard_resume_contest_btn = scoreboard.clone();
		share.controls.resume_contest_btn.set_callback(move |_resume_contest_btn| {
			scoreboard_resume_contest_btn.with_lock(|share| {
				share.hide_end_contest();
				share.change_state(share.previous_state);
				share.update_controls();
				match share.state {
					State::SuperiorityDecision => share.show_superiority_decision(),
					_ => ()
				}
			});
		});
		
		let scoreboard_clear_scoreboard_btn = scoreboard.clone();
		share.controls.clear_scoreboard_btn.set_callback(move |_clear_scoreboard_btn| {
			scoreboard_clear_scoreboard_btn.with_lock(|share| {
				share.hide_contest_winner();
				share.hide_display();
				share.hide_screen();
				share.hide_contest_number();
				
				share.clear_scoreboard();
				
				share.change_state(State::Settings);
				share.update_controls();
				share.show_settings();
				
				share.increment_contest_number_settings();
				share.update_data();
			});
		});
	});
	
    while app.wait() {
		if !main_win.shown() || !screen_win.shown() {
			panic!("Exit!");
		}
		thread::sleep(Duration::from_millis(100));
		scoreboard.with_lock(|share| {
			match share.state {
				State::None => {
					share.change_state(State::Settings);
					share.show_settings();
					share.set_default_settings();
					share.update_data();
				},
				State::Settings => {
					share.validate_settings();
				},
				State::CallContestants => (),
				State::Round => {
					share.variate_time_round(-0.1);
					if share.is_keu_man_conditions() {
						share.change_state(State::KeumanCondition);
						share.update_controls();
					}
					share.update_display();
					share.update_screen();
				},
				State::Timeout => {
					share.blink_time += 0.1;
					if share.blink_time >= 0.5 {
						share.blink = !share.blink;
						share.blink_time -= 0.5;
					}
					if share.is_keu_man_conditions() {
						share.change_state(State::KeumanCondition);
						share.update_controls();
					}
					share.update_display();
					share.update_screen();
				},
				State::MedicalTimeout => {
					share.variate_kye_shi_time(-0.1);
					share.update_medical_timeout();
				},
				State::KeumanCondition => {
					if !share.is_keu_man_conditions() {
						share.change_state(State::Timeout);
						share.update_controls();
					}
					share.update_display();
					share.update_screen();
				},
				State::SuperiorityDecision => (),
				State::RestFirstPart => {
					share.variate_time_rest(-0.1);
					share.is_after_5_seconds();
					share.update_display();
					share.update_screen();
				},
				State::RestSecondPart => {
					share.variate_time_rest(-0.1);
					share.time_ends();
					share.update_display();
					share.update_screen();
				},
				State::EndContest => (),
				State::ContestWinner => ()
			}
		});
	}
}