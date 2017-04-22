#![feature(io)]

use std;

// use time;
// use timer;

// http://aturon.github.io/stability-dashboard/std/io/timer/struct.Timer.html

pub struct Clock {
    pub rate: std::time::Duration,
    pub ticks: u64, 
    // ticker: std::sync::mpsc::Receiver<()>,
}

impl Clock {
    fn new(rate: std::time::Duration) -> Clock {
        Clock {
            rate: rate,
            ticks: 0, 
            // ticker: Timer::new().unwrap().periodic(rate),
        }
    }

    fn tick(&self) -> () {
        // self.ticker.recv();
    }
}