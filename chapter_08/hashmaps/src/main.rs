use std::collections::HashMap;

fn main() {
    {
        // Creating a new hashmap
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yello"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("team: {team_name}, score is: {score}");
    }

    {
        // Iterate over each key-value pairs.
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yello"), 50);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        // Hashmap and ownerships
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point.
    }

    {
        // Updating
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);
        println!("{scores:?}");

        // Adding a key and value only if a key isn't present
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(100);
        println!("{scores:?}");
    }

    {
        // Updating a value based on the old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{map:?}");
    }
}
