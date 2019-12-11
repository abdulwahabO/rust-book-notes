fn main() {
    
    let p = Sale {
        tax_type: Tax::INCOME(21)
    };
    
    let b : Option<String> = None;

    // Match expression take ownership of values
    match &b {
        Some(u) => println!("Yaaaay {}", u),
        None => {
            // Curly braces are needed to execute more than a line of code 
        }
    }

    let z = String::from("He");

    // Match arms can return values
    // Match arms consist of a pattern and code to run separated by =>
    // The value of the Option if present, can be bound to the pattern in the match expression
    let d =  match b {
        None => Some(z),
        Some(p) => Some(p)
    };

    // if-let is used when only one pattern is important to check the value against
    if let Tax::SALES(x) = p.tax_type {
        println!("The tax type is sales");
    } else {
        println!("The tax type is not sales");
    }

    if let Tax::INCOME(21) = p.tax_type {
        // Do something if p.tax_type matches the pattern
    } else {
        // Do another thing if it doesn't
    }
    
}

// Each enum variant can have different type of data associated with it.
enum Tax {
    SALES(u32),
    INCOME(u32),
    EXEMPT(String),
    REVENUE {x: u32, y: i32}
}

impl Tax {

    // Just like structs enums can have methods and associated functions

    fn check(&self) {
        // `self` refers to the enum variant on which this is called
    }

}

struct Sale {
    tax_type: Tax
}
