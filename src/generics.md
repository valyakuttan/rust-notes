# Chapter 4 Generics and Trait Objects

## 4.1 Trait Objects

```rust

struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
  
    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}

```

## 4.2 Rust Generics

```rust

use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Point(i64, i64);

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

/// Calculate the overlap between two rectangles, or `None` if there is no
/// overlap.
fn overlap(a: Bounds, b: Bounds) -> Option<Bounds> {
    None
}

/// Trait for objects that can be drawn graphically.
pub trait Draw {
    /// Return the bounding rectangle that encompasses the object.
    fn bounds(&self) -> Bounds;

    // ...
}

#[derive(Clone)] // no `Debug`
struct Square {
    top_left: Point,
    size: i64,
}

impl Draw for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: self.top_left,
            bottom_right: Point(self.top_left.0 + self.size, self.top_left.0 + self.size),
        }
    }
}

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: i64,
}

impl Draw for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point(self.center.0 - self.radius, self.center.1 - self.radius),
            bottom_right: Point(self.center.0 + self.radius, self.center.1 + self.radius),
        }
    }
}

static SCREEN_BOUNDS: Bounds = Bounds {
    top_left: Point(0, 0),
    bottom_right: Point(640, 480),
};

/// Generics
/// ========
///
/// the programmer writes a single generic function, but the compiler outputs
/// a different monomorphized version of that function for every different
/// type that the function is invoked with.
///
/// An important advantage of generic trait bounds is that it can be used to
/// conditionally make different functionality available, depending on
/// whether the type parameter implements multiple traits.
///
/// Implementing the same functionality with trait objects most often leads to
/// awkward looking code.

// The `area` function is available for all containers holding things
// that implement `Draw`.
fn area<T>(draw: &T) -> i64
where
    T: Draw,
{
    let bounds = draw.bounds();
    (bounds.bottom_right.0 - bounds.top_left.0) * (bounds.bottom_right.1 - bounds.top_left.1)
}

// The `show` method is available only if `Debug` is also implemented.
fn show<T>(draw: &T)
where
    T: Debug + Draw,
{
    println!("{:?} has bounds {:?}", draw, draw.bounds());
}

pub fn on_screen_genric(draw: &impl Draw) -> bool {
    // this function signature is the pretty version of
    //
    // pub fn on_screen_genric<T: Draw>(draw: &T) -> bool {}
    //

    overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}

/// Trait Objects
/// =============
///
/// In comparison, trait objects are fat pointers that combine a pointer to
/// the underlying concrete item with a pointer to a vtable that in turn
/// holds function pointers for all of the trait implementation's methods.
///
/// This means that a function that accepts a trait object doesn't need to
/// be generic and doesn't need monomorphization: the programmer writes a
/// function using trait objects, and the compiler outputs only a single
/// version of that function, which can accept trait objects that come
/// from multiple input types:

pub fn on_screen_trait_object(draw: &dyn Draw) -> bool {
    overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}

fn main() {
    let square = Square {
        top_left: Point(1, 2),
        size: 2,
    };

    let circle = Circle {
        center: Point(3, 4),
        radius: 1,
    };

    //==========================================================================
    //
    // using generic bounds
    //
    //==========================================================================

    //
    // Calls `on_screen::<Square>(&Square) -> bool`
    let visible = on_screen_genric(&square);
    assert!(!visible);

    // Calls `on_screen::<Circle>(&Circle) -> bool`
    let visible = on_screen_genric(&circle);
    assert!(!visible);

    // Both `Square` and `Circle` implement `Draw`.
    println!("area(square) = {}", area(&square));
    println!("area(circle) = {}", area(&circle));

    // `Circle` implements `Debug`.
    show(&circle);

    // `Square` does not implement `Debug`, so this wouldn't compile:
    // show(&square);

    //==========================================================================
    //
    // using trait objects
    //
    //==========================================================================

    // Calls `on_screen(&dyn Draw) -> bool`.
    let visible = on_screen_trait_object(&square);
    assert!(!visible);

    // Also calls `on_screen(&dyn Draw) -> bool`.
    let visible = on_screen_trait_object(&circle);
    assert!(!visible);
}

```

## 4.3 Trait Objects as Genrics

```rust

use std::fmt::Debug;

fn main() {
    let b = return_trait_using_trait_objects(false);
    println!("{:?}",b.bounds())
}

/// Returning Trait objects with `dyn Trait`
///
///
/// The Rust compiler needs to know how much space a function's return type
/// requires. Because the different implementations of a trait probably
/// uses different amounts of memory, functions need to either return a
/// concrete type or the same type when using impl Trait, or return
/// a trait object with `dyn`
fn return_trait_using_trait_objects(square: bool) -> Box<dyn Draw> {
    if square {
        Box::new(Square {
            top_left: Point(0, 0),
            size: 10,
        })
    } else {
        Box::new(Circle {
            center: Point(0, 0),
            radius: 5,
        })
    }
}

/// Returning Traits with `impl Trait`
/// ===========================
///
/// Because of monomorphization step of generic functions
/// we can't return different types by giving the return
/// type as `impl Trait`.
///
fn return_trait_using_generics<T>(square: bool) -> impl Draw {
    if square {
        Square {
            top_left: Point(0, 0),
            size: 10,
        }
    } else {
        // not possible becasuse of monomorphization
        //
        // Circle {
        //     center: Point(0, 0),
        //     radius: 5,
        // }
        Square {
            top_left: Point(20, 30),
            size: 10,
        }
    }
}
/// Trait Object Safety
/// ===================
///
/// Another restriction on trait objects is the requirement for object
/// safety: only traits that comply with the following two rules can be
/// used as trait objects:
/// 1. The trait must not contain any generic type parameters.
/// 2. The trait's methods must not involve a type that includes
///    Self, except for the receiver (the object on which the
///    method is invoked).
///
/// There is an exception to this second restriction. A method returning
/// some Self-related type does not affect object safety if Self comes
/// with an explicit restriction to types whose size is known at compile
/// time, indicated by the Sized marker trait as a trait bound.
///
/// A `Stamp` can be copied and drawn multiple times.
trait Stamp: Draw {
    fn make_copy(&self) -> Self
    where
        Self: Sized;
}

impl Stamp for Square {
    fn make_copy(&self) -> Self {
        self.clone()
    }
}
/// ```
///
/// let square = Square {
/// top_left: Point(1, 2),
/// size: 2,
/// };
///
/// // `Square` implements `Stamp`, so it can call `make_copy()`.
/// let copy = square.make_copy();
///
/// // Because the `Self`-returning method has a `Sized` trait bound,
/// // creating a `Stamp` trait object is possible.
/// let stamp: &dyn Stamp = &square;
///
/// ```
///
/// The `Sized` trait bound means that the `make_copy` method can't be used
/// with trait objects anyway, because trait objects refer to something
/// that's of unknown size (dyn Trait), and so the method is irrelevant
/// for object safety.
///
/// ```
///
/// // However, the method can't be invoked via a trait object.
/// let copy = stamp.make_copy();
///
/// ```
///
/// ```
///
/// error: the `make_copy` method cannot be invoked on a trait object
///     --> src/main.rs:397:22
///     |
/// 353 |         Self: Sized;
///     |               ----- this has a `Sized` requirement
/// ...
/// 397 |     let copy = stamp.make_copy();
///     |                      ^^^^^^^^^
///
/// ```

/// More Trait Bounds
/// =================
///
/// In addition to using trait bounds to restrict what type parameters
/// are acceptable for a generic function, you can also apply them
/// to trait definitions themselves.
///
/// Anything that implements `Shape` must also implement `Draw`.
trait Shape: Draw {
    /// Render that portion of the shape that falls within `bounds`.
    fn render_in(&self, bounds: Bounds);

    /// Render the shape.
    fn render(&self) {
        // Default implementation renders that portion of the shape
        // that falls within the screen area.
        if let Some(visible) = overlap(SCREEN_BOUNDS, self.bounds()) {
            self.render_in(visible);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point(i64, i64);

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

/// Calculate the overlap between two rectangles, or `None` if there is no
/// overlap.
fn overlap(_: Bounds, _: Bounds) -> Option<Bounds> {
    None
}

/// Trait for objects that can be drawn graphically.
pub trait Draw {
    /// Return the bounding rectangle that encompasses the object.
    fn bounds(&self) -> Bounds;
}

#[derive(Clone)] // no `Debug`
struct Square {
    top_left: Point,
    size: i64,
}

impl Draw for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: self.top_left,
            bottom_right: Point(self.top_left.0 + self.size, self.top_left.0 + self.size),
        }
    }
}

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: i64,
}

impl Draw for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point(self.center.0 - self.radius, self.center.1 - self.radius),
            bottom_right: Point(self.center.0 + self.radius, self.center.1 + self.radius),
        }
    }
}

static SCREEN_BOUNDS: Bounds = Bounds {
    top_left: Point(0, 0),
    bottom_right: Point(640, 480),
};


```
