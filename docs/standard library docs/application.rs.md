- provides a simple trait to standardise the way applications are called and run from the shell or other applications. 

```rust
use async_trait::async_trait;

// all applications should implment this trait and be started by using simlar to:
let res = ImplementsApplication.run(args).await?;

#[async_trait]
pub trait Application {
	fn new -> Self;

	async fn run(&mut self, _: Vec<String>) -> Result<(), Error>;
}
```
## important: 
- the async_trait crate must be in scope in the file or else an error about trait bounds will be displayed at compile time. 

