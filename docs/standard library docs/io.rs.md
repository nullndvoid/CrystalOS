
- provides structs and functions for input and output

provided features:

```rust
print()        // prints a formatted string to the screen
println!()     // print with a newline appended

print_log!()   // prints a formatted log string in yellow
println_log!() // print_log with newline appended

printerr!()    // prints formatted error in yellow with appended newline, this is called when the OS panics.

impl Screen {
	pub fn terminal_mode();    // changes the display mode of the kernel to terminal mode, meaning text can be inputted line by line as commands.
	pub fn application_mode(); // changes the display mode of the kernel to application mode, meaning applications are responsible for generating their own frames and rendering them.
	pub fn switch();           // toggles the current display mode
	pub fn clear();            // clears the screen
}

impl Stdin {
	pub async fn readline() -> String;      // reads a line of input in terminal mode
	pub async fn keystroke() -> char;       // waits for the user to enter a keystroke
	pub fn try_keystroke() -> Option<char>; // immediately returns a keystroke if an unread one has been received by the kernel
}
```
