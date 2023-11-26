pub use crate::system::kernel::tasks::{Task, executor::Executor};

pub fn stop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}