use std::collections::HashMap;

fn main() {
    let given_numbers = vec![1, 2, 2, 5, 5, 7, 7, 7, 7];

    let data = prepare_data(&given_numbers);

    let mode = mode(&data);

    println!("list of given_number: {given_numbers:?}");
    println!("key value pairt : {data:?}");
    println!("mode: {mode}");
}

fn prepare_data(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut key_value = HashMap::new();

    for &val in numbers {
        let count = key_value.entry(val).or_insert(0);
        *count += 1;
    }

    key_value
}

fn mode(data: &HashMap<i32, i32>) -> i32 {
    let mut mode = 0;

    for (&key, &value) in data {
        if mode < value {
            mode = key;
        }
    }

    mode
}
