fn main() {

    // This struct instance is immutable. If it is marked mutable, all it field values can be changed
    let product = Product {
        has_variants: true,
        barcode: 232,
        sku: String::from("Sku-oo1"),
        name: String::from("")
    };

    // This uses the _struct update syntax_. The remaining fields in the struct are set from another instance
    let mut product_1 = Product {
        barcode: 12172,
        ..product // This inits the remaining fields using values from 'product', another instance
    };

    let prdct_c = Product::make_product();

    product_1.get_sku_rev(); // `product_1` is moved into the method and dropped when it's popped off the stack
}

fn new_user(sku: String, name: String) -> Product {

    // This uses the _field init shorthand_ so function params can be used to init struct fields of same name
    Product {
        name,
        sku,
        barcode: 2312,
        has_variants: false
    }
}

// OWNERSHIP OF STRUCT DATA
// Stuct fields can contain references to data not owned by the struct with the use of lifetimes (chapter 10)
// Data owned by the struct last for the struct's lifetime.

struct Product {
    sku: String,
    barcode: u32,
    name: String,
    has_variants: bool,
}


impl Product {
    // Every struct is allowed to have multiple Implementation blocks   
}

// Implementation block for a struct is where methods and associated functions are defined
impl Product {

    // `self` is the instance of the struct on which the method is called.
    // Methods can take ownership of `self` or borrow it. All rules of ownership and references apply

    fn get_barcode_rev(&self) -> u32 {
        // method borrows self immuatably. Field values cannot be changed
        self.barcode * 100
    }

    fn get_sku_rev(self) {
        // Method takes ownership of `self`. The instance is moved in here.
        // The struct instance is dropped when this method exits.
        // This is rarely done.
    }

    fn get_name_rev(&mut self) {
        // Borrows `self` as mutable. This way the field values can be changed
    }

    // Associated functions are not called on struct instances.
    
    fn make_product() -> Product {
        Product {
            barcode: 2112,
            sku: String::from("sew-ww"),
            has_variants: true,
            name: String::from("Benx")
        }
    }

}

// These are `tuple structs`, fields are not named like in regular structs
// The instances of these tuple structs have their own type unlike regular tuples
// They can't be used in place of each other even if their fields have the same types
// Instances of each can be destructured in variables of their type, or access their elements by indexing 
struct Point(u32, u32);
struct Coordinate(u32, u32);

fn tup_strct() {
    let a: Coordinate = Coordinate(2,1);
    let b: Point = Point(6,2);
}
