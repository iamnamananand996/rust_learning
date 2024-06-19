use std::{thread, time::Duration};

fn simulate_expansive_intensity(intensity: u32) -> u32 {
    println!("calculation slow ...");
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
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
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

fn main() {
    generate_workout_function(33, 10)
}

fn generate_workout_function(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 32 {
        println!("Today, do {} pushup", cached_result.value(intensity));
        println!("Next do {} situp", cached_result.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes", cached_result.value(intensity))
        }
    }
}
