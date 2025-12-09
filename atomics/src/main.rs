use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::spawn,
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // SAFETY: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

fn main() {
    // let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    // let handles: Vec<_> = (0..100)
    //     .map(|_| {
    //         std::thread::spawn(move || {
    //             for _ in 0..1000 {
    //                 l.with_lock(|v| {
    //                     *v += 1;
    //                 })
    //             }
    //         })
    //     })
    //     .collect();

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // assert_eq!(l.with_lock(|v| *v), 100 * 1000);
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    spawn(move || {
        x.store(true, Ordering::SeqCst);
    });
    spawn(move || {
        y.store(true, Ordering::SeqCst);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    println!("z = {}", z);
    // what are the possible values of z?
    // is 0 possible?
    // is 1 possible?
    // is 2 possible?
}
