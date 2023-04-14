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

use fltk::{prelude::*, button, enums::Color};
use crate::constants::{SHI_JAK, KYE_SOK, KAL_YEO, KYE_SHI, KEU_MAN, END_CONTEST, PLUS_GAM_JEON, MINUS_GAM_JEON, PLUS_ONE_SECOND, MINUS_ONE_SECOND};

pub fn shi_jak_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut shi_jak_btn = button::Button::default()
		.with_pos((screen_width * 2./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(SHI_JAK);
	shi_jak_btn.deactivate();
	shi_jak_btn.hide();
	return shi_jak_btn;
}

pub fn kye_sok_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut kye_sok_btn = button::Button::default()
		.with_pos((screen_width * 2./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(KYE_SOK);
	kye_sok_btn.deactivate();
	kye_sok_btn.hide();
	return kye_sok_btn;
}

pub fn kal_yeo_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut kal_yeo_btn = button::Button::default()
		.with_pos((screen_width * 7./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(KAL_YEO);
	kal_yeo_btn.deactivate();
	kal_yeo_btn.hide();
	return kal_yeo_btn;
}

pub fn kye_shi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut kye_shi_btn = button::Button::default()
		.with_pos((screen_width * 12./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(KYE_SHI);
	kye_shi_btn.deactivate();
	kye_shi_btn.hide();
	return kye_shi_btn;
}

pub fn keu_man_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut keu_man_btn = button::Button::default()
		.with_pos((screen_width * 17./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(KEU_MAN);
	keu_man_btn.deactivate();
	keu_man_btn.hide();
	return keu_man_btn;
}

pub fn end_contest_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut end_contest_btn = button::Button::default()
		.with_pos((screen_width * 22./27.) as i32, (screen_height * 14./16.) as i32)
		.with_size((screen_width * 1./9.) as i32, (screen_height * 1./16.) as i32)
		.with_label(END_CONTEST);
	end_contest_btn.deactivate();
	end_contest_btn.hide();
	return end_contest_btn;
}

pub fn cheong_plus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 2./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_jirugi_btn.set_color(Color::Blue);
	cheong_plus_jirugi_btn.deactivate();
	cheong_plus_jirugi_btn.hide();
	return cheong_plus_jirugi_btn;
}

pub fn cheong_minus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 2./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_jirugi_btn.set_color(Color::Blue);
	cheong_minus_jirugi_btn.deactivate();
	cheong_minus_jirugi_btn.hide();
	return cheong_minus_jirugi_btn;
}

pub fn cheong_plus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 5./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momtong_btn.set_color(Color::Blue);
	cheong_plus_momtong_btn.deactivate();
	cheong_plus_momtong_btn.hide();
	return cheong_plus_momtong_btn;
}

pub fn cheong_minus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 5./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momtong_btn.set_color(Color::Blue);
	cheong_minus_momtong_btn.deactivate();
	cheong_minus_momtong_btn.hide();
	return cheong_minus_momtong_btn;
}

pub fn cheong_plus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 8./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momdollyeo_momtong_btn.set_color(Color::Blue);
	cheong_plus_momdollyeo_momtong_btn.deactivate();
	cheong_plus_momdollyeo_momtong_btn.hide();
	return cheong_plus_momdollyeo_momtong_btn;
}

pub fn cheong_minus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 8./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momdollyeo_momtong_btn.set_color(Color::Blue);
	cheong_minus_momdollyeo_momtong_btn.deactivate();
	cheong_minus_momdollyeo_momtong_btn.hide();
	return cheong_minus_momdollyeo_momtong_btn;
}

pub fn cheong_plus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 11./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_olgul_btn.set_color(Color::Blue);
	cheong_plus_olgul_btn.deactivate();
	cheong_plus_olgul_btn.hide();
	return cheong_plus_olgul_btn;
}

pub fn cheong_minus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 11./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_olgul_btn.set_color(Color::Blue);
	cheong_minus_olgul_btn.deactivate();
	cheong_minus_olgul_btn.hide();
	return cheong_minus_olgul_btn;
}

pub fn cheong_plus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 14./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_plus_momdollyeo_olgul_btn.set_color(Color::Blue);
	cheong_plus_momdollyeo_olgul_btn.deactivate();
	cheong_plus_momdollyeo_olgul_btn.hide();
	return cheong_plus_momdollyeo_olgul_btn;
}

pub fn cheong_minus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 14./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	cheong_minus_momdollyeo_olgul_btn.set_color(Color::Blue);
	cheong_minus_momdollyeo_olgul_btn.deactivate();
	cheong_minus_momdollyeo_olgul_btn.hide();
	return cheong_minus_momdollyeo_olgul_btn;
}

pub fn cheong_plus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_plus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(PLUS_GAM_JEON);
	cheong_plus_gam_jeon_btn.set_color(Color::Blue);
	cheong_plus_gam_jeon_btn.set_label_color(Color::White);
	cheong_plus_gam_jeon_btn.deactivate();
	cheong_plus_gam_jeon_btn.hide();
	return cheong_plus_gam_jeon_btn;
}

pub fn cheong_minus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut cheong_minus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 17./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(MINUS_GAM_JEON);
	cheong_minus_gam_jeon_btn.set_color(Color::Blue);
	cheong_minus_gam_jeon_btn.set_label_color(Color::White);
	cheong_minus_gam_jeon_btn.deactivate();
	cheong_minus_gam_jeon_btn.hide();
	return cheong_minus_gam_jeon_btn;
}

pub fn hong_plus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(PLUS_GAM_JEON);
	hong_plus_gam_jeon_btn.set_color(Color::Red);
	hong_plus_gam_jeon_btn.set_label_color(Color::White);
	hong_plus_gam_jeon_btn.deactivate();
	hong_plus_gam_jeon_btn.hide();
	return hong_plus_gam_jeon_btn;
}

pub fn hong_minus_gam_jeon_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_gam_jeon_btn = button::Button::default()
		.with_pos((screen_width * 26./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 5./48.) as i32, (screen_width * 1./24.) as i32)
		.with_label(MINUS_GAM_JEON);
	hong_minus_gam_jeon_btn.set_color(Color::Red);
	hong_minus_gam_jeon_btn.set_label_color(Color::White);
	hong_minus_gam_jeon_btn.deactivate();
	hong_minus_gam_jeon_btn.hide();
	return hong_minus_gam_jeon_btn;
}

pub fn hong_plus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 32./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_jirugi_btn.set_color(Color::Red);
	hong_plus_jirugi_btn.deactivate();
	hong_plus_jirugi_btn.hide();
	return hong_plus_jirugi_btn;
}

pub fn hong_minus_jirugi_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_jirugi_btn = button::Button::default()
		.with_pos((screen_width * 32./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_jirugi_btn.set_color(Color::Red);
	hong_minus_jirugi_btn.deactivate();
	hong_minus_jirugi_btn.hide();
	return hong_minus_jirugi_btn;
}

pub fn hong_plus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 35./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momtong_btn.set_color(Color::Red);
	hong_plus_momtong_btn.deactivate();
	hong_plus_momtong_btn.hide();
	return hong_plus_momtong_btn;
}

pub fn hong_minus_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momtong_btn = button::Button::default()
		.with_pos((screen_width * 35./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momtong_btn.set_color(Color::Red);
	hong_minus_momtong_btn.deactivate();
	hong_minus_momtong_btn.hide();
	return hong_minus_momtong_btn;
}

pub fn hong_plus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 38./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momdollyeo_momtong_btn.set_color(Color::Red);
	hong_plus_momdollyeo_momtong_btn.deactivate();
	hong_plus_momdollyeo_momtong_btn.hide();
	return hong_plus_momdollyeo_momtong_btn;
}

pub fn hong_minus_momdollyeo_momtong_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momdollyeo_momtong_btn = button::Button::default()
		.with_pos((screen_width * 38./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momdollyeo_momtong_btn.set_color(Color::Red);
	hong_minus_momdollyeo_momtong_btn.deactivate();
	hong_minus_momdollyeo_momtong_btn.hide();
	return hong_minus_momdollyeo_momtong_btn;
}

pub fn hong_plus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 41./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_olgul_btn.set_color(Color::Red);
	hong_plus_olgul_btn.deactivate();
	hong_plus_olgul_btn.hide();
	return hong_plus_olgul_btn;
}

pub fn hong_minus_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_olgul_btn = button::Button::default()
		.with_pos((screen_width * 41./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_olgul_btn.set_color(Color::Red);
	hong_minus_olgul_btn.deactivate();
	hong_minus_olgul_btn.hide();
	return hong_minus_olgul_btn;
}

pub fn hong_plus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_plus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 44./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_plus_momdollyeo_olgul_btn.set_color(Color::Red);
	hong_plus_momdollyeo_olgul_btn.deactivate();
	hong_plus_momdollyeo_olgul_btn.hide();
	return hong_plus_momdollyeo_olgul_btn;
}

pub fn hong_minus_momdollyeo_olgul_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut hong_minus_momdollyeo_olgul_btn = button::Button::default()
		.with_pos((screen_width * 44./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32);
	hong_minus_momdollyeo_olgul_btn.set_color(Color::Red);
	hong_minus_momdollyeo_olgul_btn.deactivate();
	hong_minus_momdollyeo_olgul_btn.hide();
	return hong_minus_momdollyeo_olgul_btn;
}

pub fn plus_one_second_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut plus_one_second_btn = button::Button::default()
		.with_pos((screen_width * 23./48.) as i32, (screen_height * 9./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32)
		.with_label(PLUS_ONE_SECOND);
	plus_one_second_btn.deactivate();
	plus_one_second_btn.hide();
	return plus_one_second_btn;
}

pub fn minus_one_second_btn(screen_width: f64, screen_height: f64) -> button::Button {
	let mut minus_one_second_btn = button::Button::default()
		.with_pos((screen_width * 23./48.) as i32, (screen_height * 11./16.) as i32)
		.with_size((screen_width * 1./24.) as i32, (screen_width * 1./24.) as i32)
		.with_label(MINUS_ONE_SECOND);
	minus_one_second_btn.deactivate();
	minus_one_second_btn.hide();
	return minus_one_second_btn;
}