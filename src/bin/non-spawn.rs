use beatoraja_start::create_beatoraja_command;

fn main() {
    let mut command = create_beatoraja_command();

    println!("Starting beatoraja...");
    let mut child = command.spawn().expect("Failed to run game");

    let _ = child.wait().expect("Failed to wait for game process");
    println!("beatoraja process completed");

    println!("Type Enter to exit...");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
}
