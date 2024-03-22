use cmos_rtc::Time;
use crate::println;
use super::super::kernel::interrupts::GLOBALTIMER;
use x86_64::instructions::interrupts;
use crate::system::kernel::interrupts::InterruptIndex;

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

pub struct Timer {
    duration: f64,
    end: f64
}

impl Timer {
    pub(crate) fn new(seconds: f64) -> Self {
        let mut start = 0;
        interrupts::without_interrupts(||{
            start = GLOBALTIMER.lock().val;
        });

        Timer {
            duration: seconds,
            end: start as f64 + seconds * 16.0
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        let mut done = false;
        interrupts::without_interrupts(||{
            done = GLOBALTIMER.lock().val as f64 > self.end;
        });
        done
    }

    pub(crate) fn reset(&mut self) {
        let mut start = 0;
        interrupts::without_interrupts(||{
            start = GLOBALTIMER.lock().val;
        });
        self.end = start as f64 + self.duration * 16.0
    }
}