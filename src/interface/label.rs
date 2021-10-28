// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{draw_text, set_drawing_colors};

#[derive(Debug)]
pub struct Label {
    string: &'static str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Label {
    pub const fn new(string: &'static str, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            string,
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self) {
        set_drawing_colors(0x03);
        draw_text(self.string, self.x + 1, self.y + 1);
    }
}
