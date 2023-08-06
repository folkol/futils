use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;

struct Brownian {
    history: VecDeque<f32>,
}

impl Brownian {
    fn new() -> Self {
        Brownian {
            history: VecDeque::new()
        }
    }
}

const WINDOW: usize = 20;

impl Iterator for Brownian {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let i: f32 = rand::thread_rng().gen();
        self.history.push_back(i);
        if self.history.len() > WINDOW {
            self.history.pop_front();
        }
        let iter = self.history.iter();
        let x = iter.sum::<f32>() / self.history.len() as f32;
        // println!("{x}");
        Some(x)
    }
}

impl rodio::Source for Brownian {
    fn current_frame_len(&self) -> Option<usize> {
        // println!("current_frame_len");
        None
    }

    fn channels(&self) -> u16 {
        // println!("channels");
        1
    }

    fn sample_rate(&self) -> u32 {
        // println!("sample_rate");
        44000
    }

    fn total_duration(&self) -> Option<Duration> {
        // println!("total_duration");
        None
    }
}

fn main() {
    let (_os, handle) = rodio::OutputStream::try_default().unwrap();
    let source = Brownian::new();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.set_volume(0.05);
    sink.append(source);
    sink.sleep_until_end();
}