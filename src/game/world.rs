// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! World structure

#[derive(Debug)]
pub struct World([bool; World::WIDTH * World::HEIGHT]);

impl World {
    pub const HORIZONTAL_DIVIDER: usize = 4;
    pub const VERTICAL_DIVIDER: usize = 8;
    pub const WIDTH: usize = 160 / Self::HORIZONTAL_DIVIDER - 2;
    pub const HEIGHT: usize = 160 / Self::VERTICAL_DIVIDER - 2;

    pub const fn new() -> Self {
        World([false; Self::WIDTH * Self::HEIGHT])
    }

    pub fn replace(&mut self, other: &Self) {
        self.0.copy_from_slice(&other.0);
    }

    pub fn flip_cell(&mut self, x: i16, y: i16) {
        self.set_cell(x, y, !self.get_cell(x, y));
    }

    pub fn set_cell(&mut self, x: i16, y: i16, value: bool) {
        let x = x as usize;
        let y = y as usize;
        self.0[y * Self::WIDTH + x] = value;
    }

    pub fn get_cell(&self, x: i16, y: i16) -> bool {
        let x = x as usize;
        let y = y as usize;
        self.0[y * Self::WIDTH + x]
    }

    pub fn count_live_neighbours(&self, x: i16, y: i16) -> usize {
        IntoIterator::into_iter(self.neighbours(x, y))
            .filter(|(x, y)| self.get_cell(*x, *y))
            .count()
    }

    fn neighbours(&self, x: i16, y: i16) -> [(i16, i16); 8] {
        let width = Self::WIDTH as i16;
        let height = Self::HEIGHT as i16;
        [
            ((x + width - 1) % width, (y + height - 1) % height),
            ((x + width) % width, (y + height - 1) % height),
            ((x + width + 1) % width, (y + height - 1) % height),
            ((x + width - 1) % width, (y + height) % height),
            ((x + width + 1) % width, (y + height) % height),
            ((x + width - 1) % width, (y + height + 1) % height),
            ((x + width) % width, (y + height + 1) % height),
            ((x + width + 1) % width, (y + height + 1) % height),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_in_the_middle() {
        let world = World::new();
        let neighbours: [(i16, i16); 8] = world.neighbours(4, 5);
        let expected: [(i16, i16); 8] = [
            (3, 4),
            (4, 4),
            (5, 4),
            (3, 5),
            (5, 5),
            (3, 6),
            (4, 6),
            (5, 6),
        ];

        assert_eq!(neighbours, expected);
    }

    #[test]
    fn neighbours_in_the_top() {
        let world = World::new();
        let neighbours: [(i16, i16); 8] = world.neighbours(4, 0);
        let expected: [(i16, i16); 8] = [
            (3, (World::HEIGHT as i16) - 1),
            (4, (World::HEIGHT as i16) - 1),
            (5, (World::HEIGHT as i16) - 1),
            (3, 0),
            (5, 0),
            (3, 1),
            (4, 1),
            (5, 1),
        ];

        assert_eq!(neighbours, expected);
    }

    #[test]
    fn neighbours_in_the_bottom() {
        let world = World::new();
        let neighbours: [(i16, i16); 8] = world.neighbours(4, (World::HEIGHT as i16) - 1);
        let expected: [(i16, i16); 8] = [
            (3, (World::HEIGHT as i16) - 2),
            (4, (World::HEIGHT as i16) - 2),
            (5, (World::HEIGHT as i16) - 2),
            (3, (World::HEIGHT as i16) - 1),
            (5, (World::HEIGHT as i16) - 1),
            (3, 0),
            (4, 0),
            (5, 0),
        ];

        assert_eq!(neighbours, expected);
    }

    #[test]
    fn neighbours_in_the_left() {
        let world = World::new();
        let neighbours: [(i16, i16); 8] = world.neighbours(0, 5);
        let expected: [(i16, i16); 8] = [
            ((World::WIDTH as i16) - 1, 4),
            (0, 4),
            (1, 4),
            ((World::WIDTH as i16) - 1, 5),
            (1, 5),
            ((World::WIDTH as i16) - 1, 6),
            (0, 6),
            (1, 6),
        ];

        assert_eq!(neighbours, expected);
    }

    #[test]
    fn neighbours_in_the_right() {
        let world = World::new();
        let neighbours: [(i16, i16); 8] = world.neighbours((World::WIDTH as i16) - 1, 5);
        let expected: [(i16, i16); 8] = [
            ((World::WIDTH as i16) - 2, 4),
            ((World::WIDTH as i16) - 1, 4),
            (0, 4),
            ((World::WIDTH as i16) - 2, 5),
            (0, 5),
            ((World::WIDTH as i16) - 2, 6),
            ((World::WIDTH as i16) - 1, 6),
            (0, 6),
        ];

        assert_eq!(neighbours, expected);
    }

    #[test]
    fn count_live_neighbours_in_the_middle() {
        let mut world = World::new();
        world.set_cell(3, 4, true);
        world.set_cell(3, 5, true);
        world.set_cell(5, 4, true);
        world.set_cell(5, 5, true);

        let neighbours_count = world.count_live_neighbours(4, 5);
        let expected = 4;

        assert_eq!(neighbours_count, expected);
    }

    #[test]
    fn check_star() {
        let mut world = World::new();
        world.set_cell(8, 7, true);
        world.set_cell(8, 8, true);
        world.set_cell(8, 9, true);

        assert_eq!(world.count_live_neighbours(7, 7), 2);
        assert_eq!(world.count_live_neighbours(7, 8), 3);
        assert_eq!(world.count_live_neighbours(7, 9), 2);

        assert_eq!(world.count_live_neighbours(8, 7), 1);
        assert_eq!(world.count_live_neighbours(8, 8), 2);
        assert_eq!(world.count_live_neighbours(8, 9), 1);

        assert_eq!(world.count_live_neighbours(9, 7), 2);
        assert_eq!(world.count_live_neighbours(9, 8), 3);
        assert_eq!(world.count_live_neighbours(9, 9), 2);
    }
}
