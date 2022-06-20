#![allow(non_snake_case)] //just for crate name

extern crate termion;

use std::io::{stdin, stdout};
use termion::event::MouseButton;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

const CHARREF: [char; 71] = [
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-',
    '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v',
    'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k',
    'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$', ' ',
];

fn print_board(
    zoom: f64,
    centre: (f64, f64),
    _width: i32,
    _height: i32,
    squeeze: f64,
    max_iter: i32,
    crosshair: bool,
    _x: u16,
    _y: u16,
) {
    let mut buffer: String = String::new();

    let width: f64 = _width as f64 + 1.0;
    let height: f64 = _height as f64 + 1.0;
    if _y <= 1 {
        let mut _x = 0;
        let mut _y = 0;
    };
    if _x <= 1 {
        let mut _x = 0;
        let mut _y = 0;
    };
    //println!("{}", _y);
    for i in 0..(_height + 2) as i32 {
        for j in 0..(_width + 2) as i32 {
            let case: i32 = (i % (_height + 1) == 0
                || if crosshair == true {
                    i % (_height + 1) == (_y - 1) as i32
                } else {
                    false
                }) as i32
                + (j % (_width + 1) == 0
                    || if crosshair == true {
                        j % (_width + 1) == (_x - 1) as i32
                    } else {
                        false
                    }) as i32
                    * 2;

            //(if crosshair == true {}) as i32 +
            //(w) as i32*2

            match case {
                0 => {
                    let mut _x: f64 = j as f64 - (width / 2.0);
                    let mut _y: f64 = i as f64 - (height / 2.0);

                    _x = _x / zoom + centre.0;
                    _y = _y / zoom * squeeze + centre.1; //characters are taller than they are wide; hence the squeeze

                    buffer.push(calculate_char(_x, _y, max_iter))
                }
                1 => buffer.push('-'),
                2 => buffer.push('|'),
                3 => buffer.push('+'),
                _ => {}
            }
        }
    }

    print!("{}", buffer);
}

fn mandelbrot(x: f64, y: f64, max_iter: i32) -> i32 {
    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;
    let mut i: i32 = 0;

    loop {
        let _zx: f64 = f64::powi(zx, 2);
        let _zy: f64 = f64::powi(zy, 2);

        if !(_zx + _zy < 4.0 && i < max_iter) {
            break;
        }
        let tmp: f64 = _zx - _zy + x;
        zy = 2.0 * zx * zy + y;
        zx = tmp;
        i += 1;
    }

    (i as f64 / max_iter as f64 * 70.0) as i32
}

fn calculate_char(x: f64, y: f64, max_iter: i32) -> char {
    return CHARREF[(mandelbrot(x, y, max_iter)).abs() as usize];
}

fn main() {
    let stdin = stdin();
    let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let size = termion::terminal_size().unwrap();
    let (width, height): (i32, i32) = ((size.0 - 2) as i32, (size.1 - 2) as i32);

    let mut zoom: f64 = 1.0;
    let mut centre: (f64, f64) = (0.0, 0.0);
    let mut squeeze: f64 = 1.0;
    let mut max_iter: i32 = 70;
    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);

    for c in stdin.events() {
        let evt: Event = c.unwrap();
        match evt {
            Event::Key(ke) => match ke {
                Key::Char('q') => break,
                Key::Up => {
                    squeeze -= 0.1;
                    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                }
                Key::Down => {
                    squeeze += 0.1;
                    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                }
                Key::Right => {
                    max_iter = max_iter + 10;
                    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                }
                Key::Left => {
                    max_iter = (max_iter - 10).max(70);
                    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                }
                _ => (),
            },
            Event::Mouse(me) => match me {
                MouseEvent::Release(_x, _y) => {
                    let x: f64 = (_x) as f64 - ((width) as f64 / 2.0);
                    let y: f64 = (_y - 1) as f64 - ((height + 1) as f64 / 2.0);
                    centre.0 = centre.0 + x / zoom;
                    centre.1 = centre.1 + y / zoom * squeeze;

                    print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                }
                MouseEvent::Hold(_x, _y) => {
                    print_board(zoom, centre, width, height, squeeze, max_iter, true, _x, _y);
                }
                MouseEvent::Press(b, _x, _y) => match b {
                    MouseButton::Left => {
                        print_board(zoom, centre, width, height, squeeze, max_iter, true, _x, _y);
                    }
                    MouseButton::WheelDown => {
                        zoom = if zoom < 1.0 { 1.0 } else { zoom - zoom * 0.1 };
                        print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                    }
                    MouseButton::WheelUp => {
                        zoom += zoom * 0.1;
                        print_board(zoom, centre, width, height, squeeze, max_iter, false, 0, 0);
                    }
                    _ => (),
                },
            },
            _ => {}
        }
    }
}
