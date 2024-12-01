# 5. Rust Macros

## 5.1. Introduction

To define a macro, you use the `macro_rules!` construct. Let’s
explore how to use `macro_rules!` by looking at how the `vec!`
macro is defined. You can call the macro as `vec[1, 2]` or
`vec(1, 2)` or as `vec{1, 2}`. Sometimes you may need to insert
`;` after the macro call.

```rust

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
     let v1 = vec![1, 2]; // ok
    let v2 = vec!(1, 2); // ok
    let v3 = vec!{1, 2}; // ok

   println!("{:?} {:?} {:?}",v1, v2, v3);
}

```

The `#[macro_export]` annotation indicates that this macro should
be made available whenever the crate in which the macro is defined
is brought into scope. Without this annotation, the macro can’t
be brought into scope. The arguments of a macro are prefixed by
a dollar sign `$` and type annotated with a designator:

## 5.2. Repeat

Macros can use `+` in the argument list to indicate that an argument
may repeat at least once, or `*`, to indicate that the argument may
repeat zero or more times. Also we can use `?` to indicate the
occurrence of at most one repetition.

```rust

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}

```

Let's define a macro which can offer functionality like `vec!`
for `HasMap<K, V>`, which uses `?`, to accept an optional
trailing `,` in the argument list.


```rust

#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($k:expr => $v:expr),+ $(,)?) => {
        {
          let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($k, $v);
         )*

        map
        }
    };
}

fn main() {
    let h1 = hashmap![1=>2, 2=>4];
    let h2 = hashmap![1=>2, 2=>4,];

    println!("{:?} {:?}",h1, h2);
}


```
