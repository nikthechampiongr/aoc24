use std::io::Write;

fn main() {
    print!("Please enter day: ");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin. How?");
    let num: u8 = buf.trim().parse().expect("Input is not a number");

    let path = format!("days/day_{num}");
    if !std::fs::exists(&path).expect("Failed to determine whether day exists.") {
        eprintln!("Day not found.");
        std::process::exit(1);
    }

    std::env::set_current_dir(path).unwrap();

    if std::process::Command::new("cargo")
        .arg("run")
        .stdout(std::io::stdout())
        .spawn()
        .expect("Failed to run solution")
        .wait()
        .unwrap()
        .code()
        .unwrap()
        != 0
    {
        eprintln!("Running solution returned failure code.");
    }
}
