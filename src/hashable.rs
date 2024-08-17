use std::collections::HashMap;
use core::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct HashableMap<K, V> {
  map: HashMap<K, V>
}

impl <K, V> HashableMap<K, V> 
where K: Eq + PartialEq + Hash
{
  pub fn new() -> Self {
    return Self {
      map: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    self.map.insert(key, value)
  }

  // Get a reference to a value by key
  pub fn get(&self, key: &K) -> Option<&V> {
      self.map.get(key)
  }

  // Get a mutable reference to a value by key
  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
      self.map.get_mut(key)
  }

  // Check if the map contains a key
  pub fn contains_key(&self, key: &K) -> bool {
      self.map.contains_key(key)
  }

  // Remove a key-value pair by key
  pub fn remove(&mut self, key: &K) -> Option<V> {
      self.map.remove(key)
  }

  // Get the number of elements in the map
  pub fn len(&self) -> usize {
      self.map.len()
  }

  // Check if the map is empty
  pub fn is_empty(&self) -> bool {
      self.map.is_empty()
  }

}

impl <K, V> Hash for HashableMap<K, V> 
where 
  K: Hash + Eq + Ord,
  V: Hash
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      let mut items: Vec<_> = self.map.iter().collect();
      items.sort_by(|a, b| a.0.cmp(b.0));
      for (key, value) in items {
        key.hash(state);
        value.hash(state);
      }
  }
}

impl<K, V> PartialEq for HashableMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<K, V> Eq for HashableMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
}

#[derive(Debug)]
pub struct HashableRcRefCell<T>(pub Rc<RefCell<T>>);

impl <T> HashableRcRefCell<T> {
  pub fn init(val: T) -> Self {
    return Self(Rc::new(RefCell::new(val)))
  }
}

impl <T: Hash> Hash for HashableRcRefCell<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      let borrowed = self.0.borrow();
      borrowed.hash(state);
  }
}

impl <T> Clone for HashableRcRefCell<T> {
  fn clone(&self) -> Self {
      HashableRcRefCell(Rc::clone(&self.0))
  }
}

impl <T: PartialEq> PartialEq for HashableRcRefCell<T> {
  fn eq(&self, other: &Self) -> bool {
      let self_borrowed = self.0.borrow();
      let other_borrowed = other.0.borrow();
      *self_borrowed == *other_borrowed
  }
}

impl <T: PartialEq + Eq> Eq for HashableRcRefCell<T> {}