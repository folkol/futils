#![feature(ascii_char)]

use std::fs;
use std::time::SystemTime;

use quick_xml::{events::Event, reader::Reader};

enum Tag {
    Title,
    Link,
    PubDate,
    Description,
    Other,
}

#[derive(Debug)]
struct Item {
    title: String,
    description: String,
    pub_date: String,
    link: String,
}

impl Item {
    fn new() -> Item {
        Item {
            title: String::from(""),
            description: String::from(""),
            pub_date: String::from(""),
            link: String::from(""),
        }
    }
}

// https://news.un.org/feed/subscribe/en/news/all/rss.xml
fn main() {
    let xml = get_feed();
    let items = parse_items(&xml);
    for Item { title, description, pub_date, .. } in items.iter().rev() {
        println!();
        println!("[{pub_date}]");
        println!("# {title}");
        println!("{description}");
    }
}

fn parse_items(xml: &String) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut current_item: Item = Item::new();
    let mut current_tag = Tag::Other;
    let mut reading_item = false;
    let mut reader = Reader::from_str(&xml);
    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                current_tag = match e.name().as_ref() {
                    b"item" => {
                        reading_item = true;
                        current_item = Item::new();
                        Tag::Other
                    }
                    b"title" => {
                        Tag::Title
                    }
                    b"description" => {
                        Tag::Description
                    }
                    b"pubDate" => {
                        Tag::PubDate
                    }
                    b"link" => {
                        Tag::Link
                    }
                    _ => {
                        Tag::Other
                    }
                };
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"item" {
                    reading_item = false;
                    items.push(current_item);
                    current_item = Item::new();
                }
            }
            Ok(Event::Text(event)) => {
                if reading_item {
                    match current_tag {
                        Tag::Title => {
                            current_item.title += event.unescape().unwrap().trim();
                        }
                        Tag::Description => {
                            current_item.description += event.unescape().unwrap().trim();
                        }
                        Tag::Link => {
                            current_item.link += event.unescape().unwrap().trim();
                        }
                        Tag::PubDate => {
                            current_item.pub_date += event.unescape().unwrap().trim();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    items
}

fn get_feed() -> String {
    let cache_dir = dirs::cache_dir().map(|d| d.join("news-feed-cli")).expect("Couldn't find cache dir");
    let cache_file = cache_dir.join("news.un.org");

    let is_stale = match fs::metadata(&cache_file) {
        Ok(f) => {
            let result = f.modified().unwrap();
            let duration = SystemTime::now().duration_since(result).expect("Couldn't read last modified");
            duration.as_secs() > 3600
        }
        Err(_) => true
    };

    if is_stale {
        eprintln!("(Updating cache)");
        let resp = match ureq::get(" https://news.un.org/feed/subscribe/en/news/all/rss.xml").call() {
            Ok(resp) => resp.into_string().expect("expected UTF-8 string response"),
            Err(_) => {
                eprintln!("Couldn't fetch news feed");
                std::process::exit(1);
            }
        };
        fs::create_dir_all(cache_dir).expect("Couldn't create cache directory");
        fs::write(cache_file, &resp).unwrap_or_else(|_| eprintln!("Failed updating cache"));
        resp
    } else {
        eprintln!("(Using cached result)");
        fs::read_to_string(cache_file).expect("Couldn't read cached result")
    }
}
