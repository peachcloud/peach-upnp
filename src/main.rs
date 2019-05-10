use std::process::Command;

fn main() {
    let cmd = Command::new("upnpc")
        .args(&["-e", "PeachCloud user X", "-r", "8008", "TCP"])
        .output()
        .expect("failed to execute upnpc");

    let output = String::from_utf8(cmd.stdout).unwrap();

    println!("{}", output);
}
