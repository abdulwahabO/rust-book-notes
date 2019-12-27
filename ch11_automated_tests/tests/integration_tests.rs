
// This file like any other in /tests is an individual crate.
// To test the library crate in /src, it has to be brought into scope.
use ch11_automated_tests;

#[test]
fn tst_fn_from_lib() {
    // test a function from lib crate
}

#[test]
fn tst_another() -> Result<(), String> {
    Ok(())
}

#[test]
#[should_panic]
fn test_3() {
    panic!("test passes");
}

// `cargo test --test [name of integration test file]` will run all tests in the specified file

// Integration test crates like this one can have modules within them, though they behave a bit differently from
// regular modules -- see section 11.3 of TRPL for more info.

// Integration tests cannot be written for binary crates only libraries.
