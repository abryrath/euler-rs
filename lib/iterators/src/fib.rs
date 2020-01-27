use num::traits::*;
use std::ops::Add;
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct Fib<T> {
    pub i: u32,
    curr: T,
    next: T,
}

impl<T> Fib<T>
where
    T: Zero + One,
{
    pub fn new() -> Fib<T> {
        Fib {
            i: 0,
            curr: T::zero(),
            next: T::one(),
        }
    }
}

impl<T> Iterator for Fib<T>
where
    T: Zero + One,
    for<'a, 'b> &'a T: Add<&'b T, Output = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // let new_next = self.curr + self.next;
        // self.curr = self.next;
        // self.next = new_next;
        // self.i += 1;
        let next1 = (&self.next) + (&self.curr);
        let next2 = (&self.next) + (&self.curr);

        self.curr = mem::replace(&mut self.next, next1);
        let index = (&self.i) + 1;
        self.i = mem::replace(&mut self.i, index);

        Some(next2)
    }
}
#[test]
fn fib_test() {
    let values: Vec<u32> = Fib::new().take(4).collect();
    assert_eq!(values, [1, 2, 3, 5]);
}
