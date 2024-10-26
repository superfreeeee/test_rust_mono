use std::{sync::mpsc, thread, time::Duration};

use rand::Rng;

const PRODUCER_INTERVAL: u64 = 50;
const PRODUCER_INTERVAL_STEP_MIN: u64 = 1;
const PRODUCER_INTERVAL_STEP_MAX: u64 = 3;
const CONSUMER_INTERVAL: u64 = 0;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let producers = {
        let mut list = vec![];

        for i in 1..=3 {
            let sender_cloned = sender.clone();
            list.push(thread::spawn(move || {
                let mut rng = rand::thread_rng();

                for j in 1..=5 {
                    let delay = rng
                        .gen_range(PRODUCER_INTERVAL_STEP_MIN..=PRODUCER_INTERVAL_STEP_MAX)
                        * PRODUCER_INTERVAL;
                    thread::sleep(Duration::from_millis(delay));

                    sender_cloned
                        .send(format!("producer {}, task {}", i, j))
                        .unwrap();
                }

                drop(sender_cloned);

                thread::sleep(Duration::from_secs(1));

                println!("producer {} finished", i);
            }));
        }

        list
    };
    drop(sender);

    let consumer = thread::spawn(move || {
        while let Ok(res) = receiver.recv() {
            println!("res = {}", res);
            // thread::sleep(Duration::from_millis(CONSUMER_INTERVAL));
        }
    });

    consumer.join().unwrap();

    println!("consumer done");

    for producer in producers {
        producer.join().unwrap();
    }

    println!("producers done");
}
