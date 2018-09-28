pub struct SortedSliceIter<'a, T, F>
where
  T: 'a,
  F: Fn(&'a T, &'a T) -> bool,
{
  elems: &'a [T],
  predicate: F,
}

impl<'a, T, F> Iterator for SortedSliceIter<'a, T, F>
where
  T: 'a,
  F: Fn(&'a T, &'a T) -> bool,
{
  type Item = &'a [T];
  fn next(&mut self) -> Option<&'a [T]> {
    if self.elems.len() == 0 {
      None
    } else {
      let mut i = 0;
      let cmp_to = &self.elems[0];
      while i < self.elems.len() && (self.predicate)(&self.elems[i], cmp_to) {
        i += 1;
      }
      let ret = &self.elems[0..i];
      self.elems = &self.elems[i..];
      Some(ret)
    }
  }
}

impl<'a, T, F> SortedSliceIter<'a, T, F>
where
  T: 'a,
  F: Fn(&'a T, &'a T) -> bool,
{
  pub fn new(elems: &'a [T], predicate: F) -> SortedSliceIter<'a, T, F> {
    SortedSliceIter { elems, predicate }
  }
}
