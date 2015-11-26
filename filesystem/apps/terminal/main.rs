use redox::fs::File;
use redox::process::Command;
use redox::syscall::sys_close;

pub fn main() {
    unsafe {
        sys_close(2);
        sys_close(1);
        sys_close(0);
    }

    let stdin = File::open("terminal:Terminal").unwrap();
    let stdout = stdin.dup().unwrap();
    stdout.dup().unwrap();

    let path = "file:/apps/shell/main.bin";
    if let Some(mut child) = Command::new(path).spawn() {
        if let Some(status) = child.wait() {
            if let Some(code) = status.code() {
                debugln!("{}: Child exited with exit code: {}", path, code);
            } else {
                debugln!("{}: No child exit code", path);
            }
        } else {
            debugln!("{}: Failed to wait", path);
        }
    } else {
        debugln!("{}: Failed to execute", path);
    }
}
