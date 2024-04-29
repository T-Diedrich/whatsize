fn main() {
    let (x, y) = crossterm::terminal::size().expect("Could not get size");
    println!("Size is {}x{}!", x, y);
}
