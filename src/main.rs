extern crate crab_console;

use std::io::{self, Write};
use std::{thread, time};


fn main() {
    let scenes = ["\\( °~° )\\","|( °~° )|","/( °~° )/","|( °~° )|","\\( °~° )\\"];
    crab_console::console_controls::clear_screen();
    crab_console::console_controls::hide_cursor();
    let mut steps = 1;
    let mut direction = 1;
    loop {
        for i in scenes.iter() {
            crab_console::console_controls::move_cursor(1,steps);
            print!(" {} ",i);
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(200));
            steps = steps + direction;
        }
        direction = direction * -1;
    }
}
