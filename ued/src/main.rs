use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, stdin, stdout, Write};

use clap::Parser;
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::input::TermReadEventsAndRaw;
use termion::raw::IntoRawMode;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    filename: String,
    #[arg(default_value_t)]
    lineno: usize,
}

fn main() -> Result<(), Error> {
    let Args { filename, lineno } = Args::parse();
    let file = File::open(&filename).unwrap();
    let lines = BufReader::new(file).lines();
    let line = match lines.skip(lineno).next() {
        Some(Ok(line)) => line,
        _ =>
            {
                eprintln!("Couldn't get line {} from {}.", lineno, filename);
                std::process::exit(1);
            }
    };

    let mut screen = stdout().into_raw_mode().unwrap();
    let (x, y) = screen.cursor_pos().unwrap();
    write!(screen, "{}", termion::cursor::Goto(0, y)).unwrap();
    write!(screen, "{}", termion::clear::CurrentLine).unwrap();
    writeln!(screen, "{line}\r").unwrap();
    screen.flush().unwrap();
    let mut buf = Vec::<u8>::new();
    for (event, raw) in stdin().events_and_raw().map_while(Result::ok) {
        if let Event::Key(Key::Esc) = event { break; }
        if let Event::Key(Key::Char('\n')) = event { break; }
        buf.extend(&raw);
        screen.write_all(&raw).unwrap();
        screen.flush().unwrap();
    }
    let infile = File::open(&filename).unwrap();
    let mut outfile = OpenOptions::new().write(true).create_new(true).open("/tmp/foobar").unwrap();
    let lines = BufReader::new(infile).lines();
    for (i, line) in lines.map_while(Result::ok).enumerate() {
        if i == lineno {
            outfile.write_all(&buf).unwrap();
        } else {
            write!(outfile, "{}", &line).unwrap();
        }
        write!(outfile, "{}", '\n').unwrap();
    }

    Ok(())
}
