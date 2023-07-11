use std::{os::unix::prelude::MetadataExt, thread, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(x: T) -> Cacher<T> {
        Cacher {
            calculation: x,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, simulated_random_number: u32) {
    // closures: they are similar to functions just that the compiler trys to get the
    // the data type itself
    let mut cache_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        print!("Do {}", cache_result.value(intensity));
        print!("Next Do {} pushup", cache_result.value(intensity));
    } else {
        if simulated_random_number == 3 {
            println!("Take a break");
        } else {
            println!("Today run for {} minutes", cache_result.value(intensity));
        }
    }
}

fn main2() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

fn main3() {
    let x = vec![1, 2, 3];

    let same_values = |value| value == x;

    let y = vec![1, 2, 3];

    assert!(same_values(y));
}

// ITERATORS
// allows you to iterating through vec, hasmash, graph

fn main4() {
    let v1 = vec![1, 2, 3];

    let mut v1_int = v1.iter();

    assert_eq!(v1_int.next(), Some(&1));

    for val in v1_int {
        println!("Got {}", val)
    }
}

struct Shoes {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoes>, shoe_size: String) -> Vec<Shoes> {
    shoes.into_iter().filter(|s| s.style == shoe_size).collect()
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {}
