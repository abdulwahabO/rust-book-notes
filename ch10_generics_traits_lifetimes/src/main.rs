

// There are no runtime costs to using generics. The Rust compiler performs monomorphization at compile time.
// i.e The compiler replaces all generic types with the concrete types the generic code was called with. 

fn main() {


}

fn return_middle<T>(list: &[T]) {
    // A function using generics.
}

struct ShapeContainer<S> {
    shape: S,
    volume: i32
}

// Generics can be used in method definitions
impl<S> ShapeContainer<S> {

    pub fn getShape(&self) -> &S {
        &self.shape
    }

    // Methods can define different generic types from those used by the struct.
    // Here `P` is only relevant to the method so it's defined in the method signature. 
    pub fn mixup<P>(&self, p: P) -> &S {
        
    }
 }

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Section 2: Traits -- Defining Shared Behaviour

pub struct Person {
    age: int,
    name: String
}

pub trait Summary {
    fn summarize(&self) -> String;

    pub fn default_summarize(&self) -> String {
        // a default implementation. Can call other methods in this trait.
    }
}

impl Summary for Person {
    fn summarize(&self) -> String {
        "This is a description of the person".to_owned();
    }

    // no need to implement `default_summarize()`
}

// To implement a trait defined elsewhere, it would have to be brought into scope with `use`.
// Trait implementations only work if either the type or the trait is local to the crate.

pub fn check_summary(item: impl Summary) {
    // A function that takes any type which implements Summary Trait.
    // This short form for a trait bound syntax works in simple straightforward cases.
}

// Generic types have trait bounds.

pub fn check_summary<T: Summary>(item: T) {
    // Function uses a trait bound syntax.
}

pub fn check_summary<T: Summary + Display>(item: T) {
    // More than one trait bound can also be used.
    // In this case `item` must be a type that implements both Summary and Display traits.
}

// Using a `where` clause can make a function/method with trait bounds less cluttered
pub fn check_summary<T, W>(item: T, wrap: W) -> String 
    where T: Summary + Display, W: Clone + Debug {

}

// Todo, trait implementations can also be function return types
fn check_summary() -> impl Summary {
    // returns a type that implements the Summary trait.
}

// Trait bounds can be used to conditionally implement methods.

struct Persistable<T, I> {
    id: I,
    data: T
}

impl<T, I> Persistable<T, I> {

    // This function and any methods defined in this block are always available to the struct.
    fn new(id: I, daa: T) -> Self {
        
    }
}

impl<T: PartialOrd, I> Persistable<T, I> {
    // Any methods defined here are only available if the `T` implements the PartialOrd trait.
}

// Section 10.3 -- Lifetimes.

// Rust's compiler uses a 'borrow checker' to determine that all borrows are valid.

fn exm() {

    let p;

    {
        let x = String::from("Hello!");
        p = &x;

        // The referent `x` is dropped at this point and the reference `p` becomes invalid.
    }

    // This will throw a compile time error since Rust doesn't allow null values and the string `p` had a reference to has been cleaned.
    println!("{}", p);


    let b = "dssd";  // start of lifetime 'a ----------------------

    {
        let c = "ghef"; 
        let v = longest(c, b);  // the longest function requires `c` to live as long as 'a, it doesn't so this fails.
    }

    // Will fail to compile since the lifetime of `v` is bigger than that of `c`.
    // Function could be returning a reference to `c` whic has gone out of scope at this point.
    print!("{}", v);

    // end of lifetime 'a -------------------------------
} 

// This compiler ensures that the reference returned by the function has a lifetime which is same as the smallest of the
// lifetimes of the references passed in.
fn longest<'a, T, U>(x: &'a str, y: &'a str, p: T, w: U) -> &'a str 
    where T: Display + Default, U: Clone {

    // generic types, trait bounds and liftimes all used in this function

    if 1 < 10 {
        y
    } else {
        x
    }

}

// lifetime annotations must be used to annotate structs that take references.
// This means an instance of this struct can't outlive any of the references it fields hold.
struct BookExcerpt<'a> {
    excerpt: &'a str
}

fn strct() -> {

    // Good: The struct instance lives long enough for the reference to be valid.
    let excpt = ".....Some part of a book...";
    let mut bk_expt = BookExcerpt { excerpt: excpt};

    let bk_expt_2;

    {
        let excpt = "..hello..";
        bk_expt_2 = BookExcerpt {excerpt: excpt};
    }

    // Bad: The struct instance is outliving a reference used by it's field. Won't compile.
    let vv = &bk_expt_2;

}




