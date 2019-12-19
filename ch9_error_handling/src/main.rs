use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {

}

fn end_program() {
    // When a panic occurs, by default Rust walks back up the stack and cleans data from each function.

    // However, it is possible to get the program to abort immediately after a panic by using `panic = 'abort'` in Cargo.toml
    // This is useful in making release binaries smaller.
    // By setting the `RUST_BACKTRACE` env variable to 1 or anything other than 0, we get a backtrace of all functin calls leading up to the error.
    panic!("Crash!!");
}

fn handle_result(val: u32) {

    // Returns Result::OK(with file handle) if the call succeeds. 
    // Returns Result::Err(with error type) if the call fails.
    let file_handle = File::open("hello.rs");

    // Overshadows previous declaration of `file_handle`
    // Return the handle if ok, pass any errors to the panic! macro.
    let file_handle = match file_handle {
        Ok(handle) => handle,
        Err(error) => {
            panic("Something went wrong", error);
        }
    }

    let another_file = File::open("copy.rs");

    let another_file = match another_file {
        Ok(file) => file, // returns the file handle if all is good.

        // Get the io::ErrorKind and react to different kinds of error
        Err(error) => match error.kind() {

            ErrorKind::NotFound => {
                // Handle not found in some way, probably by creating the file.
            },

            other_errors => {
                // Do something for all other error kinds, probably panic.
            }
        }
    }

    // Closures offer a cleaner way to handle Result::Err
    // Result Enum has many methods that take closures as argument
    let another_file = File::open("types.rs").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // Do something
        } else {
            // Do another thing
        }
    });

    let another_file_3 = File::open("hello.tct");

    // Result has many helper functions defined on it that eliminate the need for match expressions.
    // `unwrap()` returns Ok(T) or panics.
    another_file_3.unwrap();

    // `expect(..)` returns Ok(T) or panics with the given message
    another_file_3.expect("The operation failed");

}


// PROPAGATING ERRORS
// Errors can be propagated to the calling code, so it decides how to handle them.
// The ? can automatically convert from one error type to another.
// If the error received is different from the error type defined by the function it is converted.....
// .....as long as the error type received implements the `from()` function defined by the `From` trait.
fn return_error() -> Result<String, Error> {

    // Any io::Error is propagated to the caller.
    let file = File::open("open_file.txt")?;

    let txt = String::with_capacity(26);

    file.read_to_string(&mut txt)?;

    // Can be rewritten as:
    File::open("file.txt")?.read_to_string(&mut txt)?;
}
