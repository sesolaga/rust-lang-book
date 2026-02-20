use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    hash: HashMap<U, U>,
    calc_run_count: u64,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            hash: HashMap::new(),
            calc_run_count: 0,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.hash.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.hash.insert(arg, v);
                self.calc_run_count += 1;
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));

        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_calculates_value_only_once_for_same_arg() {
        let mut cacher = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        assert_eq!(cacher.value(2), 2);
        assert_eq!(cacher.value(2), 2);
        assert_eq!(cacher.value(2), 2);
        assert_eq!(cacher.calc_run_count, 1);
    }
    #[test]

    fn cache_returns_different_values_for_different_args() {
        let mut cacher = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        assert_eq!(cacher.value(2), 2);
        assert_eq!(cacher.value(3), 3);
        assert_eq!(cacher.value(4), 4);
    }
}
