pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Basic Formatting
    println!("Number: {} {} {}", 1, "lol", 5);

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} likes to play {activity}", name = "Dominic", activity = "Lost Ark");


    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello", 0x123));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}