use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use cmos_rtc::{ReadRTC, Time};
use lazy_static::lazy_static;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use spin::Mutex;

lazy_static! {
    pub static ref RANDOM: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64({
        let mut cmos = ReadRTC::new(0x00, 0x00);
        let time: Time = cmos.read();
        time.second as u64
            + time.minute as u64
            + time.hour as u64
            + time.day as u64
            + time.month as u64
            + time.year as u64
    }));
}

pub struct Random;

impl Random {
    pub fn int(lower: usize, upper: usize) -> usize {
        loop {
            let integer: u64 = RANDOM.lock().next_u64();
            let mut integer: String = integer.to_string();
            integer = "0".repeat(20 - integer.len()) + &integer;
            let integer: usize = integer[1..upper.to_string().len() + 1].parse().unwrap();
            if integer <= upper && integer >= lower {
                return integer;
            } else {
                continue;
            }
        }
    }

    pub fn selection<T>(ls: &Vec<T>) -> &T {
        let range = Random::int(0, ls.iter().len() - 1);
        &ls[range as usize]
    }
}
