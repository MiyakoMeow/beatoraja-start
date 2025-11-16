use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
fn add_common_args(mut command: Command) -> Command {
    command
        .arg("-Xms1g")
        .arg("-Xmx8g")
        .arg("-XX:+UseZGC")
        .arg("-XX:+DisableExplicitGC")
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
fn can_run(exe: &str) -> bool {
    Command::new(exe)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

#[cfg(target_os = "windows")]
fn resolve_executable<'a>(candidates: &'a [&'a str]) -> &'a str {
    candidates
        .iter()
        .copied()
        .find(|p| can_run(p))
        .unwrap_or_else(|| candidates.last().copied().expect("no candidates provided"))
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command() -> Command {
    let exe = resolve_executable(&[r".\jre\bin\java.exe", r".\jdk\bin\java.exe", r"java.exe"]);
    let command = Command::new(exe);
    add_common_args(command)
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command_with_javaw() -> Command {
    let exe = resolve_executable(&[r".\jre\bin\javaw.exe", r".\jdk\bin\javaw.exe", r"javaw.exe"]);
    let command = Command::new(exe);
    add_common_args(command)
}
