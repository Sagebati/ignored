#![no_std]

///! Crate adding the ignore feature to rust. this mimics the ignore function in F#.
///! The utility for this crate is to add more clarity to the source code. It can also help
///! removing some braces in match arms and closures, by transforming an statement to an expression.
///! Usefull also to long mutable pipelines to ignore the last result.
///! Created because adding braces in closures and match arms is somewhat annayoning and add verbosity
///! without adding clarity for a future reader/maintener.
///! Ex:
/// ```
///use ignored::{Ignore, ignore};
///fn handle(i: &mut usize) -> bool {
///   *i += 12;
///    true
///}
///let mut res: Result<usize, ()> = Ok(3);
///
///match res {
///    Ok(ref mut x) => handle(x).ignore(), // Clear intentions
///    // Ok(_) => (), // Alternative
///    Err(_) => ignore()
///    // _ => ignore()
/// }
///
/// assert_eq!(res , Ok(15))
/// ```

pub trait Ignore {
    #[inline(always)]
    fn ignore(&self) {}
}

impl<T> Ignore for T {}

#[inline(always)]
pub fn ignore() -> () {}

#[cfg(test)]
mod tests {
    use crate::{ignore, Ignore};

    #[test]
    fn test1() {
        fn handle(i: &mut usize) -> bool {
            *i += 12;
            true
        }
        let mut res: Result<usize, ()> = Ok(3);

        match res {
            Ok(ref mut x) => handle(x).ignore(), // Clear intentions
            // Ok(_) => (), // Alternative
            Err(_) => ignore(),
        }

        assert_eq!(res, Ok(15));
    }

    #[test]
    fn test2() {
        fn ret_res(i: &mut usize) -> Result<(), ()> {
            *i += 2;
            Ok(())
        }
        let mut i = 1;

        // ret_res(&mut i);  Warn unused must use;
        ret_res(&mut i).ignore(); // Clear intentions
        assert_eq!(i, 3)
    }

    #[test]
    fn test3() {
        fn map_with_ret(i: &mut usize) -> bool {
            *i += 1;
            true
        }
        let mut array = [0; 10];
        array.iter_mut().for_each(|i| map_with_ret(i).ignore());
        assert_eq!(array, [1; 10]);
    }
}