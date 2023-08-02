#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            return Ok(Self { 
                age, 
                name: name.to_owned()
            });
        } 
        else {
            return Err("Adult must be at least 21 years old");
        }
    }    
}

fn main() {
    let adult= Adult::new(22, "Kim");
    let child = Adult::new(15, "Jay");

    match adult {
        Ok(person) => println!("{}: {}", person.name, person.age),
        Err(e) => println!("{}", e)
    }

    match child {
        Ok(person) => println!("{}: {}", person.name, person.age),
        Err(e) => println!("{}", e)
    } 
}