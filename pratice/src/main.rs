use pratice::{Cat, Dog, MakeSound, Numbers};

// fn main() {
//     let mut users: Vec<User> = Vec::new();
//     // create a user
//     let user = User {
//         name: String::from("Collins"),
//         email: String::from("collins@gmail.com"),
//     };

//     users.push(user);

//     print!("These are all the {:?}", &users);

//     let first_user = &mut users[0];

//     let name = "dave";

//     first_user.chnage_user_name(name);

//     print!("These are all the {:?}", &users);
// }

enum Signs {
    Add,
    Substract,
    Mul,
}

fn main2() {
    let user_input = Some("sub");
    let numbers = Numbers { num1: 2, num2: 2 };

    match user_input {
        Some(x) if x == String::from("add") => {
            let answer = Numbers::add(&numbers);
            println!("{}", answer)
        }
        Some(x) if x == String::from("sub") => {
            let answer = Numbers::sub(&numbers);
            println!("{}", answer)
        }
        _ => panic!(),
    };
}

fn main() {
    let dog = Dog {
        sound: String::from("Ahuuu"),
    };

    let cat = Cat {
        sound: String::from("Meow"),
    };

    dog.make_sound();

    println!("{}", dog.sound)
}
