/*
 * Copyright (c):
 * 2025 zephyrj
 * zephyrj@protonmail.com
 *
 * This file is part of zephyrj-automation-tools.
 *
 * zephyrj-automation-tools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * zephyrj-automation-tools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with zephyrj-automation-tools. If not, see <https://www.gnu.org/licenses/>.
 */

pub mod car;
pub mod sandbox;
pub mod validation;

mod types;
mod steam;
mod math_utils;

pub use types::*;

use std::path::PathBuf;

pub const STEAM_GAME_NAME: &str = "Automation";
pub const STEAM_GAME_ID: i64 = 293760;

pub fn is_installed() -> bool {
    get_install_path().is_dir()
}

pub fn get_install_path() -> PathBuf {
    steam::get_game_install_path(STEAM_GAME_NAME)
}

pub const FIRST_AL_RIMA_VERSION_NUM: f32 = 2412240000.0;
