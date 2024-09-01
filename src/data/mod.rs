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

use std::fs;
use directories::ProjectDirs;
use cacache::{list_sync, write_sync, read_sync, remove_sync, index::Metadata};
use chrono::Local;
use fltk::{prelude::*, group, frame, button, dialog, enums::Align, enums::FrameType, enums::Color};
use crate::{
	Data,
	Scoreboard,
	constants::{CSV_HEADER, DATE_FORMAT_STR, DAY_FORMAT_STR},
	enums::Winner
};

pub mod groups;

fn cache_dir() -> Option<String> {
	if let Some(proj_dirs) = ProjectDirs::from("eu", "irzinfante", env!("CARGO_PKG_NAME")) {
    	proj_dirs.cache_dir().to_str().map(str::to_string)
    } else {
		None
	}
}

fn data_row(data_key: &str) -> group::Group {
	let mut group = group::Group::default().with_size(40, 20);
	group.set_frame(FrameType::FlatBox);
	group.end();
	
	let data_label = frame::Frame::default()
		.with_size(1, 20)
		.with_align(Align::Right)
		.with_label(&format!("{}", data_key));
	group.add(&data_label);
	
	let mut data_select = button::CheckButton::default()
		.with_size(1, 20);
	data_select.set_callback(move |data_select| {
		if data_select.is_checked() {
			data_select.parent().unwrap().as_group().unwrap().set_color(Color::FrameDefault.darker());
		} else {
			data_select.parent().unwrap().as_group().unwrap().set_color(Color::FrameDefault);
		}
		data_select.parent().unwrap().as_group().unwrap().redraw();
	});
	group.add(&data_select);
	
	return group;
}

impl Scoreboard {
	pub fn update_data(&mut self) {
		self.hide_data();
		
		let mut entries: Vec<Metadata> = Vec::new();
		match cache_dir() {
			Some(dir) => {
				for entry in list_sync(&dir) {
					match entry {
						Ok(metadata) => {
							match entries.binary_search_by(|probe| probe.time.cmp(&metadata.time)) {
								Err(pos) => entries.insert(pos, metadata),
								Ok(_) => ()
							}
						},
						_ => ()
					}
				}
			},
			_ => ()
		}
		
		self.settings.data_scroll.show();
		self.settings.export_data_btn.show();
		self.settings.delete_data_btn.show();
		if entries.len() > 0 {
			self.settings.export_data_btn.activate();
			self.settings.delete_data_btn.activate();
		}
		
		self.settings.data_scroll.child(0).unwrap().as_group().unwrap().clear();
		self.settings.data_scroll.redraw();
	    for entry in &entries {
			self.settings.data_scroll.child(0).unwrap().as_group().unwrap().add(&data_row(&entry.key));
	    }
	}
	
	pub fn hide_data(&mut self) {
		self.settings.data_scroll.hide();
		self.settings.export_data_btn.hide();
		self.settings.export_data_btn.deactivate();
		self.settings.delete_data_btn.hide();
		self.settings.delete_data_btn.deactivate();
	}
	
	pub fn initialize_contest_data(&mut self) {
		self.data.contest_id = format!("{} - Contest #{}", Local::now().format(DATE_FORMAT_STR), self.settings.contest_number_input.value());
		
		self.data.contest_number = self.settings.contest_number_input.value();
		self.data.round_time = format!("{}:{:02}", (self.round_time/60.).trunc(), (self.round_time%60.).trunc());
		self.data.rest_time = format!("{}:{:02}", (self.rest_time/60.).trunc(), (self.rest_time%60.).trunc());
	}
	
	pub fn save_round_data(&mut self) {
		let round_number = self.round_number as usize;
		self.data.cheong_score[round_number - 1] = self.cheong_score.to_string();
		self.data.cheong_gamjeon[round_number - 1] = self.cheong_gam_jeon_count.to_string();
		self.data.hong_score[round_number - 1] = self.hong_score.to_string();
		self.data.hong_gamjeon[round_number - 1] = self.hong_gam_jeon_count.to_string();
		self.data.round_winner[round_number - 1] = self.round_winner[round_number - 1].to_string();
	}
	
	pub fn save_contest_winner_data(&mut self, winner: Winner) {
		self.data.contest_winner = winner.to_string();
	}
	
	pub fn write_data(&self) {
		match cache_dir() {
			Some(dir) => {
				match write_sync(dir, &self.data.contest_id, self.data.to_csv()) {
					_ => ()
				}
			},
			_ => ()
		}
	}
	
	fn data_csv(&self) -> String {
		let mut csv_file = String::from(CSV_HEADER);
		
		for data_index in 0..self.settings.data_scroll.child(0).unwrap().as_group().unwrap().children() {
			let data_row = self.settings.data_scroll.child(0).unwrap().as_group().unwrap().child(data_index).unwrap().as_group().unwrap();
			unsafe {
				if button::CheckButton::from_widget(data_row.child(1).unwrap()).is_checked() {
					let data_key = data_row.child(0).unwrap().label();
					match cache_dir() {
						Some(dir) => {
							match read_sync(&dir, data_key) {
								Ok(data) => {
									csv_file.push_str(&format!("\n{}", String::from_utf8_lossy(&data)));
								},
								_ => ()
							}
						},
						_ => ()
					}
				}
			}
		}
		
		return csv_file;
	}
	
	pub fn export_data(&mut self) {
		let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseSaveFile);
		dialog.set_title("Export contests data to CSV");
		dialog.set_filter("*.csv");
		dialog.set_preset_file(&format!("{}_tkd-scoreboard.csv", Local::now().format(DAY_FORMAT_STR)));
        dialog.show();
        
        match fs::write(dialog.filename(), self.data_csv()) {
			Ok(_) => self.update_data(),
			_ => ()
		}
	}
	
	pub fn delete_data(&mut self) {
		for data_index in 0..self.settings.data_scroll.child(0).unwrap().as_group().unwrap().children() {
			let data_row = self.settings.data_scroll.child(0).unwrap().as_group().unwrap().child(data_index).unwrap().as_group().unwrap();
			unsafe {
				if button::CheckButton::from_widget(data_row.child(1).unwrap()).is_checked() {
					let data_key = data_row.child(0).unwrap().label();
					match cache_dir() {
						Some(dir) => {
							match remove_sync(dir, data_key) {
								_ => ()
							}
						},
						_ => ()
					}
				}
			}
		}
		self.update_data();
	}
}

impl Data {
	pub fn to_csv(&self) -> String {
		let mut csv_line = String::new();
		
		csv_line.push_str(&self.contest_id);
		csv_line.push_str(&format!(",{}", self.contest_number));
		csv_line.push_str(&format!(",{}", self.round_time));
		csv_line.push_str(&format!(",{}", self.rest_time));
		
		csv_line.push_str(&format!(",{},{},{},{},{}", self.cheong_score[0], self.cheong_gamjeon[0], self.hong_score[0], self.hong_gamjeon[0], self.round_winner[0]));
		csv_line.push_str(&format!(",{},{},{},{},{}", self.cheong_score[1], self.cheong_gamjeon[1], self.hong_score[1], self.hong_gamjeon[1], self.round_winner[1]));
		csv_line.push_str(&format!(",{},{},{},{},{}", self.cheong_score[2], self.cheong_gamjeon[2], self.hong_score[2], self.hong_gamjeon[2], self.round_winner[2]));
		
		csv_line.push_str(&format!(",{}", self.contest_winner));
		
		return csv_line;
	}
}