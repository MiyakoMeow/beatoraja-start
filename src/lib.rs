use std::process::Command;

#[cfg(target_os = "windows")]
fn add_common_args(mut command: Command) -> Command {
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
        .arg("bms.player.beatoraja.MainLoader")
        .args(std::env::args().skip(1));
    command
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command() -> Command {
    let command = Command::new(r".\jre\bin\java.exe");
    add_common_args(command)
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command_with_javaw() -> Command {
    let command = Command::new(r".\jre\bin\javaw.exe");
    add_common_args(command)
}
