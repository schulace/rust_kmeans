use std::io::{self, Read};
use std::str::FromStr;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

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

pub fn vec_to_rc_vec<T>(mut v: Vec<T>) -> Vec<Rc<RefCell<T>>> {
  let mut ret = Vec::with_capacity(v.len());
  for _ in 0..v.len() {
    ret.push(Rc::new(v.pop().unwrap()));
  }
  ret
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