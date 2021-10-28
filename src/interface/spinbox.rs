// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::from_utf8_unchecked;

use crate::graphics::{draw_rect, draw_text, set_drawing_colors};

#[derive(Debug)]
pub struct SpinBox {
    value: u8,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl SpinBox {
    pub const fn new(value: u8, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            value,
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self) {
        set_drawing_colors(0x41);
        draw_rect(self.x, self.y, self.width, self.height);
        set_drawing_colors(0x31);
        draw_rect(self.x - 1, self.y - 1, self.width, self.height);
        set_drawing_colors(0x03);

        if self.value < 10 {
            let c = ('0' as u32) + self.value as u32;
            let cs: [u8; 4] = [
                (c & 0xff) as u8,
                ((c >> 8) & 0xff) as u8,
                ((c >> 16) & 0xff) as u8,
                ((c >> 24) & 0xff) as u8,
            ];
            let s = unsafe { from_utf8_unchecked(&cs) };
            draw_text(s, self.x + 1, self.y + 1);
        }
    }
}
