pub struct Queue<T> {
    pub older: Vec<T>,
    pub younger: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, v: T) {
        self.younger.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

           use std::mem::swap;
           swap(&mut self.older, &mut self.younger);
           self.older.reverse(); 
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

#[test]
fn test_queue() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    assert!(q.is_empty());

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), None);
}