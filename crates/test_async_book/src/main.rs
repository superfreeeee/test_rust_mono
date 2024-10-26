use futures::{
    executor::block_on,
    future::{self, BoxFuture},
    join, pin_mut, select,
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    f32::consts::PI,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{
        self,
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread::{self, sleep},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
// The timer we wrote in the previous section:
use test_async_book::TimerFuture;

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender
            .send(task.clone())
            .expect("too many tasks queued");
        self.task_sender.send(task).unwrap();
    }
}

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}

/// =============================================
/// manual future
#[test]
fn test_block_on() {
    let task = async {
        println!("test");
    };
    block_on(task);
}

#[inline]
fn sys_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Debug)]
struct ManualFutureState {
    count: u32,
    done: bool,
    waker: Option<Waker>,
}

impl ManualFutureState {
    fn update(&mut self, done: bool) {
        println!("[{:?}] update", sys_now());
        self.count += 1;
        self.done = done;
        if done {
            if let Some(waker) = self.waker.take() {
                waker.wake();
            }
        } else {
            if let Some(waker) = self.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }
}

struct ManualFuture {
    state: Arc<Mutex<ManualFutureState>>,
}

impl Future for ManualFuture {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        println!(
            "[{}] poll ManualFuture: count = {}, done = {}",
            sys_now(),
            state.count,
            state.done
        );

        if state.done {
            Poll::Ready(state.count)
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[test]
fn test_manual_future() {
    let state = Arc::new(Mutex::new(ManualFutureState {
        count: 0,
        done: false,
        waker: None,
    }));

    println!("test start");
    let state1 = state.clone();
    thread::spawn(move || {
        println!("thread start");
        sleep(Duration::from_millis(100));
        state1.lock().unwrap().update(false);
        sleep(Duration::from_millis(100));
        state1.lock().unwrap().update(false);
        sleep(Duration::from_millis(100));
        state1.lock().unwrap().update(true);
        println!("thread end");
    });

    let future = ManualFuture {
        state: state.clone(),
    };

    block_on(async {
        println!("block on start");
        let count = future.await;
        println!("{}", count);
    });
}

/// =============================================
/// deref
struct Test;

impl Deref for Test {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &1
    }
}

impl DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

fn test(num: &mut u32) {}

#[test]
fn test_deref() {
    let mut t = Test;
    test(&mut t);
}

/// =============================================
/// ref lifetime
async fn foo(x: &u8) -> u8 {
    *x
}

fn bar() -> impl Future<Output = u8> {
    let x = 5;
    async move {
        let f = foo(&x);
        f.await
    }
    // async {
    //     f.await;
    //     let x = foo(&1).await;
    //     x + 5
    // }
}

/// =============================================
/// mutex
#[test]
fn test_mutex() {
    // sync_mutex_test();
    block_on(future_mutex_test());
}

fn sync_mutex_test() {
    println!("go");

    let mutex = sync::Mutex::new(0);

    let f1 = async {
        println!("run f1");
        let mut v1 = mutex.lock().unwrap();
        *v1 += 1;
        println!("value 1 = {}", v1);
    };

    let f2 = async {
        println!("run f2");
        let v2 = mutex.lock().unwrap();
        println!("value 2 = {}", v2);
    };

    block_on(async {
        futures::join!(f1, f2);
    });
}

async fn future_mutex_test() {
    let mutex = futures::lock::Mutex::new(0);

    let v1 = mutex.lock();
    let f1 = async {
        let mut v1 = v1.await;
        *v1 += 1;
        println!("value 1 = {}", *v1);
    };

    let f2 = async {
        let v2 = mutex.lock().await;
        println!("value 2 = {}", *v2);
    };

    async {
        futures::join!(f1, f2);
    }
    .await;
}

/// =============================================
/// join!
#[test]
fn test_join() {
    let (_, _, res3) = block_on(async {
        join!(
            async {
                println!("test 1");
                1u32
            },
            async {
                println!("test 2");
                2u32
            },
            async {
                println!("test 3");
                3u32
            }
        )
    });

    println!("res3 = {}", res3);
}

/// =============================================
/// select!
#[test]
fn test_select_loop() {
    let mut a_fut = future::ready(1);
    let mut b_fut = future::ready(2);

    let mut sum = 0u32;

    loop {
        select! {
            a = a_fut => {
                sum += a;
                println!("a = {}", a);
            },
            b = b_fut => {
                sum += b;
                println!("b = {}", b);
            },
            complete => break,
            default => unreachable!()
        }
    }

    // let res = block_on(async {
    //     a_fut.await
    // });

    assert_eq!(sum, 3);
    // assert_eq!(res, 1);
}

#[test]
fn test_select() {
    let mut timer1 = TimerFuture::new(Duration::from_millis(200)).fuse();
    let mut timer2 = TimerFuture::new(Duration::from_millis(100)).fuse();

    block_on(async {
        println!("[{}] start", sys_now());

        loop {
            select! {
                _ = timer1 => {
                    println!("[{}] timer1 done", sys_now());
                },
                _ = timer2 => {
                    println!("[{}] timer2 done", sys_now());
                },
                complete => {
                    println!("complete");
                    break;
                },
            }
        }
    });
}
