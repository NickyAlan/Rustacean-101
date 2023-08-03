use std::collections::HashMap;

fn main() {
    // random order
    let mut people: HashMap<&str, i32> = HashMap::new();
    people.insert("susan", 21);
    people.insert("kim", 24);
    println!("{:?}", people);    
    println!("{:?}", people.get(&"kim"));

    *people.get_mut("kim").unwrap() += 10;

    for (person, age) in people.iter() {
        if age >= &18 {
            println!("person: {}, age: {}", person, age);
        }
    }
}