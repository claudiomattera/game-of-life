// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A game engine library implementing Conway's Game of Life based on WASM-4
//! engine

#![no_std]

pub mod game;
pub mod graphics;
pub mod input;
pub mod interface;
pub mod time;
pub mod wasm4;

#[cfg(target_family = "wasm")]
use core::arch::wasm32;

#[cfg(target_family = "wasm")]
use core::panic::PanicInfo;

#[cfg(target_family = "wasm")]
use crate::wasm4::trace;

#[cfg(target_family = "wasm")]
#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    trace("panic error");

    #[cfg(debug_assertions)]
    if let Some(cause) = _panic_info.payload().downcast_ref::<&str>() {
        trace(cause);
    }

    unsafe { wasm32::unreachable() }
}
