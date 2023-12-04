use x86_64::VirtAddr;
use crate::system::kernel::memory::{StackBounds, ThreadId};

#[derive(Debug)]
pub struct Thread {
	id: ThreadId,
	stack_ptr: Option<VirtAddr>,
	stack_bounds: Option<StackBounds>,
}