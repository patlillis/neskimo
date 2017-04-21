#![feature(io)]

// use time;
// use timer;

// http://aturon.github.io/stability-dashboard/std/io/timer/struct.Timer.html
use std::old_io::Timer;
use std::old_io::timer;
use std::sync::mpsc;

pub struct Clock {
    pub rate: time::Duration,
    pub ticks: u64,
    ticker: mspc::Receiver<()>,
}

impl Clock {
    fn new(rate: time::Duration) -> Clock {
        Clock {
            rate: rate,
            ticks: 0
            ticker: Timer::new().unwrap().periodic(rate),
        }
    }

    fn tick(&self) -> () {
        self.ticker.recv();
    }
}