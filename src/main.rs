use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut users = ["Ray", "Martin", "Robin", "Huw", "Al", "Steve", "Neil", "Will", "John"];
    users.shuffle(&mut rng);
    println!("Today's order:   {:?}", users);
}
