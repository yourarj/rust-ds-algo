pub struct CustomStack {
    max_size: i32,
    inner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    pub fn new(max_size: i32) -> Self {
        CustomStack {
            max_size,
            inner: Vec::with_capacity(max_size as usize),
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.inner.len() < self.max_size as usize {
            self.inner.push(x);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.inner.pop().unwrap_or(-1)
    }

    pub fn increment(&mut self, k: i32, val: i32) {
        for (index, elem) in self.inner.iter_mut().enumerate() {
            if index < k as usize {
                *elem += val;
            }
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

#[cfg(test)]
mod tests {
    use super::CustomStack;
    #[test]
    fn test_01() {
        // ["CustomStack","push","push","pop","push","push","push","increment","increment","pop","pop","pop","pop"]
        // [[3],[1],[2],[],[2],[3],[4],[5,100],[2,100],[],[],[],[]]
        let mut obj = CustomStack::new(3);
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.pop(), 2);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        obj.increment(5, 100);
        obj.increment(2, 100);
        assert_eq!(obj.pop(), 103);
        assert_eq!(obj.pop(), 202);
        assert_eq!(obj.pop(), 201);
        assert_eq!(obj.pop(), -1);
    }
}
