// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! User interface controls

use crate::graphics::{
    draw_horizontal_line, draw_line, draw_point, draw_rect, draw_text, draw_vertical_line,
    set_drawing_colors,
};

mod button;
pub use button::Button;

mod checkbox;
pub use checkbox::CheckBox;

mod label;
pub use label::Label;

mod spinbox;
pub use spinbox::SpinBox;

pub fn draw_title() {
    let first_line = "CONWAY'S";
    let second_line = "GAME OF LIFE";
    let first_line_x = (160 - 8 * first_line.len() as i32) / 2;
    let second_line_x = (160 - 8 * second_line.len() as i32) / 2;

    set_drawing_colors(3);
    draw_text(first_line, first_line_x + 1, 3 + 1);
    draw_text(second_line, second_line_x + 1, 14 + 1);

    set_drawing_colors(2);
    draw_text(first_line, first_line_x, 3);
    draw_text(second_line, second_line_x, 14);
}

pub fn draw_frame(offset: (i32, i32)) {
    let width = 160;
    let height = 80;

    set_drawing_colors(1);
    draw_rect(offset.0 + 4, offset.1 + 4, width - 8, height - 8);

    set_drawing_colors(2);
    draw_rect(offset.0 + 2, offset.1, width - 4, 4);
    draw_rect(offset.0, offset.1 + 2, 4, height - 4);

    set_drawing_colors(3);
    draw_rect(offset.0 + 2, offset.1 + height as i32 - 3, width - 4, 2);
    draw_rect(offset.0 + width as i32 - 3, offset.1 + 2, 2, height - 4);

    set_drawing_colors(4);
    draw_horizontal_line(offset.0, offset.1, width);
    draw_horizontal_line(offset.0 + 3, offset.1 + 3, width - 6);
    draw_vertical_line(offset.0, offset.1, height);
    draw_vertical_line(offset.0 + 3, offset.1 + 3, height - 6);

    draw_horizontal_line(offset.0, offset.1 + height as i32 - 1, width);
    draw_horizontal_line(offset.0 + 3, offset.1 + height as i32 - 1 - 3, width - 6);
    draw_vertical_line(offset.0 + width as i32 - 1, offset.1, height);
    draw_vertical_line(offset.0 + width as i32 - 1 - 3, offset.1 + 3, height - 6);

    draw_line(offset.0, offset.1, offset.0 + 3, offset.1 + 3);
    draw_line(
        offset.0 + width as i32 - 1,
        offset.1,
        offset.0 + width as i32 - 1 - 3,
        offset.1 + 3,
    );
    draw_line(
        offset.0,
        offset.1 + height as i32 - 1,
        offset.0 + 3,
        offset.1 + height as i32 - 1 - 3,
    );
    draw_line(
        offset.0 + width as i32 - 1,
        offset.1 + height as i32 - 1,
        offset.0 + width as i32 - 1 - 3,
        offset.1 + height as i32 - 1 - 3,
    );

    for i in 1..((width as i32) / 2 - 1) {
        draw_point(offset.0 + 1 + 2 * i, offset.1 + height as i32 - 3);
        draw_point(offset.0 + 2 * i, offset.1 + height as i32 - 2);
    }
    for i in 1..((height as i32) / 2 - 1) {
        draw_point(offset.0 + width as i32 - 3, offset.1 + 1 + 2 * i);
        draw_point(offset.0 + width as i32 - 2, offset.1 + 2 * i);
    }
}
