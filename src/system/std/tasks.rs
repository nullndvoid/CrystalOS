pub use crate::kernel::tasks::{Task, executor::Executor};

pub fn stop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}