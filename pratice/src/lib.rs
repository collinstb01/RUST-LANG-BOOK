#[derive(Debug)]
// pub struct User {
//     pub name: String,
//     pub email: String,
// }

// impl User {
//     pub fn chnage_user_name(&mut self, name: &str) {
//         self.name.push_str(name);
//     }

//     pub fn get_user_name(&self) -> String {
//         self.name.clone()
//     }
// }

pub struct Numbers {
    pub num1: u32,
    pub num2: u32,
}

impl Numbers {
    pub fn add(&self) -> u32 {
        self.num1 + self.num2
    }
    pub fn sub(&self) -> u32 {
        self.num1 - self.num2
    }
    pub fn mul(&self) -> u32 {
        self.num1 * self.num2
    }
}

pub struct Dog {
    pub sound: String,
}
pub struct Cat {
    pub sound: String,
}

pub trait MakeSound {
    fn make_sound(&self) -> String;
}

impl MakeSound for Dog {
    fn make_sound(&self) -> String {
        println!("This is the sound made my dog {}", self.sound);
        self.sound.to_string()
    }
}

impl MakeSound for Cat {
    fn make_sound(&self) -> String {
        println!("This is the sound made my dog {}", self.sound);
        self.sound.to_string()
    }
}
