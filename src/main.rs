fn main() {
    exec()
}

#[cfg(target_os = "windows")]
fn exec() {
    use std::process::Command;

    let mut command = Command::new(r".\jre\bin\javaw.exe");
    command
        .arg("-Xms1g")
        .arg("-Xmx8g")
        .arg("-XX:+UseShenandoahGC")
        .arg("-XX:+ExplicitGCInvokesConcurrent")
        .arg("-XX:+TieredCompilation")
        .arg("-XX:+UseNUMA")
        .arg("-XX:+AlwaysPreTouch")
        .arg("-XX:-UsePerfData")
        .arg("-XX:+UseThreadPriorities")
        .arg("-XX:+ShowCodeDetailsInExceptionMessages")
        .arg("-cp")
        .arg(r"beatoraja.jar;ir/*")
        .arg("bms.player.beatoraja.MainLoader");

    println!("Starting beatoraja...");
    let _ = command.spawn().expect("Failed to run game");
    println!("Started beatoraja...");
}
