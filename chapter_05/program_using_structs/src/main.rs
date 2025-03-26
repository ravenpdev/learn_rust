#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    {
        // Calculate area of rectangle
        let width = 30;
        let height = 50;

        println!(
            "The area of a rectangle is {} square pixels",
            area(width, height)
        );
    }

    {
        // Calcuate area of rectangle using tuple
        let rect = (30, 50);
        println!("The area of a rectangle is {} square pixels", area2(rect));
    }

    {
        // Refactoring with Structs
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        println!("The area of a rectangle is {} square pixels", area3(&rect));
    }

    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect is {rect:?}");
        println!("rect is {rect:#?}");
    }

    {
        // Another way to print out a value using the Debug format is to use the dbg! macro
        //
        // Note: calling dbg! macro print to the standard error console stream (stderr), as opposed
        // to println!, which prints to the standard output console streram (stdout).
        let scale = 2;
        let rect = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
