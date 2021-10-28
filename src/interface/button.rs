// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{draw_rect, draw_text, set_drawing_colors};
use crate::input::Mouse;

#[derive(Debug)]
pub struct Button {
    string: &'static str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Button {
    pub const fn new(string: &'static str, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            string,
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_clicked(&self, mouse: &Mouse) -> bool {
        self.is_mouse_over(mouse) && mouse.left_pressed()
    }

    pub fn draw(&self, mouse: &Mouse) {
        if self.is_mouse_over(mouse) && mouse.left_pressed() {
            self.draw_pressed();
        } else {
            self.draw_raised(self.is_mouse_over(mouse));
        }
    }

    fn draw_raised(&self, hovered: bool) {
        set_drawing_colors(0x41);
        draw_rect(self.x, self.y, self.width, self.height);
        if hovered {
            set_drawing_colors(0x32);
        } else {
            set_drawing_colors(0x31);
        }
        draw_rect(self.x - 1, self.y - 1, self.width, self.height);

        let text_length = self.string.len() as i32;
        let padding = (self.width as i32 - text_length * 8 - 4) / 2;
        set_drawing_colors(0x03);
        draw_text(&self.string, self.x + padding + 1, self.y + 1);
    }

    fn draw_pressed(&self) {
        set_drawing_colors(0x41);
        draw_rect(self.x, self.y, self.width, self.height);
        set_drawing_colors(0x32);
        draw_rect(self.x, self.y, self.width, self.height);

        let text_length = self.string.len() as i32;
        let padding = (self.width as i32 - text_length * 8 - 4) / 2;
        set_drawing_colors(0x03);
        draw_text(&self.string, self.x + padding + 2, self.y + 2);
    }

    fn is_mouse_over(&self, mouse: &Mouse) -> bool {
        let (mx, my) = mouse.coordinates();
        let x = self.x as i16;
        let y = self.y as i16;
        let width = self.width as i16;
        let height = self.height as i16;
        mx > x && mx < (x + width) && my > y && my < (y + height)
    }
}
