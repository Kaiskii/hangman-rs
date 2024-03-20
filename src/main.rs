use rand::Rng;

fn main() {
    println!("Hello, {}!", rand::thread_rng().gen_range(1..101));
}
