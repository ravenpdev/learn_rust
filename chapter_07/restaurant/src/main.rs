use rand::Rng;

fn main() {
    restaurant::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");
}
