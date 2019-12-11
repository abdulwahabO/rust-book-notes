

// Cargo conventions: This file is the crate root of a library crate with same name as this package
// i.e `ch7_packages_crates` is name of library crate
// The contents of this file form a module named `crate` which is at the root of the crate's module structure or 'module tree'

fn exm() {
    // This function belongs in a module named `crate` 
}

mod util {

    // Even if a struct is public, it's private fields cannot be accessed from outside the module
    // This means new instances of this struct cannot be created outside this module without an associated function
    pub struct Person {
        pub name: String,
        age: u32
    }

    // All variants of a public enum are public
    pub enum Status {

    }

    // This module is nested inside the crate root 
    fn general_util() {

    }

    // Parent modules can not use private items in their child modules
    // Modules can be declared as public too. 
    // Making a module public does not automatically make it's items public too.
    mod string_util {
        // This function is in crate --> util --> string_util
        pub fn end_with_slash() {

        }
    }

    mod currency_util {
        // Child modules can access private items in their parent modules but not in their siblings

        fn chec() {
            super::general_util();

            // absolute path to a public member of sibling module
            crate::util::url_util::encode();

        }
    } 

    mod url_util {
        pub fn encode () {

        }
    }

}
