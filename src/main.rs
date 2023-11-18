use std::time::Duration;

use crate::{
    bitarr::BitArray2D,
    sand::{update_sand, user_input_sand},
    timer::Timer,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod bitarr;
mod sand;
mod timer;

const WORLD_SIZE: usize = 120;
const WINDOW_SIZE: usize = 1024;

const TARGET_FRAME_RATE: f64 = 60.0;

const CLEAR_COLOR: [u8; 4] = [0, 0, 0, 255];
const SAND_COLOR: [u8; 4] = [225, 225, 150, 255];
const GROUND_COLOR: [u8; 4] = [198, 135, 103, 255];
const SKY_COLOR: [u8; 4] = [125, 200, 225, 255];

fn main() {
    let mut rng = rand::thread_rng();

    let inner_size = LogicalSize::new(WINDOW_SIZE as f64, WINDOW_SIZE as f64);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(inner_size)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut screen = Pixels::new(WORLD_SIZE as u32, WORLD_SIZE as u32, surface_texture).unwrap();

    let mut sand = BitArray2D::new(WORLD_SIZE, WORLD_SIZE);

    let mut mouse = Mouse::default();
    let mut mode = UserMode::Add;

    let mut clock = Timer::new(Duration::from_secs_f64(1.0 / TARGET_FRAME_RATE));

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if clock.check_with_reset() {
                user_input_sand(&mut sand, &mouse, &mode);
                sand = update_sand(&sand, &mut rng);
                window.request_redraw();
            }
        }
        Event::RedrawRequested(_) => {
            clear_frame(&mut screen, &CLEAR_COLOR);
            render_background(&mut screen, &GROUND_COLOR, &SKY_COLOR);
            render_sand(&mut screen, &sand, &SAND_COLOR);

            screen.render().unwrap();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::CursorMoved { position, .. } => {
                mouse.screen_pos = screen
                    .window_pos_to_pixel(position.into())
                    .unwrap_or_else(|pos| screen.clamp_pixel_pos(pos));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                mouse.left_button_pressed =
                    button == MouseButton::Left && state == ElementState::Pressed
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state != ElementState::Pressed {
                    return;
                }

                match input.virtual_keycode {
                    Some(VirtualKeyCode::A) => mode = UserMode::Add,
                    Some(VirtualKeyCode::S) => mode = UserMode::Subtract,
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    });
}

#[derive(Default)]
pub struct Mouse {
    screen_pos: (usize, usize),
    left_button_pressed: bool,
}

pub enum UserMode {
    Add,
    Subtract,
}

fn clear_frame(screen: &mut Pixels, color: &[u8; 4]) {
    for pixel in screen.frame_mut().chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}

fn set_pixel(screen: &mut Pixels, x: usize, y: usize, color: &[u8; 4]) {
    let index = (WORLD_SIZE * y + x) * 4;
    screen.frame_mut()[index..index + 4].copy_from_slice(color);
}

fn render_sand(screen: &mut Pixels, sand: &BitArray2D, color: &[u8; 4]) {
    for (x, y) in sand.iter_true() {
        let (row, col) = world_to_screen(x, y);
        set_pixel(screen, row, col, color);
    }
}

fn lerp(val_1: u8, val_2: u8, factor: f64) -> u8 {
    ((1.0 - factor) * val_1 as f64 + factor * val_2 as f64).round() as u8
}

fn render_background(screen: &mut Pixels, ground_color: &[u8; 4], sky_color: &[u8; 4]) {
    for row in 0..WORLD_SIZE {
        let factor = row as f64 / (WORLD_SIZE - 1) as f64;

        let row_color = [
            lerp(sky_color[0], ground_color[0], factor),
            lerp(sky_color[1], ground_color[1], factor),
            lerp(sky_color[2], ground_color[2], factor),
            255,
        ];

        for col in 0..WORLD_SIZE {
            set_pixel(screen, col, row, &row_color)
        }
    }
}

fn screen_to_world(x: usize, y: usize) -> (usize, usize) {
    (x, WORLD_SIZE - 1 - y)
}

fn world_to_screen(x: usize, y: usize) -> (usize, usize) {
    (x, WORLD_SIZE - 1 - y)
}
