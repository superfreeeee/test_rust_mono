#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    cell::{Cell, RefCell, UnsafeCell},
    sync::{Arc, RwLock},
    thread,
};

fn main() {}

#[test]
fn test_closure() {
    let mut count: u32 = 0;

    println!("count = {}", count);

    let mut increment = || -> u32 {
        count += 1;
        count
    };

    increment();
    increment();
    increment();

    println!("count = {}", count);
}

#[test]
fn test_cell() {
    let count = Cell::new(0);
    count.set(1);
    println!("count = {}", count.get());
}

#[test]
fn test_cell_closure() {
    let count = Cell::<u32>::new(0);

    let increment = || -> u32 {
        count.set(count.get() + 1);
        count.get()
    };

    println!("count = {}", count.get());

    increment();
    increment();
    increment();

    println!("count = {}", count.get());
}

#[test]
fn test_ref_cell_closure() {
    let count = RefCell::<u32>::new(0);

    let increment = || -> u32 {
        let mut mut_count = count.borrow_mut();
        *mut_count += 1;
        *mut_count
    };

    println!("count = {}", count.borrow());

    increment();
    increment();
    increment();

    println!("count = {}", count.borrow());
}

struct ThreadSafeRwCell<T: Copy> {
    inner: RwLock<T>,
}

impl<T: Copy> ThreadSafeRwCell<T> {
    fn new(value: T) -> Self {
        ThreadSafeRwCell {
            inner: RwLock::new(value),
        }
    }

    fn get(&self) -> T {
        println!("ThreadSafeRwCell get");
        let value = self.inner.read().unwrap();
        *value
    }

    fn set(&self, next_value: T) {
        println!("ThreadSafeRwCell set");
        let mut value = self.inner.write().unwrap();
        *value = next_value;
    }
}

#[test]
fn test_cell_closure_multi_thread() {
    let count = Arc::new(ThreadSafeRwCell::new(0));

    let count_cloned = count.clone();
    let worker1 = thread::spawn(move || {
        println!("get = {}", count_cloned.get());
    });

    let count_cloned = count.clone();
    let worker2 = thread::spawn(move || {
        println!("set");
        count_cloned.set(1);
    });

    worker1.join().unwrap();
    worker2.join().unwrap();
}

/// ==============================
/// strategy pattern
struct BinaryCalculator<F: Fn(u32, u32) -> u32> {
    func: F,
}

impl<F: Fn(u32, u32) -> u32> BinaryCalculator<F> {
    fn new(func: F) -> Self {
        BinaryCalculator { func }
    }

    fn run(&self, x: u32, y: u32) -> u32 {
        (self.func)(x, y)
    }
}

#[test]
fn test_strategy_closure() {
    let res = BinaryCalculator::new(|x, y| x + y).run(3, 7);
    println!("res = {}", res);
    let res = BinaryCalculator::new(|x, y| x - y).run(7, 3);
    println!("res = {}", res);
}
