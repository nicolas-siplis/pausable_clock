# Pausable Clock

This crate provides a clock that can be paused ... (duh?). The provided struct `PausableClock` allows you to get the current time in a way that respects the atomic state and history of the clock.  Put more simply, a pausable clock's elapsed time increases at the same as real time but only when the clock is resumed.

## Features
- Thread-Safe: (`Send`/`Sync`) All operations on the clock are atomic or use std mutexes
- Resume Notification: the `wait_for_resume` method will block until the clock is resumed (if the clock is paused)
- Guarantees: Just like `std::time::Instant::now()` guarantees that [time always increases](https://doc.rust-lang.org/src/std/time.rs.html#238), `PausableClock` guarantees that the time returned by `clock.now()` while the clock is paused is >= any other instant returned before the clock was paused.
- Unpausable Tasks: We provide a method called `run_unpausable` that allows tasks to be run that can prevent the timer from being paused while they are still running.

## Example

```rust
use pausable_clock::PausableClock;
use std::sync::Arc;
use std::thread;
use instant::{Instant, Duration};

let clock = Arc::new(PausableClock::default());

// With the default parameters, there should be no difference
// between the real time and the clock's time
assert!(Instant::from(clock.now()).elapsed().as_millis() == 0);

// Pause the clock right after creation
clock.pause();

// Clone the arc of the clock to pass to a new thread
let clock_clone = clock.clone();

let t = thread::spawn(move || {
    // In the new thread, just wait for resume
    clock_clone.wait_for_resume();
});

// Sleep for a sec, then resume the clock
thread::sleep(Duration::from_secs(1));
clock.resume();

// Wait for the spawned thread to unblock
t.join().unwrap();

// After being paused for a second, the clock is now a second behind
// (with a small error margin here because sleep is not super accurate)
assert!((Instant::from(clock.now()).elapsed().as_secs_f64() - 1.).abs() < 0.005);
```

## Caveats
- We use an `AtomicU64` to contain the entire state of the pausable clock, so the granularity of the instant's produced by the clock is milliseconds. This means the maximum time the timer can handle is on the order of hundreds of thousands of years.
- Reads of the pause state for `PausableClock::is_paused` is done atomically with `Ordering::Relaxed`. That allows the call to be slightly faster, but it means you shouldn't think it as fencing a operations. You can use `PausableClock::is_paused_ordered` if you need that kind of guarantee.
- There is a significant amount of weakly-ordered atomic operation going on in this library to make sure the calls to now and unpausable task don't require any locks. I can't claim that it is provably correct, but it has been tested to high degree of certainty on x86_64 processors. Tests on weakly ordered systems are forthcoming as are `loom`-based tests.