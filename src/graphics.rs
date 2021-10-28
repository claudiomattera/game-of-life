// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Graphics primitives and subsystems

mod drawcolors;
pub use drawcolors::set_drawing_colors;

mod palette;
pub use palette::Palette;

use crate::wasm4::{hline, line, rect, text, vline};

/// Draw a rectangle using the current colours
pub fn draw_rect(x: i32, y: i32, width: u32, height: u32) {
    rect(x, y, width, height)
}

/// Draw a horizontal line using the current colours
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    line(x1, y1, x2, y2)
}

/// Draw a horizontal line using the current colours
pub fn draw_horizontal_line(x: i32, y: i32, width: u32) {
    hline(x, y, width)
}

/// Draw a vertical using the current colours
pub fn draw_vertical_line(x: i32, y: i32, height: u32) {
    vline(x, y, height)
}

/// Draw a vertical using the current colours
pub fn draw_point(x: i32, y: i32) {
    vline(x, y, 1)
}

/// Draw text using the current colours
pub fn draw_text<T>(string: T, x: i32, y: i32)
where
    T: AsRef<str>,
{
    text(string, x, y)
}
