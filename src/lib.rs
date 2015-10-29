//! Provides ways of creating random strings from patterns.
//!
extern crate rand;

#[cfg(test)]
mod tests;

use rand::{sample, Rng};

use std::collections::HashMap;

/// Error type for string generation. `empty` contains any catagories that contained no variants,
/// and so therefore had no effect on the generated string; `failed` contains any catagories that
/// were requested by the pattern but did not exist in the catagories hashmap; `attempt` contains
/// what the program could construct ignoring any errors. It should be noted, that if any error
/// occurs, there will be null characters (U+0000) present in the string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenStringError {
    empty: Vec<String>,
    failed: Vec<String>,
    attempt: String

}
impl GenStringError {
    // Create a new empty error
    pub fn new() -> Self {
        GenStringError {
            empty: Vec::new(),
            failed: Vec::new(),
            attempt: String::new()
        }
    }
}

/// Generate a string from a set of catagories, a pattern, and a random number generator. The
/// patterns are scanned through one at a time, then used to pull a slice out of the hashmap. An
/// entry from this is then reandomly selected, and turned into a character. These are all combined
/// into a string and returned. Should an error occur, an instance of `GenStringError` will be
/// returned.
pub fn gen_string<'a, P, R>(cat: &HashMap<String, &'a [u8]>, 
                          pat: P,
                          rng: &mut R) -> Result<String, GenStringError>
    where P: IntoIterator<Item=String>,
          R: Rng
{
    let pat = pat.into_iter();
    let mut failed = false;
    let mut error = GenStringError::new();
    let output: String = pat.map(|key| {
        if let Some(catagory) = cat.get(&key) {
            if let Some(u) = sample(rng, catagory.iter(), 1).iter().nth(0) {
                **u as char
            } else {
                failed = true;
                error.empty.push(key);
                0u8 as char
            }
        } else {
            failed = true;
            error.failed.push(key);
            0u8 as char
        }
    }).collect(); // Hopefully the compiler optimises this from O(2n) to O(n)
    if failed {
        error.attempt = output;
        Err( error )
    } else {
        Ok( output )
    }
}
