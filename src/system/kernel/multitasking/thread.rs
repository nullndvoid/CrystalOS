use crate::system::kernel::memory::{StackBounds, ThreadId};
use x86_64::VirtAddr;

#[derive(Debug)]
pub struct Thread {
    id: ThreadId,
    stack_ptr: Option<VirtAddr>,
    stack_bounds: Option<StackBounds>,
}
