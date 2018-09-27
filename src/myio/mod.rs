use std::io::{self, Read};
use std::str::FromStr;
use std::fmt::Debug;
use std::time::{Instant, Duration};

pub fn string_to_parsed_tokens<T>(s: String) -> Vec<T>
where
  T: FromStr,
  <T as FromStr>::Err: Debug //T's associated Err type must be debug printable
{
  let splits = s.split(|c:char| c.is_whitespace())
    .filter(|s| *s != "")
    .map(|sl| sl.parse::<T>().expect(""))
    .collect::<Vec<T>>();
  splits
  
}

pub fn get_tokens_from_stdin<T>() -> Vec<T>
where
  T: FromStr,
  <T as FromStr>::Err: Debug
{
  let mut s = String::new();
  io::stdin().read_to_string(&mut s).expect("couldn't read from stdin");
  return string_to_parsed_tokens::<T>(s);
}

/// benchmark a function. Pass in a closure taking no parameters, returns
/// whatever the closure returned and how long it took to run the closure
pub fn benchmark<T, F>(func: F) -> (Duration, T)
where F: FnOnce() -> T
{
  let time_start = Instant::now();
  let res = func();
  let time_end = Instant::now();
  (time_end - time_start, res)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_get_tokens() {
    assert_eq!(string_to_parsed_tokens::<u32>("1 2 3 15  61".to_string()), vec![1,2,3,15,61]);
    assert_eq!(string_to_parsed_tokens::<String>("hello world hi there".to_string()),
      vec!["hello", "world", "hi", "there"]);
  }
}