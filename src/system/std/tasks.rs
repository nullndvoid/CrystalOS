pub use crate::system::kernel::tasks::{executor::Executor, Task};

pub fn stop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
