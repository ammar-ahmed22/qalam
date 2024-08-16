pub struct Stack<T> {
  elements: Vec<T>,
}

impl <T> Stack<T> {
  pub fn new() -> Self {
    return Self {
      elements: Vec::new()
    }
  }

  pub fn push(&mut self, item: T) {
    self.elements.push(item);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.elements.pop()
  }

  pub fn peek(&self) -> Option<&T> {
    self.elements.last()
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.elements.last_mut()
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  pub fn size(&self) -> usize {
    self.elements.len()
  }

  pub fn get(&self, i: usize) -> Option<&T> {
    self.elements.get(i)
  }

  pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
    self.elements.get_mut(i)
  }
}

