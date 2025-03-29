fn main() {
    let mut numbers = vec![1, 5, 3, 2, 4, 6];
    let result = median(&mut numbers);
    println!("{result}");

    let mut numbers = vec![1, 5, 3, 2, 4];
    let result = median(&mut numbers);
    println!("{result}");
}

fn median(numbers: &mut Vec<i32>) -> f64 {
    numbers.sort();

    let length = numbers.len();

    if length % 2 == 0 {
        let middle = length / 2;
        return (numbers[middle - 1] + numbers[middle]) as f64 / 2.0;
    }

    numbers[length / 2] as f64
}
