//#![allow(non_snake_case)]

extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use std::io::{stdout, stdin};
use termion::raw::IntoRawMode;
use termion::event::MouseButton;

const CHARREF: [char; 71] = [' ', '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w', 'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j', 'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>', 'i', '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' '];

fn print_board(zoom: f64, centre:(f64, f64), _width: i32, _height: i32, squeeze:f64) {
    let mut buffer:String = String::new();

    let width: f64 = _width as f64 + 1.0;
    let height: f64 = _height as f64 + 1.0;
    
    for i in 0..(_height+2) as i32 {
        for j in 0..(_width+2) as i32 {


            let case: i32 = (i%(_height+1) == 0) as i32 + (j%(_width+1) == 0) as i32*2;

            match case {
                0 => {
                    let mut _x:f64 = j as f64-(width/2.0);
                    let mut _y:f64 = i as f64-(height/2.0);

                    _x = _x/zoom + centre.0;
                    _y = _y/zoom*squeeze + centre.1; //characters are taller than they are wide; hence the squeeze

                    buffer.push(calculate_char(_x, _y))
                },
                1 => buffer.push('-'),
                2 => buffer.push('|'),
                3 => buffer.push('+'),
                _ => {}
            }
        }
    }

    print!("{}", buffer);

}

fn mandelbrot(x: f64, y: f64) -> i32 {

    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;
    let mut i: i32 = 0;

    let max_iter: i32 = 280;

    while zx*zx + zy*zy < 4.0 && i < max_iter {
        let tmp = zx*zx - zy*zy + x;

        zy = 2.0 * zx*zy + y;
        zx = tmp;
        i += 1;
    }

    i/2
}

fn calculate_char(x:f64,y:f64) -> char {

    return CHARREF[((140-mandelbrot(x,y))/2) as usize];
}


fn main() {
    let stdin = stdin();
    let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let size = termion::terminal_size().unwrap();
    let (width, height): (i32, i32) = ((size.0-2) as i32, (size.1-2) as i32);

    
    let mut zoom: f64 = 1.0;
    let mut centre: (f64, f64) = (0.0,0.0);
    let mut squeeze: f64 = 1.0;
    print_board(zoom, centre, width, height, squeeze);
    
    for c in stdin.events() {
        let evt: Event = c.unwrap();
        let step: f64 = 1.0/zoom*width as f64/24.0;
        match evt {
            Event::Key(ke) => {
                match ke {
                    Key::Char('q') => break,
                    Key::Char('-') => {squeeze += 0.1},
                    Key::Char('=') => {squeeze -= 0.1},
                    Key::Down => {centre.1 += step},
                    Key::Up => {centre.1 -= step},
                    Key::Left => {centre.0 -= step},
                    Key::Right => {centre.0 += step},
                    _ => ()
                }
            },
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(b, _x, _y) => {
                        match b {
                            MouseButton::Left => {

                                let x: f64 = _x as f64 - ((width+1) as f64/2.0);
                                let y: f64 = _y as f64 - ((height+1) as f64/2.0);
                                let hyp = 5.0*step*((y*y+x*x).sqrt()/(((width+1) as f64/2.0)*((width+1) as f64/2.0) + ((height+1) as f64/2.0)*((height+1) as f64/2.0)).sqrt());
                                let angle: f64 = y.atan2(x);
                                let new_x: f64 = hyp*angle.cos();
                                let new_y: f64 = hyp*angle.sin();
                                centre.0 = new_x+centre.0;
                                centre.1 = new_y+centre.1;

                            }
                            MouseButton::WheelDown => {
                                zoom = if zoom < 1.0 {1.0} else {zoom - zoom * 0.1};
                            },
                            MouseButton::WheelUp => {zoom += zoom * 0.1},
                            _ => (),
                            }
                        },
                        _ => (),
                    }
                }
                _ => {}
            }
            print_board(zoom, centre, width, height, squeeze);
        }
}
