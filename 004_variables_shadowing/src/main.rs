fn main() {
    // Shadowing: re-declaring `x` with `let` creates a new binding.
    let x = 5;
    let x = x + 1;

    {
        // Inner-scope shadowing is confined to this block.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Unlike `mut`, shadowing also allows the type to change.
    let spaces = "     ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
