#![allow(unused_imports)]

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use rand::Rng;

#[allow(dead_code)]
const PRODUCER_INTERVAL: u64 = 50;
#[allow(dead_code)]
const PRODUCER_INTERVAL_STEP_MIN: u64 = 1;
#[allow(dead_code)]
const PRODUCER_INTERVAL_STEP_MAX: u64 = 3;
#[allow(dead_code)]
const CONSUMER_INTERVAL: u64 = 0;

#[test]
fn test_vec() {
    let vec = vec![1, 2, 3];
    for v in vec.iter() {
        println!("v = {v}");
    }
    // for v in vec {
    //     println!("v = {v}");
    // }
    for v in vec {
        println!("v = {v}");
    }
}

#[test]
fn test_vec_iter() {
    let mut vec = vec![1, 2, 3];
    vec.iter().for_each(|v: &i32| {
        println!("iter v = {v}");
    });
    vec.iter_mut().for_each(|v: &mut i32| {
        println!("iter_mut v = {v}");
    });
    vec.into_iter().for_each(|v: i32| {
        println!("into_iter v = {v}");
    });
    // vec.iter().for_each(|v| {
    //     println!("v = {v}");
    // });
}

fn main() {}

#[test]
fn test_producer_and_consumer() {
    let (sender, receiver) = mpsc::channel();

    let producers = (0..3)
        .map(|i| {
            let sender_cloned = sender.clone();
            thread::spawn(move || exec_producer(i, sender_cloned))
        })
        .collect::<Vec<_>>();
    drop(sender);

    let consumer = thread::spawn(move || exec_consumer(receiver));

    consumer.join().unwrap();
    producers.into_iter().for_each(|producer| {
        producer.join().unwrap();
    });
}

#[allow(dead_code)]
fn exec_producer(id: u32, sender: Sender<String>) {
    let mut rng = rand::thread_rng();

    for j in 1..=5 {
        let delay = rng.gen_range(PRODUCER_INTERVAL_STEP_MIN..=PRODUCER_INTERVAL_STEP_MAX)
            * PRODUCER_INTERVAL;
        thread::sleep(Duration::from_millis(delay));

        sender
            .send(format!(
                "producer {}{}-{}",
                "....".repeat(id as usize),
                id,
                j
            ))
            .unwrap();
    }

    drop(sender);

    thread::sleep(Duration::from_secs(1));

    println!("producer {} finished", id);
}

#[allow(dead_code)]
fn exec_consumer(receiver: Receiver<String>) {
    while let Ok(res) = receiver.recv() {
        println!("res = {}", res);
        // thread::sleep(Duration::from_millis(CONSUMER_INTERVAL));
    }
}
