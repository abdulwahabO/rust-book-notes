
fn return_first_word(words: &str) -> String {
   words.split_whitespace().next().unwrap().to_owned()
}

fn return_err() -> Result<(), String> {
    Result::Err("Using String as error type, hahahaha!".to_owned())
}

#[cfg(test)]
mod tests {

    // `tests` is just like any other module and conforms to te usual visibility rules.
    use super::{return_first_word, return_err};


    #[test]
    // This is a test function.
    fn it_works() {

        // The assert** macros provided by std lib are useful for checking that some condition is met.
        assert_eq!(2 + 2, 4);
    }

    fn non_test() {
        // test module can also contain regular functions which are not tests.
    }

    #[test]
    fn should_return_first_word() {

        let word = return_first_word("Wind at seas");

        // The order in which the values are passed to the macro doesn't matter.
        // For this macro to work for custom enums and structs, they to implment `Debug` and `PartialEq`.
        // This is easily done by using #[derive(Debug, PartialEq)] on the definitions.
        assert_eq!(word, "Wind".to_owned());

        let st = "something something";

        // The assert macro can also take custom failure messages.
        // Underneath, anything after the two values to be checked is passed to format! macro.
        assert_eq!(word, "Wind".to_owned(), "This is an optional custom failure msg with a placeholder {}", st);
    }

    #[test]
    #[should_panic] // The test only passes if the code within panics.
    fn check_panic() {
        panic!("critical failure!!");
    }

    #[test]
    fn retn_err() -> Result<(), String> {

        // Test can return a Result::Err when it fails, instead of panicking.
        // #[should_panic] cannot be used on tests with this return type.
        // This make it easier to use the ? operator within test body.

        
        // The test fails if an Err is returned by the function.
        let v = return_err()?;

        Ok(v)
    }


    // Controlling how tests are run.

    // By default, Tests are run in parrallel using threads. So it's important tests don't share any state or depend
    // on each other in anyway.
    // The number of threads used can be controlled via --threads flag passed to `cargo test`
    // --threads=1 means no parrellism will be used.

    // By default, Rust's test library captures anything printed to std_out if a test passes. So calling println!
    // within a test will have no effect unless the test fails. 
    // to prevent this use `--nocapture` with `cargo test`

    // A test of subset of tests can be run by passing their name or part of their name to `cargo test`
    // e.g `cargo test retn_err` will run that test alone. `cargo test retn` will run that test and any whose
    // name start with `retn`. Also, the name of the module containing the test can be used.

    #[test]
    #[ignore] // This test won't run unless specifically requested using the `-- --ignored` flag
    fn ignored_test() {

    }

    // Note: While passing command options to `cargo test`, some are used by the command itself and others go
    // to the resulting test binaries. Use -- to separate the two, 
    // e.g `cargo test -- --threads=1`. --threads=1 is passed to the test binary, while any commands before -- are
    // used by `cargo test` itself e.g the name of a test.

}

// Test Organization

// Unit tests are put in the same file as the code they are testing. The convention is to use a module named 'tests'
// which is annotated with `#[cfg(test)]` - this ensures the module is only compied when `cargo test` is run and 
// is ignored for `cargo build`.alloc

// Rust's visibility rules allow private functions to be unit tested since test modules are just like any other.

// Integration Tests are external to a library and test it's public API -- 
// By Cargo conventions, they are kept in a directory /tests next to /src --
// Each Rust file in /tests is a separate crate --
