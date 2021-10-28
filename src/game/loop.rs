// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Game main loop

use crate::graphics::{
    draw_horizontal_line, draw_rect, draw_vertical_line, set_drawing_colors, Palette,
};
use crate::input::Mouse;
use crate::interface::{draw_frame, draw_title, Button, CheckBox, Label, SpinBox};
use crate::time::Ticker;

use super::World;

static mut WORLD: World = World::new();
static mut PAUSED: bool = false;
static mut SPEED: u8 = 1;
static mut BUTTON_COOLDOWN: u8 = 0;

const MAX_BUTTON_COOLDOWN: u8 = 6;

#[no_mangle]
fn start() {
    Palette::IceCream.set();

    let world = unsafe { &mut WORLD };

    world.set_cell(8, 6, true);
    world.set_cell(9, 7, true);
    world.set_cell(7, 8, true);
    world.set_cell(8, 8, true);
    world.set_cell(9, 8, true);
}

#[no_mangle]
fn update() {
    let world = unsafe { &mut WORLD };
    let paused = unsafe { &mut PAUSED };
    let speed = unsafe { &mut SPEED };
    let button_cooldown = unsafe { &mut BUTTON_COOLDOWN };

    let offset = (0, 30);

    draw_with_mouse(&Mouse, offset, world);

    let period = 60 / *speed;
    if !*paused && Ticker.get() % period == 0 {
        propagate_life(world);
    }

    let controls_y = 118;
    let (mut paused_checkbox, label, minus_button, spinbox, plus_button) =
        create_user_controls(controls_y, *paused, *speed);

    operate_user_controls(
        &mut paused_checkbox,
        &minus_button,
        &plus_button,
        paused,
        speed,
        button_cooldown,
    );

    draw(
        offset,
        world,
        &paused_checkbox,
        &label,
        &minus_button,
        &spinbox,
        &plus_button,
    );

    Mouse.update();
    Ticker.update();
}

fn draw_mouse_pointer() {
    let (x, y) = Mouse.coordinates();
    set_drawing_colors(4);
    draw_vertical_line(x as i32, y as i32 - 1, 3);
    draw_horizontal_line(x as i32 - 1, y as i32, 3);
}

fn draw_with_mouse(mouse: &Mouse, offset: (i32, i32), world: &mut World) {
    if mouse.left_pressed() || mouse.right_pressed() {
        let (x, y) = mouse.coordinates();
        let x = (x - offset.0 as i16) / 4 - 1;
        let y = (y - offset.1 as i16) / 4 - 1;
        if x >= 0 && x < (World::WIDTH as i16) && y >= 0 && y < (World::HEIGHT as i16) {
            if mouse.left_pressed() {
                world.set_cell(x, y, true);
            } else {
                world.set_cell(x, y, false);
            }
        }
    }
}

fn propagate_life(world: &mut World) {
    let mut new_world = World::new();

    for x in 0..(World::WIDTH as i16) {
        for y in 0..(World::HEIGHT as i16) {
            let alive = world.get_cell(x, y);
            let count = world.count_live_neighbours(x, y);

            if alive {
                new_world.set_cell(x, y, (2..4).contains(&count));
            } else {
                new_world.set_cell(x, y, count == 3);
            }
        }
    }

    world.replace(&new_world);
}

fn create_user_controls(
    controls_y: i32,
    paused: bool,
    speed: u8,
) -> (CheckBox, Label, Button, SpinBox, Button) {
    let paused_button_y = 15;
    let row_y = 0;

    let checkbox_width = 4 + 8 * 10;
    let checkbox_x = (160 - checkbox_width as i32) / 2;
    let mut paused_checkbox = CheckBox::new(
        "Pause",
        checkbox_x,
        controls_y + paused_button_y,
        checkbox_width,
        4 + 8,
    );
    paused_checkbox.set_pressed(paused);

    let row_width = (4 + 8 * 7) + (4 + 8) + (4 + 8) + (4 + 8);
    let row_x = (160 - row_width as i32) / 2;

    let label_x = row_x;
    let minus_button_x = row_x + 4 + 8 * 7;
    let spinbox_x = row_x + 4 * 2 + 8 * (7 + 1);
    let plus_button_x = row_x + 4 * 3 + 8 * (7 + 1 + 1);

    let label = Label::new("Speed:", label_x, controls_y + row_y, 4 + 8 * 7, 4 + 8);
    let minus_button = Button::new("-", minus_button_x, controls_y + row_y, 4 + 8, 4 + 8);
    let spinbox = SpinBox::new(speed, spinbox_x, controls_y + row_y, 4 + 8, 4 + 8);
    let plus_button = Button::new("+", plus_button_x, controls_y + row_y, 4 + 8, 4 + 8);

    (paused_checkbox, label, minus_button, spinbox, plus_button)
}

fn operate_user_controls(
    paused_checkbox: &mut CheckBox,
    minus_button: &Button,
    plus_button: &Button,
    paused: &mut bool,
    speed: &mut u8,
    button_cooldown: &mut u8,
) {
    paused_checkbox.toggle(&Mouse);

    if *button_cooldown == 0 && plus_button.is_clicked(&Mouse) && *speed < 6 {
        *speed += 1;
        *button_cooldown = MAX_BUTTON_COOLDOWN;
    }

    if *button_cooldown == 0 && minus_button.is_clicked(&Mouse) && *speed > 1 {
        *speed -= 1;
        *button_cooldown = MAX_BUTTON_COOLDOWN;
    }

    *paused = paused_checkbox.is_pressed();

    if *button_cooldown > 0 {
        *button_cooldown -= 1;
    }
}

fn draw(
    offset: (i32, i32),
    world: &World,
    paused_checkbox: &CheckBox,
    label: &Label,
    minus_button: &Button,
    spinbox: &SpinBox,
    plus_button: &Button,
) {
    draw_title();
    draw_frame(offset);
    draw_world(world, offset);

    paused_checkbox.draw(&Mouse);
    label.draw();
    minus_button.draw(&Mouse);
    spinbox.draw();
    plus_button.draw(&Mouse);

    draw_mouse_pointer();
}

fn draw_world(world: &World, offset: (i32, i32)) {
    set_drawing_colors(2);
    for x in 0..(World::WIDTH as i16) {
        for y in 0..(World::HEIGHT as i16) {
            if world.get_cell(x, y) {
                draw_rect(
                    offset.0 + 4 + (x * 4) as i32,
                    offset.1 + 4 + (y * 4) as i32,
                    4,
                    4,
                );
            }
        }
    }
}
