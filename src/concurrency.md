# Chapter 3

## 3.1 Concurrencry Using Channels

```rust

use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};

pub fn main() {
    let xs = run(vec![1, 2, 3, 4]);
    println!("{:?}", xs);
}

fn run(xs: Vec<u32>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();

    for x in xs {
        let vals: Vec<u32> = vec![x, x + 1];
        let tx1 = tx.clone();
        thread::spawn(move || {
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // Clones of tx are dropped when threads finish,
    // but the original tx remains alive. This means
    // we can still receive messages from it, potentially
    // causing the receiver to block indefinitely. To
    // prevent this, we need to drop the original tx to
    // signal that the channel is closed and no further
    // messages will be sent.
    drop(tx);

    // Now the receiver can safely iterate and won't block forever
    let mut xs = vec![];
    for received in rx {
        xs.push(received);
    }

    xs
}


```


## 3.2 Concurrency Using Mutexes

```rust

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let v = vec!["hello", "world"];
    let m = frequency(&v, 1);
    println!("{:?}", m);
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let xs: Vec<char> = input
        .iter()
        .flat_map(|s| {
            s.chars().filter_map(|c| {
                if c.is_alphabetic() {
                    let lc: Vec<_> = c.to_lowercase().collect();
                    Some(lc[0])
                } else {
                    None
                }
            })
        })
        .collect();
    let chunk_size = xs.len() / (worker_count + 1) + 1;

    let chunks = xs.chunks(chunk_size);
    let counter = Arc::new(Mutex::new(HashMap::new()));
    for chunk in chunks {
        let xs = chunk.to_vec();
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let xs = counts(xs);
            for (c, n) in xs {
                if let Ok(ref mut num) = counter.try_lock() {
                    let map = num;
                    *map.entry(c).or_insert(0) += n;
                }
            }
        })
        .join()
        .unwrap();
    }

    let mut result = HashMap::new();
    for (k, v) in counter.lock().unwrap().iter() {
        result.insert(*k, *v);
    }

    result
}
fn counts(xs: Vec<char>) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for c in xs {
        *counter.entry(c).or_insert(0) += 1;
    }
    counter
}

```
