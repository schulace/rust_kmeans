use std::io::{self, Read};
use std::str::FromStr;
use std::fmt::Debug;

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

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_get_tokens() {
    assert_eq!(string_to_parsed_tokens::<u32>("1 2 3 15  61".to_string()), vec![1,2,3,15,61]);
  }
}