use std::collections::HashMap;

fn main() {

    // 1. Vectors

    // The type annotation is needed as Rust cannot infer it since no elements are been added.
    let v : Vec<i32> = Vec::new();

    // The type is inferred. Has to be mutable if elements will be added.
    let mut v2 = vec![23, 21, 12];

    // Elements of the vector can be accessed by indexing or calling `get*()` function.
    let b = v2.get(2);

    // Implicitly takes a mutable reference to the vector.
    v2.push(90); 

    for i in 0..v2.len() {
        println!("Length is : {}", v2.len());
        println!("{}", &v2[i]);
    }

    // 2. Strings

    // Creates an empty String with initial capacity.
    let name = String::with_capacity(10);

    let description = String::from("This is a blue car");

    // The string types (String and &str) can take any UTF-8 encoded text.

    // The push_str method can be used to grow a String. It takes a &str.
    name.push_str("Adeshina");

    // `push()` appends a single character.
    name.push('o');

    // Won't compile! - two `String` cannot be joined using `+`, only String + &str works.
    // This is because the `+` operator actually calls function `String#add(self, &str)`
    let b = name + description;

    // Compiles! - `name` is now invalid since it's data has been moved into the add(self, &str) function.
    // The compiler coerces &description into &description[..] - since the type needed is &str not &String.
    // Essentially, name.add(&desc[..]) - returns a String to 'new_name'
    let new_name : String = name + &description;

    // The format macro can also concatenate Strings and doesn't take ownership of it's parameters.
    let another_string = format!("{} {}", new_name, description);

    // Fail! -- Rust does not support String indexing.
    let v = another_string[2];

    // Rust allows slicing -- this returns the first two bytes. 
    let p = &another_string[0..2];

    for c in new_name.chars() {
        // iterates over the characters (i.e Unicode scalar values) in a String.
    }

    for b in new_name.bytes() {
        // iterates over the bytes that make up a String.
        // NOTE: Unicode scalar values may be made up of more than one byte.
    }
  
    // 3. HashMaps

    // An empty hash map
    let mut kv_pairs = HashMap::new();

    // For owned types, the HashMap takes ownership of values once they are inserted.
    // For references, the values must be valid for at least as long as the HashMap is valid.
    kv_pairs.insert(String::from("team2"), 53);

    // 'get()' returns an Option that can contain a reference to a value wrapped with Some, or None. 
    kv_pairs.get(&String::from("team2"));

    // Iterating over hash maps.
    for (k, v) in &kv_pairs {
        println!("{}: {}", k, v);
    }

    // Check if a key is already used in the map and insert a value if not. Returns a reference to the value.
    kv_pairs.entry(String::from("team3")).or_insert(55);

}
