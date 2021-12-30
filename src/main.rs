use std::io::{self, BufRead};

const STRIP_COLOR: bool = true;

enum State {
    Normal,
    Escape,
    Csi,
}

fn write_with_strip(graph_end: &mut bool, b: u8) {
    if *graph_end == false {
        if b >= '0' as u8 && b <= '9' as u8 {
            *graph_end = true;
        } else if b >= 'a' as u8 && b <= 'f' as u8 {
            *graph_end = true;
        }
    }
    if *graph_end == true {
        print!("{}", b as char);
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_data = line.unwrap();
        let mut state = State::Normal;
        let mut graph_end = false;
        for b in line_data.bytes() {
            match &state {
                State::Normal => {
                    if b == 0x1B { // ESC
                        state = State::Escape;
                        if !STRIP_COLOR {
                            print!("{}", b as char);
                        }
                    } else {
                        write_with_strip(&mut graph_end, b);
                    }
                },
                State::Escape => {
                    if !STRIP_COLOR {
                        print!("{}", b as char);
                    }
                    if b == 0x5B { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    if !STRIP_COLOR {
                        print!("{}", b as char);
                    }
                    if b >= 0x40 && b < 0x80 {
                        state = State::Normal;
                    }
                },
            }
        }
        println!();
    }
}
