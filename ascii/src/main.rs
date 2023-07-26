use std::env;
use std::io::{stdin, stdout, Read, Write};

use termion::raw::IntoRawMode;
use termion::{color, style};

fn main() {
    let queries: Vec<String> = env::args().skip(1).collect();
    if queries.is_empty() {
        print_ascii_table(vec![]);
        interactive_queries();
    } else if queries.iter().all(|query| query.parse::<u8>().is_ok()) {
        ascii_to_chars(
            queries
                .iter()
                .map(|query| query.parse::<u8>())
                .map_while(Result::ok)
                .collect(),
        );
    } else {
        chars_to_ascii(queries);
    }
}

fn ascii_to_chars(queries: Vec<u8>) {
    for code in &queries {
        print!("{:>3} ", *code);
    }
    println!();
    for code in &queries {
        let default_output = &format!("{}", *code as char);
        let output = try_ascii_name(code).unwrap_or(default_output).trim_start();
        print!("{:>3} ", output);
    }
    println!();
}

fn chars_to_ascii(queries: Vec<String>) {
    for query in queries {
        for char in query.chars() {
            print!("{:>3} ", char)
        }
        println!();
        for char in query.chars() {
            print!("{:>3} ", char as u8)
        }
        println!();
    }
}

fn interactive_queries() {
    println!("Go ahead, hit some keys! (double-tap q to exit)");
    let mut screen = stdout().into_raw_mode().unwrap();
    screen.flush().unwrap();
    let mut prev_char: u8 = 0;
    for code in stdin().bytes().map_while(Result::ok) {
        let quitter = b"q\x03\x04\x1b".contains(&code);
        if quitter && prev_char == code {
            break;
        }
        let default_output = &format!("{}", code as char);
        let output = try_ascii_name(&code).unwrap_or(default_output).trim_start();
        write!(screen, "{:08b} {:>3} {:>3}", code, code, output).unwrap();
        if code == b'q' {
            write!(screen, " (hit again to quit)").unwrap();
        }
        write!(screen, "  {}", row_comment(code)).unwrap();
        writeln!(screen, "\r").unwrap();
        prev_char = code;
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}

fn print_ascii_table(highlight: Vec<u8>) {
    println!("  HI   0   0 0 0 1 1 1 1");
    println!(" \\     0   0 1 1 0 0 1 1");
    println!("LO     0   1 0 1 0 1 0 1");
    for row in 0..16 {
        print!("{:04b} ", row);
        for col in 0..8 {
            let code: u8 = 16 * col + row;
            let default_output = &format!("{}", code as char);
            let output = try_ascii_name(&code).unwrap_or(default_output);
            if highlight.contains(&code) {
                print!("{}{}{}", style::Bold, output, style::Reset);
            } else {
                print!("{}", output);
            }
            let last_col = col == 7;
            if !last_col {
                print!(" ");
            }
        }
        println!("  {}", row_comment(row))
    }
    println!(
        "{}{}{}",
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        style::Reset
    );
}

fn row_comment(row: u8) -> &'static str {
    match row {
        0 => "// Null (blank space on the card, no holes punched)",
        1 => "// Start of (message) Heading",
        2 => "// Start of (message body) Text",
        3 => "// End of (message body) Text",
        4 => "// End of Transmission",
        5 => "// Enquiry (\"ping?\")",
        6 => "// Acknowledge (\"pong!\")",
        7 => "// Bell, Alert",
        8 => "// Backspace",
        9 => "// (Horizontal) tab, go to next tab stop",
        10 => "// Line Feed, end-of-line",
        11 => "// (Vertical) tab, go to next line tab stop",
        12 => "// Form Feed (\"page break\", \"clear screen\")",
        13 => "// Carriage Return, go to beginning of line",
        14 => "// Shift Out (switch to alternative character set)",
        15 => "// Shift In (switch back to normal character set)",
        16 => "// Data Link Escape, interpret following octets differently",
        17 => "// Device Control One, (\"XON\", \"transmit on\", \"start remote paper-tape-reader\")",
        18 => "// Device Control Two, (\"activate device\")",
        19 => "// Device Control Three, (\"XOFF\", \"transmit off\", \"start remote paper-tape-reader\")",
        20 => "// Device Control Four, (\"deactivate device\")",
        21 => "// Negative Acknowledge, (\"last block in error\", \"not ready yet\")",
        22 => "// Synchronous Idle, used for character framing in Telex et al.",
        23 => "// End of Transmission Block (\"End of paragraph\")",
        24 => "// Cancel (\"discard previous data\")",
        25 => "// End of Medium (paper or magnetic tape, sometimes interpreted as indent line)",
        26 => "// Substitute (\"invalid chars detected\", sometimes \"end-of-file\")",
        27 => "// Escape (exit from something, or \"begin special command sequence\")",
        28 => "// File Separator (File = collection of groups)",
        29 => "// Group Separator (Group = collection of Records)",
        30 => "// Record Separator (Record = collection of Units)",
        31 => "// Unit Separator (Unit = single data item)",
        32 => "// Space",
        127 => "// Delete (punched a \"all-holes\" over the character)",
        _ => "",
    }
}

fn try_ascii_name(code: &u8) -> Option<&str> {
    Some(match code {
        0 => "NUL",
        1 => "SOH",
        2 => "STX",
        3 => "ETX",
        4 => "EOT",
        5 => "ENQ",
        6 => "ACK",
        7 => "BEL",
        8 => " BS",
        9 => "TAB",
        10 => " LF",
        11 => " VT",
        12 => " FF",
        13 => " CR",
        14 => " SO",
        15 => " SI",
        16 => "DLE",
        17 => "DC1",
        18 => "DC2",
        19 => "DC3",
        20 => "DC4",
        21 => "NAK",
        22 => "SYN",
        23 => "ETB",
        24 => "CAN",
        25 => " EM",
        26 => "SUB",
        27 => "ESC",
        28 => " FS",
        29 => " GS",
        30 => " RS",
        31 => " US",
        32 => "␣",
        127 => "DEL",
        _ => return None,
    })
}
