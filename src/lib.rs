use std::mem;

pub struct Clink<T, F>
where
    F: Fn(&T) -> bool,
{
    value: T,
    invariant: F,
}

impl<T, F> Clink<T, F>
where
    F: Fn(&T) -> bool,
{
    pub fn new(val: T, inv: F) -> Clink<T, F> {
        // Check precondition
        assert!((inv)(&val));
        // Continue
        Clink {
            value: val,
            invariant: inv,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, val: T) {
        // Check precondition
        assert!((self.invariant)(&val));
        // set value
        self.value = val;
    }

    pub fn swap(&mut self, mut val: T) -> T {
        // Check precondition
        assert!((self.invariant)(&val));
        // Perform swap
        mem::swap(&mut val, &mut self.value);
        // Done
        val
    }
}
