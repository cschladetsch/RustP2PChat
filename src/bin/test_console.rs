fn main() {
    println!("Console test - can you see this?");
    println!("Args: {:?}", std::env::args().collect::<Vec<_>>());
    
    // Force console interaction
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("You typed: {}", input.trim());
    println!("Test complete!");
}