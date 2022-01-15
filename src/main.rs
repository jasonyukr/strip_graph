use std::io::{self, BufRead};

const STRIP_COLOR: bool = true;

enum State {
    Normal,
    Escape,
    Csi,
}

fn print_stripped(graph_end: &mut bool, ch: char) {
    if *graph_end == false {
        *graph_end = ch.is_digit(16)
    }
    if *graph_end == true {
        print!("{}", ch);
        return;
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_data;
        match line {
            Ok(data) => line_data = data,
            Err(_) => continue
        }
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
                        print_stripped(&mut graph_end, b as char);
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
