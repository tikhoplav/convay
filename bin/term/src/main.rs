use std::{
    io::{stdout, Write},
    thread,
    time::{Duration},
};
use termion::{clear};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();

    let (width, height) = termion::terminal_size().unwrap();

    let width = width as usize;
    let height = height as usize;
    let mut buf: Vec<u8> = Vec::with_capacity(width * height);

    for i in 0..width*height {
        buf.push(0);
    }

    let (mut j, mut p) = (0, true);
    loop {
        if j == width * height {
            j = 0;
            p = !p;
        }
        buf[j] = if p { 35 } else { 32 };
        j += 1;

        write!(stdout, "{}", clear::All).unwrap();
        stdout.write_all(&buf).unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(17));
    }
}
