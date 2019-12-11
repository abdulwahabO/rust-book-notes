fn main() {

    // Array type indicates the type and number of elements.
    let names: [&str; 2] = ["Ade", "Bola"];

    // This will create an array of 5 elements each of which is the string "ade"
    let names = ["ade"; 5];

    // Access the fourth element in the array
    let name_a = names[3];

    // Each position in a tuple can have a different type
    let tup: (u32, String, u8) = (75u32, String::new(), b'f');


    // Another way to destructure tuples is by indexing
    let name = &tup.1;

    // One way to destructure tuples
    let (a,b,c) = tup;
    println!("{} {} {}", a, b, c);


    // Variables are immutable by default. `x` can not be assigned a value anymore
    let x = String::from("43");

    // Variables can be shadowed, i.e another variable of same name can declared and used
    // Types of both varibles can be different too. 
    // One good use case is when the String representation of a value is needed
    let x: u32 = "54".parse().expect("just open it");

    // Variables can be made mutable. `y` can be assigned a different value after this
    let mut y = 2.5;

    // Constants are always immutable and can only be set to a constant expression, function calls not allowed
    // The data type of a constant must always be annotated e.g u32 in this case
    const MAX_PTS: u32 = 232;

    // byte literal. Same as unsigned 8-bit integer
    let p : u8 = b'X';

    // Floating point type. f64 is usually preffered
    let z: f64 = 23.32;

    // Architecture dependent integer type. Primarily useful for indexing a collection.
    let xy: usize = 12;

    println!("Hello, world {}", x);

    add(21, 32);
}

// Constants can be declared in any scope. They are valid for the programs lifetime within the scope they are declared in
const MAX_LINKS: u32 = 21_000_000;

// Rust is an expression based language. Expressions return value, statements do not.
// Expressions can be part of statements.
// Expressions do not end with semicolons.
fn sum(x: u32, y:u32) -> u32 {

    let a = if x > y {
        23
    } else if x == y {
        36
    } else {
        12
    };

    a
}

fn add (mut a: u32, b: u32) {

    // Rust has three kinds of loops: `loop`, `while` and `for`
    
    // 1 - loop takes no condition, can only be halted with a break command
    loop {
        if a < b {
            println!("I won't stop!");
        }
        break
    } 

    // Break statement can be used to return value out of the loop
    
    let mut counter = 0;

    let d = loop {
        counter += 1;
        if counter > b {
            println!("counter greater than b");
            break b + counter; // Breaks and returns the value
        }
    };

    println!("Value in loop is {}", d);

    let p = while a > b {
        if b < 35 {
            break; // Can't break with a value inside `while`
        }
    };

    // Use a for-loop to iterate through collections
    // Ranges can also be used with for-loops as in a..b
    for n in 1..7 {
        println!("loop itn: {}", n);
    }

}
