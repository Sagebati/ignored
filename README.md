# ignored

Crate adding the ignore feature to rust. this mimics the ignore function in F#.
The utility for this crate is to add more clarity to the source code. It can also help
removing some braces in match arms and closures, by transforming an statement to an expression.
Usefull also to long mutable pipelines to ignore the last result.
Created because adding braces in closures and match arms is somewhat annoying and add verbosity
without adding clarity for a future reader/maintener.
Ex:
```rust
use ignored::{Ignore, ignore};
fn handle(i: &mut usize) -> bool {
  *i += 12;
   true
}
let mut res: Result<usize, ()> = Ok(3);

match res {
   Ok(ref mut x) => handle(x).ignore(), // Clear intentions
   // Ok(_) => (), // Alternative
   Err(_) => ignore()
   // _ => ignore()
}

assert_eq!(res , Ok(15))
```
