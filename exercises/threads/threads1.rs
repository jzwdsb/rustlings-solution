// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.

// This program spawns multiple threads that each run for at least 250ms,
// and each thread returns how much time they took to complete.
// The program should wait until all the spawned threads have finished and
// should collect their return values into a vector.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    for i in 0..10 {
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis();
            results.lock().unwrap().push(start.elapsed().as_millis());
        }));
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        handle.join().unwrap();
        completed_threads += 1;
    }

    if results.lock().unwrap().len() != completed_threads {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.lock().unwrap().iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
