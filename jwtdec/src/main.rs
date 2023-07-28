use std::env::args;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

struct Jwt(String, String);

fn main() {
    let jwt = match args().nth(1) {
        Some(jwt) => jwt,
        None => {
            eprintln!("usage: jwtdec JTW");
            std::process::exit(1)
        }
    };
    match decode_jwt(&jwt) {
        None => eprintln!("Couldn't parse {jwt} as a JWT"),
        Some(Jwt(header, payload)) => {
            println!("{header}");
            println!("{payload}");
        }
    }
}

fn decode_jwt(jwt: &str) -> Option<Jwt> {
    let split: Vec<_> = jwt.split('.').collect();
    if split.len() != 3 {
        return None;
    }
    let option = split.get(0..=1);
    let [header, payload] = option.unwrap() else {
        panic!("Split has len 3, this should work :(")
    };

    Some(Jwt(part_to_string(header), part_to_string(payload)))
}

fn part_to_string(part: &&str) -> String {
    match URL_SAFE_NO_PAD.decode(part) {
        Ok(out) => String::from_utf8(out).expect("Expected part to be UTF-8"),
        Err(e) => {
            eprintln!("[WARNING] Couldn't decode part: `{part}` ({e})");
            "{}".to_owned()
        }
    }
}

#[test]
fn happy_case() {
    let some_jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSl\
        dUIEJ1aWxkZXIiLCJpYXQiOjAsImV4cCI6ODY0MDAsImF1ZCI6Ind3dy5leGFtcGxlLmNvbSIsIn\
        N1YiI6Img0eHgwckBleGFtcGxlLnJ1In0.8NVFhDQMXdHF4rm5nfGicry4x7AmY94dbsbm7dlYjzQ";
    let Jwt(header, payload) = decode_jwt(some_jwt).unwrap();
    assert_eq!(header, r#"{"typ":"JWT","alg":"HS256"}"#);
    assert_eq!(
        payload,
        r#"{"iss":"Online JWT Builder","iat":0,"exp":86400,"aud":"www.example.com","sub":"h4xx0r@example.ru"}"#
    );
}
