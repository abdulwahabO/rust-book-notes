fn main() {
    println!("Hello, world!");

    let a = String::from("Halo");

    // The pointer to the heap space containing the String is moved from a to b.
    // a is now invalid after this move. String is not a copy type.
    let b = a;

    // Deep copy of heap data. Expensive operation. Creates a totally new value and owner.
    // `b` remains valid and still points to a heap buffer containing "Halo"
    let c = b.clone();

    println!("c = {}, b = {}", c , b);

    print!("{}", a); // Won't compile! `a` is now invalid.

    let d = add(String::from("Kai"));

    // Only one type of reference to a variable can exist in a particular scope.
    // Mutable and non-mutable references to a variable cannot exist at the same time.
    // This is so Rust can prevent data races at compile time.
    remove(&d);

    {
        // A reference's scope starts from where it is introduced to the last time it was used
        // Curly braces create a new scope. So they can be used when more references than is allowed are needed
    }

    let mut bx = String::from("Helloqq");
}

fn add(name: String) -> String {
    // `name` must be owned and the function will take ownership when it executes.
    // After name exits, the value is dropped

    // Transfers ownership of this to the caller
    let a = String::from("Hello Rust");
    a
}

fn give() -> (i32, String) {
    // Functions can return tuples also.
    (76i32, String::from("How re you"))
}

fn remove(id: &String) {
    // Takes a shared reference (non-mutable) to an owned String type.
    // Doesn't take ownership of `id`

    // The reference gets 

    // `*` asterisk character is used to dereference but it can be omitted most of the time
    // In some scenarios like calling methods or passing args, Rust deferences automatically
    println!("The reference holds: {}", *id);

    // Even references can be referenced !!
    // This cannot be moved because the `id` reference doesn't own the heap data and the referent is not a Copy type 
    let dd = &*id;

    println!("dd == {}", dd);
}

fn delete(id: &mut String) {
    // Takes a mutually exclusive reference(mutable) to an owned String
    // Drop is not called at end of the function. Since `id` doesn't directly own any head data 
}

fn check() {

    let mut s = String::from("Hey-you");

    // Scope of reference starts from where they are defined to where they are last used

    let b = &s; // Scope of the non-mutable reference `b` starts here

    // No mutable reference to `s` can exist in this gap

    println!("{}", b); // Scope of non-mutable reference `b` ends here

    let a = &mut s; // Mutable reference to `s` can now exist since the other has gone out of scope
}

// fn dng() -> &String {

    // Once a function is done executing, it's values are popped off the stack and `a` will be deallocated
   // let a = String::from("I have a dream");
 
    //let b : &String = &a;

    // b

    // `a` goes out of scope but a reference to it is returned. This is known as a dangling pointer
    // The Rust compiler will reject this code
    // References cannot outlive their referent
// }

fn slices() {

    let a = String::from("Hello Good Morning");

    let b = a.as_bytes();

    // Uses pattern (x, y) to destructure the tuple returned by the enumerate() method
    for (i, &item) in b.iter().enumerate() {
        println!("{}", item);
    }

    for tup in b.iter().enumerate() {
        // The elements in the tuple returned by enumerate can also be accessed via indexing
        println!("{}", tup.1);
        println!("{}", &tup.0);
    }

    // NOTE: All these examples assume characters in `a` are ASCII which uses a byte per character.
    // Slicing a String in the middle of a multibyte character is problematic.
    // More robust String slicing is discussed in later chapters

    // A String slice stores a pointer to the starting index of the slice and the length of the slice
    // In the this case `c` contains a pointer to the 5th byte of `a` and the length of the slice: 6
    let c = &a[4..9];

    // This slices starts from the first byte to the 6th
    // The range syntax .. allows the first index to be dropped
    let d = &a[..5];

    let length = a.len();

    // These slices start from the fourth byte to the last
    let e = &a[3..];
    let e1 = &a[3..length];

    // A slice of the entire string. `&str` is the type of String slices
    let f : &str = &a[..];

    // String slices don't have ownership and actually borrow from the String.
    // They contain non-owning references/pointer to String data, so the rules of ownership & refereces apply

    // This is a string literal which has same type as a string slice
    let zz : &str = "A literal!";

    // String literals and slices are immutable! even if the String is.

}
