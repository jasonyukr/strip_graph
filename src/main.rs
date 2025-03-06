use std::io::{self, BufRead, Write};
use std::process;

const STRIP_COLOR: bool = true;

enum State {
    Normal,
    Escape,
    Csi,
}

fn write_stripped(stdout: &mut dyn Write, graph_end: &mut bool, ch: char) -> io::Result<()>  {
    if *graph_end == false {
        *graph_end = ch.is_digit(16)
    }
    if *graph_end == true {
        write!(stdout, "{}", ch)?;
    }
    Ok(())
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_data;
        match line {
            Ok(data) => line_data = data,
            Err(_) => continue
        }
        let mut state = State::Normal;
        let mut graph_end = false;
        for c in line_data.chars() {
            let mut res = Ok(());
            match &state {
                State::Normal => {
                    if c == 0x1B as char { // ESC
                        state = State::Escape;
                        if !STRIP_COLOR {
                            res = write!(stdout, "{}", c);
                        }
                    } else {
                        res = write_stripped(&mut stdout, &mut graph_end, c);
                    }
                },
                State::Escape => {
                    if !STRIP_COLOR {
                        res = write!(stdout, "{}", c);
                    }
                    if c == 0x5B as char { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    if !STRIP_COLOR {
                        res = write!(stdout, "{}", c);
                    }
                    if c >= 0x40 as char && c < 0x80 as char {
                        state = State::Normal;
                    }
                },
            }
            if let Err(_) = res {
                process::exit(1);
            }
        }
        if let Err(_) = writeln!(stdout) {
            process::exit(1);
        }
    }
}
