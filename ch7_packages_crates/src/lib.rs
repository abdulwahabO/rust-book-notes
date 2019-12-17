
// This loads the contents of a module 'parser_util' from a file of same name.
mod parser_util; 

// The items in the module can be used as usual.
use super::parser_util::parser_one;

// Cargo conventions: This file is the crate root of a library crate with same name as this package
// i.e `ch7_packages_crates` is name of library crate
// The contents of this file form a module named `crate` which is at the root of the crate's module structure or 'module tree'

fn exm() {
    // This function belongs in a module named `crate` 
}

// All items are private by default. The 'pub' keyword is used to make them publicly accessible.
// Items in a child module can access private items in parent module.
// Items in a parent module cannot access private items in a child module.

pub mod util {

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

    // Parent modules can not use private items from their child modules.
    // Modules can be declared as public too. 
    // Making a module public does not automatically make it's items public too.
    pub mod string_util {

        pub fn end_with_slash() {

        }

        pub struct Record {

        }

        pub struct Scheme {

        }
    }

    mod currency_util {

        // Child modules can access private items in their parent modules but not in their siblings

        fn chec() {

            // Relative paths start with 'self', 'super' or an identifier in the current module
            super::general_util();

            // absolute path to a public member of sibling module.
            // absolute paths start with literal 'crate' or the name of the crate.
            crate::util::url_util::encode();

        }

        // This brings the function end_with_slash into scope. It can now be used without specifying it's path.
        // The idiomatic way is to bring the module containing the function into scope, that way it's clear the fnctn isn't local.
        use super::string_util::end_with_slash;
        use super::url_util;

        // When bringing in enums, structs and other items, it is idiomatic to specify the full path.
        // The exception is when two/more items from different modules have same name.
        use super::string_util::Record;

        // The 'as' keyword can be used to rename an item with a local name or alias.
        // This also solves the problem of two items with same name.
        use super::url_util::Scheme as UrlScheme;
        use super::string_util::Scheme as StringScheme;

        fn dump() -> UrlScheme {
            end_with_slash();


            let record: Record = Record {};

            // Idiomatic
            url_util::recode();

            UrlScheme {}
        }
    } 

    mod url_util {

        // Can be used in currency_util module.
        pub fn encode () {

        }

        pub fn recode() {

        }

        // Cannot be used in sibling module currency_util.
        fn decode() {

        }

        pub struct Scheme {

        }
    }

}
