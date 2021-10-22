use hashbrown::HashSet;
use std::hash::{Hash, Hasher};

pub trait Intersection<T> {
  fn intersection(&mut self, subset: &Vec<&T>) -> bool;
}

impl<T> Intersection<T> for Vec<T>
where
  T: std::cmp::PartialEq + Copy
{
  fn intersection(&mut self, subset: &Vec<&T>) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut updated = false;
    let mut len = self.len();
    while i < len && j < subset.len() {
      if self[i] != *subset[j] {
        if i < j {
          self.remove(i);
          updated = true;
          len -= 1;
        } else {
          j += 1;
        }
      } else {
        i += 1;
        j += 1;
      }
    }
    updated // was the vector was updated
  }

  //fn intersection(self, subset: Vec<usize>) -> Vec<usize> {
  //  let mut i = 0;
  //  let mut j = 0;
  //  let mut v = Vec::new();
  //  while i < self.len() && j < subset.len() {
  //    if self[i] != subset[j] {
  //      if i < j {
  //        i+=1;
  //      } else {
  //        j+=1;
  //      }
  //    } else {
  //      v.push(self[i]);
  //      i+=1;
  //      j+=1;
  //    }
  //  }
  //  v
  //}

  //fn intersection(self, subset: Vec<usize>) -> Vec<usize> {
  //  let mut i = 0;
  //  let mut j = 0;
  //  let mut v = Vec::new();
  //  while i < self.len() && j < subset.len() {
  //    if self[i] == subset[j] {
  //      v.push(self[i]);
  //    } else if self[i] > subset[j] {
  //      j+=1;
  //    } else {
  //      i+=1;
  //    }
  //  }
  //  v
  //}
}

pub trait Intersect {
  fn intersect(&mut self, subset: &HashSet<usize>) -> bool;
}

impl Intersect for Vec<usize> {
  fn intersect(&mut self, subset: &HashSet<usize>) -> bool {
    let mut len = self.len();
    self.retain(|&x| subset.contains(&x));
    len != self.len() // was the vector was updated
  }
}
