// closures are anonymous functions that can be stored as variables

//There are more differences between functions and closures.
// Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
// Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
// Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.

// Closures, on the other hand, aren’t used in an exposed interface like this:
// They’re stored in variables and used without naming them and exposing them to users of our library.

// Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
// Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

use std::hash::Hash;
use std::{collections::HashMap, thread, time::Duration};

fn main() {
    let simulated_intensity = 30;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

// In order to define structs, enums, fn parameters that use closures,
// we need to use generics and trait bounds.
// 3 closure traits: Fn, FnMut, FnOnce
struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    calculation: T,
    values: HashMap<K, V>,
}

// caching values is a useful behavior, however we can't use it under different context

// 2 problems with this:

// 1. Calling our value method is going to return the same value no matter what the arg input parameter is.

// Solution: Cache one value for each argument being passed in, fix the implementation by
// storing a hashmap instead of a single value. the keys of the hashmap are the keys to the
// argument passed into the value and the hashmap values will be the result of calling the
// the closure with the argument.
// Inside the value method body, we'll need to look up the arg inside the hashmap and if the
// value for that arg exists then return the value and if it doesn't exist then run the
// expensive calculation and store the result inside the hash map

// 2. We're using hard coded types, our closure needs to accept an integer and return an integer
// and our value needs to return an integer

// Solution: We can use generics instead of hardcoded values

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        if let Some(v) = self.values.get(&arg) {
            v.clone()
        } else {
            let v = (self.calculation)(arg.clone());
            self.values.insert(arg.clone(), v.clone());
            v
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
        println!("Today, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today run for {} minutes", cached_result.value(intensity))
        }
    }
}
