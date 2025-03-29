fn main() {
    {
        // Creating a new vector
        let _: Vec<i32> = Vec::new();

        // Creating a vector with initial value using vec! macro
        let mut v = vec![1, 2, 3];

        // Updating a vector
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        // Reading elements of Vectors
        // There are two ways to reference a value stored in a vector: via indexing or by using the
        // get method.

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(value) => println!("The third element is {value}"),
            None => println!("There is no third element!"),
        }
        // if let Some(third) = third {
        //     println!("The third element is {third}");
        // } else {
        //     println!("There is no thrid element!");
        // }

        // Iterating over the values in a vector
        for i in &v {
            println!("{i}");
        }

        // We can also iterate over a mutable references to each element in a mutable vector
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{i}");
        }
    }

    {
        // Using an enum to store multiple types

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("Hello")),
        ];
    }
}
