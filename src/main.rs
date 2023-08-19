// TODO test on linux

use std::{thread, time, env, process::Command};

fn notify(message: &str) {
    match env::consts::OS {
        "macos" => notify_macos(message),
        "linux" => notify_linux(message),
        _ => {
            println!("Unsuported");
            return;
        }
    }
}

fn notify_macos(message: &str) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!("display alert {}", message))
        .spawn()
        .expect("Failed to show notification")
    ;
}

fn notify_linux(message: &str) {
    Command::new("xmessage")
        .arg(message)
        .spawn()
        .expect("Failed to show notification")
    ;
}

fn main() {
    if !["linux", "macos"].contains(&env::consts::OS) {
        println!("Unsuported os.");
        return;
    }

    loop {
        thread::sleep(time::Duration::from_secs(1200));
        notify("Take a break!");
        thread::sleep(time::Duration::from_secs(300));
        notify("Back to work!");
    }
}
