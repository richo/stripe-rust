use std::slice::Items;
pub trait Enumerable<T> {
    fn iter<'a>(&'a mut self) -> Items<'a, T>;
        // self.items().iter()
    // }

    // fn items(&self) -> ~[T];
}
