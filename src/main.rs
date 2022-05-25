fn main() {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    let mut y = ["Ray", "Martin", "Robin", "Huw", "Al", "Steve", "Neil"];
    y.shuffle(&mut rng);
    println!("Today's order:   {:?}", y);
}
