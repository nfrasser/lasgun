Rust Ownership
===

Unlike C++, Rust doesn't automatically copy heap-allocated data for you. Instead
it will "move" the data and invalidate previous references to it.

```rs
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // error[E0382]: use of moved value: `s1`
```

When you pass a heap-allocated variable into a function, the function takes ownership of it and the compiler prevents you from using it again.

```rs
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function...
    println!("{}", s) // error[E0382]: use of moved value: `s`

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope, memory is freed.
}
```

You can return a variable to move it back into the scope of the calling
function.

## References

Rust has references similar to C++. Passing something in as a reference means
the calling scope doesn't have ownership of the original data.

### Immutable references
```rs
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable References

```rs
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 'Borrow' Rules for references
- can have as many immutable references
- can have one mutable reference (in one scope)
- cannot have a mutable reference if an immutable one exists
- can't return a references to a local value. Must return the value itself (transfer ownership)

```rs
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
```

```rs
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

More information at: https://doc.rust-lang.org/stable/book/2018-edition/ch04-00-understanding-ownership.html
