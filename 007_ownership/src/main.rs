fn main() {
    // Ownership moves from the callee back to `s`.
    let s = give_ownership();
    // Ownership of `s` moves into the function; `s` is no longer usable here.
    takes_ownership(s);

    // Move in, move back out.
    let s1 = String::from("Hello");
    let s2 = takes_and_gives_back(s1);
    println!("{s2}");

    // Scalar types implement `Copy`, so `x` stays valid after the call.
    let x = 5;
    makes_copy(x);

    // Returning a tuple is one way to hand ownership back along with a result.
    let s3 = String::from("Hello, world!");
    let (s4, len) = calculate_length(s3);

    println!("The lenght of string {s4} is {len}");

    // Borrowing with `&` lets the callee read without taking ownership.
    let s5 = String::from("Hello, world!!");
    let len = calculate_length_borrow(&s5);
    println!("The lenght of string {s5} is {len}");

    // `&mut` allows in-place mutation; the owner must also be `mut`.
    let mut s = String::from("Hello");
    change(&mut s);

    println!("{s}");

    // Many shared `&` refs may coexist; their scopes end at last use,
    // so a later `&mut` is allowed once the shared refs are done.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");
}

// Receiving a `String` takes ownership; it's dropped at end of scope.
fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

// Returning a value transfers ownership to the caller.
fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// `i32` is `Copy`: no ownership transfer happens.
fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Takes a shared reference; the caller keeps ownership.
fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

// `&mut` lets the function mutate the value through the reference.
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
