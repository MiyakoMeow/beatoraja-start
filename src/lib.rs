use std::process::{Command, Stdio};

fn add_common_args(mut command: Command) -> Command {
    command
        // 初始 heap 1g，避免启动后立即触发 GC 扩展
        .arg("-Xms1g")
        // 上限 8g，覆盖绝大多数 BMS 谱面包+皮肤资源需求
        .arg("-Xmx8g")
        // ZGenerational 仍为实验性，需 unlock
        .arg("-XX:+UnlockExperimentalVMOptions")
        // 低延迟并发 GC，适合游戏场景（避免 GC pause 卡音）
        .arg("-XX:+UseZGC")
        // 分代 ZGC：年轻对象高频回收，降低总 GC 开销
        .arg("-XX:+ZGenerational")
        // OOM 后无恢复路径，直接退出防不确定状态
        .arg("-XX:+ExitOnOutOfMemoryError")
        // 禁 System.gc()，防第三方库误触 ZGC 全量 cycle
        .arg("-XX:+DisableExplicitGC")
        // Linux THP：降 ZGC 扫描时 TLB miss，其余平台 no-op
        .arg("-XX:+UseTransparentHugePages")
        .args(["-cp", CLASSPATH])
        .arg("bms.player.beatoraja.MainLoader")
        .args(std::env::args().skip(1));
    command
}

#[cfg(target_os = "windows")]
const CLASSPATH: &str = r"beatoraja.jar;ir/*";

#[cfg(not(target_os = "windows"))]
const CLASSPATH: &str = "beatoraja.jar:ir/*";

fn can_run(exe: &str) -> bool {
    Command::new(exe)
        .arg("-version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn resolve_executable<'a>(candidates: &'a [&'a str]) -> &'a str {
    candidates
        .iter()
        .copied()
        .find(|p| can_run(p))
        .unwrap_or(candidates.last().expect("no candidates provided"))
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command() -> Command {
    let exe = resolve_executable(&[r".\jre\bin\java.exe", r".\jdk\bin\java.exe", r"java.exe"]);
    let command = Command::new(exe);
    add_common_args(command)
}

#[cfg(not(target_os = "windows"))]
pub fn create_beatoraja_command() -> Command {
    let exe = resolve_executable(&["./jre/bin/java", "./jdk/bin/java", "java"]);
    let command = Command::new(exe);
    add_common_args(command)
}

#[cfg(target_os = "windows")]
pub fn create_beatoraja_command_with_javaw() -> Command {
    let exe = resolve_executable(&[r".\jre\bin\javaw.exe", r".\jdk\bin\javaw.exe", r"javaw.exe"]);
    let command = Command::new(exe);
    add_common_args(command)
}

#[cfg(not(target_os = "windows"))]
pub fn create_beatoraja_command_with_javaw() -> Command {
    let exe = resolve_executable(&["./jre/bin/javaw", "./jdk/bin/javaw", "javaw"]);
    let command = Command::new(exe);
    add_common_args(command)
}
