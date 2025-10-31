use beatoraja_start::create_beatoraja_command;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut command = create_beatoraja_command();

        println!("Starting beatoraja...");
        let mut child = command.spawn().expect("Failed to run game");

        // Wait for the process to complete
        let _ = child.wait().expect("Failed to wait for game process");
        println!("beatoraja process completed");

        // Add any additional logic here
        println!("Type Enter to exit...");
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read input");
    }

    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("This program only supports Windows");
        std::process::exit(1);
    }
}
