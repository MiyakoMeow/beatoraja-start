use beatoraja_start::create_beatoraja_command_with_javaw;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut command = create_beatoraja_command_with_javaw();

        println!("Starting beatoraja with spawn...");
        #[allow(clippy::zombie_processes)]
        let _ = command.spawn().expect("Failed to run game");
        println!("Started beatoraja with spawn...");
    }

    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("This program only supports Windows");
        std::process::exit(1);
    }
}
