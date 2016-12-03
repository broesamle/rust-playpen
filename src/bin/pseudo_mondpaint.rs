use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    io::stdout().write(b"START\n").unwrap();
    io::stdout().flush().unwrap();

    for x in 0..100 {
        thread::sleep(Duration::from_millis(20));
        io::stdout().write_fmt(format_args!("Chunk from pseudo_mondpaint: {}\n", x)).unwrap();
        io::stdout().flush().unwrap();
    }
}
