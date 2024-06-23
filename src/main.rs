mod cellground;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use names::Generator;
use sdl2::rect::{Point};
use cellground::{Cellground, Renderable};
use crate::cellground::Savable;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const CANVAS_WIDTH: i32 = 1800;
    const CANVAS_HEIGHT: i32 = 950;
    let window = video_subsystem.window("Convey's Game of Life", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 100, 255));
    let mut xl = 0;
    let mut yl = 0;
    loop {
        canvas.draw_line(Point::new(xl, 0), Point::new(xl, CANVAS_HEIGHT)).unwrap();
        canvas.draw_line(Point::new(0, yl), Point::new(CANVAS_WIDTH, yl)).unwrap();
        if yl < CANVAS_HEIGHT as i32 {
            xl += 10;
            yl += 10;
            continue;
        }
        if xl < CANVAS_WIDTH {
            xl += 10;
            continue;
        }
        break;
    }
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cellground = Cellground::new(CANVAS_HEIGHT as usize / 10, CANVAS_WIDTH as usize / 10);
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    cellground.set_cell(y as usize / 10, x as usize / 10);
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                     cellground.clear_cells();
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    cellground.next_gen();
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    cellground.next_gen();
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    cellground.save_field(&(Generator::default().next().unwrap()));
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        cellground.render_thing(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
