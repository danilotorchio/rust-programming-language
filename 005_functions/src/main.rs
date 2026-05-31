fn main() {
    print_number(10);

    let result = plus_one(10);
    println!("Value plus one: {result}");

    // Blocks are expressions: the final line (no semicolon) is the value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Parameters require explicit type annotations.
fn print_number(x: i32) {
    println!("The value of x is: {x}");
}

// Return type after `->`; the tail expression (no semicolon) is returned.
fn plus_one(value: i32) -> i32 {
    value + 1
}
