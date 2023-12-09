use std::collections::HashMap;
use rand::prelude::*;

pub struct birthday_attack {

}

impl birthday_attack {
    fn do_birthday_attack(exploit_fn : fn(input: i32) -> i16) -> (i32, i32) {

        let mut rng = rand::thread_rng();
        let mut dict: HashMap<i16, i32> = HashMap::new();
        let mut cnt = 0;

        while true {
            cnt += 1;
            let test_i32: i32 = rng.gen();

            let output_test_i32 = exploit_fn(test_i32);
            if dict.contains_key(&output_test_i32) {
                println!("Found two inputs with the same output : {}, {} - output: {} \n count: {}", &test_i32, dict.get(&output_test_i32).unwrap(), output_test_i32, &cnt);
                return (test_i32, dict.get(&output_test_i32).unwrap().to_owned());
            }
            dict.insert(exploit_fn(test_i32), test_i32);
        }

        (0, 0)
    }
}
