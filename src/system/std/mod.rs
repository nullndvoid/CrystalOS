pub mod application;
pub mod io;
pub mod os;
pub mod random;
pub mod render;
pub mod syscall;
pub mod tasks;
pub mod time;

// this is where the standard library for the operating system will be defined
// my aim is to completely separate this from the shell.

// these functions should all be asynchronous.
