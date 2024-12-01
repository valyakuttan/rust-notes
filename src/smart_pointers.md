# 2. Smart Pointers

## 2.1 Simple Heap Allocation with Box `Box<T>`

### 2.1.1 Basic Usage

```rust

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

```

## 2.2 Single-threaded Reference-counting Pointers `Rc<T>`

### 2.2.1 Synced Data Structure

```rust

#[derive(Clone, Debug)]
pub struct Guest {
    name: String,
    address: String,
    // ... many other fields
}

/// Local error type, used later.
#[derive(Clone, Debug)]
pub struct Error(String);
impl Error {
    fn new(msg: &str) -> Self {
        Error(msg.to_string())
    }
}

mod rc {
    use super::{Error, Guest};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default, Debug)]
    pub struct GuestRegister {
        by_arrival: Vec<Rc<RefCell<Guest>>>,
        by_name: std::collections::BTreeMap<String, Rc<RefCell<Guest>>>,
    }

    impl GuestRegister {
        // pub fn named(&self, name: &str) -> Option<Guest> {
        //     self.by_name.get(name).map(|rg| rg.borrow().clone())
        // }

        pub fn named<'a>(&'a self, name: &str) -> Option<Guest> {
            let idx = self.by_name.get(name)?;
            let rg = (*idx.borrow()).clone();
            Some(rg)
        }

        pub fn register(&mut self, guest: Guest) {
            let name = guest.name.clone();
            let guest = Rc::new(RefCell::new(guest));
            self.by_arrival.push(guest.clone());
            self.by_name.insert(name, guest);
        }

        pub fn deregister(&mut self, idx: usize) -> Result<(), Error> {
            if idx >= self.by_arrival.len() {
                return Err(Error::new("out of bounds"));
            }
            let guest: Rc<RefCell<Guest>> = self.by_arrival.remove(idx);
            self.by_name.remove(&guest.borrow().name);
            Ok(())
        }
        // ...
    }
}

use rc::GuestRegister;
pub fn main() {
    let mut ledger = GuestRegister::default();
    let alice = Guest {
        name: "Alice".to_string(),
        address: "Alice's addr".to_string(),
    };
    let bob = Guest {
        name: "Bob".to_string(),
        address: "Bob's addr".to_string(),
    };
    let charlie = Guest {
        name: "Charlie".to_string(),
        address: "Charlie's addr".to_string(),
    };

    ledger.register(alice);
    ledger.register(bob);
    ledger.register(charlie);

    println!("Register starts as:");
    println!("");
    println!("{ledger:?}");

    ledger.deregister(0).unwrap();
    println!("Register after deregister(0)");
    println!("");
    println!("{ledger:?}");

    let also_alice = ledger.named("Alice");
    // Alice still has index 0, which is now Bob
    println!("");
    println!("Alice is {also_alice:?}");

    let also_bob = ledger.named("Bob");
    // Bob still has index 1, which is now Charlie
    println!("");
    println!("Bob is {also_bob:?}");

    let also_charlie = ledger.named("Charlie");
    // Charlie still has index 2, which is now beyond the Vec
    println!("");
    println!("Charlie is {also_charlie:?}");
}

```

### 2.2.2 Tree with Owner Pointer

```rust

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Default)]
struct TreeId(String);

impl TreeId {
    fn new(id: &str) -> Self {
        TreeId(id.to_string())
    }
}

#[derive(Debug, Default)]
struct BranchId(String);
impl BranchId {
    fn new(id: &str) -> Self {
        BranchId(id.to_string())
    }
}

#[derive(Debug, Default)]
struct LeafId(String);
impl LeafId {
    fn new(id: &str) -> Self {
        LeafId(id.to_string())
    }
}
#[derive(Debug, Default)]
struct Tree {
    id: TreeId,
    branches: Vec<Rc<RefCell<Branch>>>,
}

impl Tree {
    fn new(id: &str) -> Self {
        Tree {
            id: TreeId::new(id),
            ..Default::default()
        }
    }
}
#[derive(Debug, Default)]
struct Branch {
    id: BranchId,
    leaves: Vec<Rc<RefCell<Leaf>>>,
    owner: Option<Weak<RefCell<Tree>>>,
}

#[derive(Debug, Default)]
struct Leaf {
    id: LeafId,
    owner: Option<Weak<RefCell<Branch>>>,
}

impl Leaf {
    fn new(id: &str) -> Self {
        Leaf {
            id: LeafId::new(id),
            ..Default::default()
        }
    }
}

impl Branch {
    fn new(id: &str) -> Self {
        Branch {
            id: BranchId::new(id),
            ..Default::default()
        }
    }

    fn add_leaf(branch: Rc<RefCell<Branch>>, mut leaf: Leaf) {
        leaf.owner = Some(Rc::downgrade(&branch));
        branch.borrow_mut().leaves.push(Rc::new(RefCell::new(leaf)));
    }

    fn location(&self) -> String {
        match &self.owner {
            None => format!("<unowned>.{}", self.id.0),
            Some(owner) => {
                // Upgrade weak owner pointer.
                let tree = owner.upgrade().expect("owner gone!");
                format!("{}.{}", tree.borrow().id.0, self.id.0)
            }
        }
    }
}

fn main() {
    let t = Tree::new("root");
    let left = Branch::new("left");
}

```

## 2.3 Non Owning Reference with `Weak<T>`

```rust

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// Use a newtype for each identifier type.
struct TreeId(String);
struct BranchId(String);
struct LeafId(String);

struct Tree {
    id: TreeId,
    branches: Vec<Rc<RefCell<Branch>>>,
}

struct Branch {
    id: BranchId,
    leaves: Vec<Rc<RefCell<Leaf>>>,
    owner: Option<Weak<RefCell<Tree>>>,
}

struct Leaf {
    id: LeafId,
    owner: Option<Weak<RefCell<Branch>>>,
}

impl Branch {
    fn add_leaf(branch: Rc<RefCell<Branch>>, mut leaf: Leaf) {
        leaf.owner = Some(Rc::downgrade(&branch));
        branch.borrow_mut().leaves.push(Rc::new(RefCell::new(leaf)));
    }

    fn location(&self) -> String {
        match &self.owner {
            None => format!("<unowned>.{}", self.id.0),
            Some(owner) => {
                // Upgrade weak owner pointer.
                let tree = owner.upgrade().expect("owner gone!");
                format!("{}.{}", tree.borrow().id.0, self.id.0)
            }
        }
    }
}

pub fn main() {
    
}

```

## 2.4 The Peril of Reference Cycles

```rust

use std::cell::{Ref, RefCell};
use std::mem::{replace, take};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn head(&self) -> Option<i32> {
        match self {
            Cons(v, _) => Some(*v),
            _ => None,
        }
    }

    fn tail(&self) -> Option<impl Deref<Target = Rc<List>> + '_> {
        match self {
            Cons(_, item) => Some(item.borrow()),
            Nil => None,
        }
    }

    fn tail_mut(&self) -> Option<impl DerefMut<Target = Rc<List>> + '_> {
        match self {
            Cons(_, item) => Some(item.borrow_mut()),
            Nil => None,
        }
    }
}

pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("nil: {:?} {}", nil, Rc::strong_count(&nil));
    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    if let Some(mut link) = a.tail_mut() {
        *link = Rc::clone(&b);
    };

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}


```

## 2.5 The Problem of Occational Mutation and `Cow<'_, B>`

```rust
/// The problem with occasional mutation
/// ====================================
///
/// Consider a case where you have a Vec of some elements. For the
/// purposes of this example, consider that an `Element` has an `ID`
/// that can uniquely identify it, and is large enough in size that
/// duplicating it would require considerable memory.
///
/// In this case, we need to make sure the vector only has unique
/// elements and does not contain any duplicates. To ensure this,
/// we have a function that can filter the `Vec`.
///
/// The code below demonstrates a simple way to implement this function:
///
/// ```
///
/// fn filter_unique(input:Vec<Element>)->Vec<Element>{
/// let mut seen_ids = HashSet::new();
/// let mut ret = Vec::new();
/// for element in input{
/// if seen_ids.contains(&element.id){
/// continue;
/// }
///    seen_ids.insert(element.id);
/// ret.push(element);
/// }
/// ret
/// }
///
/// ```
///
/// In situations where duplicates are common, this implementation is
/// pretty efficient. But what if they are not so common?
///
/// When the input only infrequently contains duplicates, we would be
/// creating the needless temporary `Vec` far more often than actually
/// necessary. As a result, we would also be unnecessarily allocating
/// that much extra memory and spending CPU time copying the elements
/// over from the original to the new.
///
/// As we know, an `Element` is big in size, so copying it takes
/// considerable memory. We also need to allocate and grow that much
/// memory for the returned `Vec`.
///
/// Even if we took the input by value, not as a reference — in which
/// case we would not have an overhead of calling clone on the elements
/// — we would still need to allocate extra memory for the returned `Vec`
/// and then free up the original after we are done copying.
/// This seems wasteful, considering that when there are no duplicates,
/// we are essentially simply returning the input array.
///
/// Cases like this involve performing some costly operation on the input,
/// but only when a certain condition is true. When that condition is so
/// rare that the output is simply the input most of the time, `Cow` can
/// help us to avoid the cost of the operation.
///
use std::{borrow::Cow, collections::HashSet, ops::Deref};

#[derive(Clone)]

struct Element {
    id: usize,
    //...
}

fn get_unique(input: &[Element]) -> Vec<Element> {
    let mut set = HashSet::new();
    let mut ret = Vec::new();
    for element in input {
        if set.contains(&element.id) {
            continue;
        } else {
            ret.push(element.clone());
            set.insert(element.id);
        }
    }
    ret
}

fn get_unique_cow<'a>(input: &'a [Element]) -> Cow<'a, [Element]> {
    let mut set = HashSet::new();
    let mut contains_duplicate = false;

    for element in input {
        if set.contains(&element.id) {
            contains_duplicate = true;
        }
        set.insert(element.id);
    }

    match contains_duplicate {
        false => Cow::Borrowed(input),
        _ => {
            let mut ret = Vec::new();
            for element in input {
                if set.contains(&element.id) {
                    ret.push(element.to_owned());
                    set.remove(&element.id);
                }
                // duplicate
            }
            Cow::Owned(ret)
        }
    }
}

fn main() {
    let elms1: Vec<_> = (1..10).map(|id| Element { id }).collect();
    let cow1: &[Element] = get_unique_cow(&elms1).deref(); // borrowed

    let mut elms = elms1.clone();
    elms.extend_from_slice(&elms1);

    let cow2 = get_unique_cow(&elms).deref(); // cloned
}

```