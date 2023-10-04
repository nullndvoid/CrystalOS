use core::time::Duration;
use embedded_time::{Clock, Timer};
use cmos_rtc::{ReadRTC, Time};
use crate::println;
use super::super::kernel::interrupts::GLOBALTIMER;
use x86_64::instructions::interrupts;
pub fn wait(seconds: f64) {
    let mut start = 0;
    interrupts::without_interrupts(||{
        start = GLOBALTIMER.lock().val;
    });

    loop {
        let mut new = 0;
        interrupts::without_interrupts(||{
            new = GLOBALTIMER.lock().val;
        });
        if new as f64 > start as f64 + seconds * 16.0 {
            return
        }
    };
}

pub fn timer() {
    interrupts::without_interrupts(||{
        println!("{}", GLOBALTIMER.lock().val);
    });
}