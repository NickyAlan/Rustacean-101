// https://youtu.be/lzKeecy4OmQ?list=PLOISWgsXLgA60iCuk3UwPVmrbW6MHWN0f&t=14994

fn first_name(name: &str) {
    println!("Hey! {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn what_about(number: i32) {
    let n = 5;
    if number > n {
        println!("{} is greater than {}", number, n);
    }
    else if number < n {
        println!("{} is less than {}", number, n);
    }
    else {
        println!("{} is equal to {}", number, n);
    }
}

// enum
enum Direction {
    Left,
    Right,
    Up
}

enum Color {
    Red,
    Green,
    Blue
}

// struct
struct Box {
    depth: i32,
    width: i32,
    height: i32
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

enum Access {
    Admin,
    Manager,
    User,
    Guest
}

enum BoxColor {
    Brown,
    Red
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Brown => println!("BoxColor: Brown"),
            BoxColor::Red => println!("BoxColor: Red")
        }
    }
}

struct Dimentions {
    width: f32,
    heigth: f32,
    depth: f32
}

impl Dimentions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("heigth: {:?}", self.heigth);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: BoxColor,
    weight: f32,
    dimentions: Dimentions
}

fn main() {
    let mut sum_number = 0;
    for n in 5..=10 {
        if n %2 == 0 {
            println!("{} is even", n);
        }
        else {
            println!("{} is odd", n);
        }
        sum_number += n;
    }
    println!("{}", sum_number);
    first_name("Jayson");
    let (x, y) = (10, 6);
    println!("{}", add(x, y));
    what_about(4);

    // match
    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("something else"),
    }

    //loop
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i-=1;
        if i == 0 {
            break;
        }
    }

    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
    }

    print_color(Color::Red);

    let my_box = Box {
        depth: 3,
        width: 5,
        height: 5
    };
    println!("my_box area: {:?}", my_box.width * my_box.height);

    // tuples
    let coord = (2, 3);
    println!("x: {:?}, y: {:?}", coord.0, coord.1);

    // expression: accept return value
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can access: {:?}", can_access_file);
    
    // ownership: data use in fn -> delete when complete
    // borrow : use & for borrow data
    let my_color = Color::Blue;
    fn display_color(my_color: &Color) {
        match my_color {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
    display_color(&my_color);
    display_color(&my_color);

    // impl: implement something
    impl Box {
        fn cal_volumn(&self) -> i32 {
            self.width * self.height * self.depth
        }
    }
    let this_box = Box{depth: 3, width: 5, height: 4};
    let vol = this_box.cal_volumn();
    println!("volumn: {:?}", vol);

    impl ShippingBox {
        fn new(weight: f32, color: BoxColor, dimentions: Dimentions) -> Self {
            Self {weight, color, dimentions}
        }
        fn print(&self) {
            self.color.print();
            self.dimentions.print();
            println!("weight: {:?}", self.weight);
        }
    }

    let small_dimentions = Dimentions {
        width: 1.4,
        heigth: 3.1,
        depth: 3.2,
    };

    let small_box = ShippingBox::new(5.0, BoxColor::Red, small_dimentions);
    small_box.print();

    // vector
    let mut numbers = vec![1, 2, 3];
    numbers.push(1);
    numbers.push(3);
    numbers.pop();
    println!("{:?}", numbers);
    println!("{:?}", numbers[numbers.len() - 1]);

    // &bowrrow in for loop
    for num in &numbers {
        println!("{:?}", num);
    }

    println!("--");
    for idx in 0..numbers.len() {
        println!("idx:{:?} val:{:?}", idx, numbers[idx]);
    }

    // string
    let owned_string = "owned string".to_owned(); // or String::from("owned string")
    struct Person {
        name: String, // in struct must use String owner / not &str borrow -> use in fn
        age: i8,
        fav_color: String
    }

    let people = vec![
        Person {
            name: "Jame".to_owned(),
            age: 10,
            fav_color: "Blue".to_owned()
        },
        Person {
            name: "Ball".to_owned(),
            age: 14,
            fav_color: "Red".to_owned()
        },Person {
            name: "Owen".to_owned(),
            age: 12,
            fav_color: "Black".to_owned()
        },
    ];

    fn just_print(data: &str) {
        println!("{:?}", data);
    } 

    for person in people {
        if person.age > 10 {
            just_print(&person.name);
            just_print(&person.fav_color);
        };
    }
    

}
