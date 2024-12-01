# 2. Interior Mutability

## 2.1 Dynamic Borrowing using `RefCell<T>`

`RefCell<T>` uses Rust’s lifetimes to implement “dynamic borrowing”,
a process whereby one can claim temporary, exclusive, mutable access
to the inner value. Borrows for `RefCell<T>`s are tracked at runtime,
unlike Rust’s native reference types which are entirely tracked
statically, at compile time.

## 2.1.1 Basic Usage

```rust

use std::cell::RefCell;

struct MutableInterior {
    hide_me: i32,
    vec: Vec<i32>,
}
struct Foo {
    //although not used in this particular snippet,
    //the motivating problem uses interior mutability
    //via RefCell.
    interior: RefCell<MutableInterior>,
}

use std::cell::Ref;
use std::vec;

impl Foo {
    pub fn get_items(&self) -> Ref<'_, Vec<i32>> {
        Ref::map(self.interior.borrow(), |mi| &mi.vec)
    }
}

fn main() {
    let f = Foo {
        interior: RefCell::new(MutableInterior {
            vec: vec![1, 2, 3],
            hide_me: 2,
        }),
    };
    let borrowed_f = &f;
    let items = &*borrowed_f.get_items();

    assert_eq!(items, &vec![1, 2, 3]);
}


```

## 2.2 Lazy Initialization with `LazyCell<T, F>`

### 2.2.1 Basic Use

```rust

use std::sync::{LazyLock, Mutex};

static ARRAY: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(vec![]));

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}

```
