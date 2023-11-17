use crate::{bitarr::BitArray2D, screen_to_world, Mouse, UserMode, WORLD_SIZE};
use rand::Rng;

pub fn user_input_sand(sand: &mut BitArray2D, mouse: &Mouse, mode: &UserMode) {
    if mouse.left_button_pressed {
        let (row, col) = mouse.screen_pos;
        let (x, y) = screen_to_world(row, col);

        match mode {
            UserMode::Add => sand.set(x, y, true),
            UserMode::Subtract => sand.set(x, y, false),
        }
    }
}

pub fn update_sand<T: Rng>(sand: &BitArray2D, rng: &mut T) -> BitArray2D {
    let mut next_sand = BitArray2D::new(WORLD_SIZE, WORLD_SIZE);

    for (x, y) in sand.iter_true() {
        if y == 0 {
            next_sand.set(x, y, true);
            continue;
        }

        let bottom = sand.get(x, y - 1);

        if let Some(false) = bottom {
            next_sand.set(x, y - 1, true);
            continue;
        }

        let bottom_left = sand.get(x - 1, y - 1);
        let bottom_right = sand.get(x + 1, y - 1);

        if x == 0 {
            match bottom_right {
                Some(false) => next_sand.set(x + 1, y - 1, true),
                _ => next_sand.set(x, y, true),
            }
            continue;
        }

        if x == WORLD_SIZE - 1 {
            match bottom_left {
                Some(false) => next_sand.set(x - 1, y - 1, true),
                _ => next_sand.set(x, y, true),
            }
            continue;
        }

        match (bottom_left, bottom_right) {
            (Some(false), Some(false)) => {
                if rng.gen() {
                    next_sand.set(x - 1, y - 1, true);
                } else {
                    next_sand.set(x + 1, y - 1, true);
                }
            }
            (Some(false), Some(true)) => next_sand.set(x - 1, y - 1, true),
            (Some(true), Some(false)) => next_sand.set(x + 1, y - 1, true),
            (Some(false), None) => next_sand.set(x - 1, y - 1, true),
            (None, Some(false)) => next_sand.set(x + 1, y - 1, true),
            _ => next_sand.set(x, y, true),
        }
    }

    next_sand
}
