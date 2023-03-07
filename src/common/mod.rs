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

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::Scoreboard;

pub mod labels;

pub fn scale_size(size: f64, width: f64, height: f64) -> i32 {
	(size * (width.powf(2.) + height.powf(2.)).sqrt() / 2000.) as i32
}

#[derive(Hash)]
struct GamJeons {
    cheong_gam_jeon_count: u8,
    hong_gam_jeon_count: u8
}

fn hash<T: Hash>(obj: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

impl Scoreboard {
	pub fn gam_jeons_hash(&self) -> u64{
		hash(GamJeons {
			cheong_gam_jeon_count: self.cheong_gam_jeon_count,
		    hong_gam_jeon_count: self.hong_gam_jeon_count
		})
	}
}