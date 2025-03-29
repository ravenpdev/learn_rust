fn main() {
    let vowels = ["a", "e", "i", "o", "u"];

    let mut words = vec![
        "apple".to_string(),
        "banana".to_string(),
        "first".to_string(),
        "".to_string(),
    ];

    for word in &mut words {
        let Some(first_char) = word.get(0..1) else {
            break;
        };

        if vowels.contains(&first_char.to_ascii_lowercase().as_str()) {
            word.push_str("hay");
        } else {
            let c = word.remove(0);
            let w = format!("{c}ay");
            word.push_str(&w);
        }
    }

    println!("{words:?}");
}
