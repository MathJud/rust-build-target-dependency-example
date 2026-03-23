#[cfg(target_os = "linux")]
use mylib;

fn main() {
    println!("Running mybin");

    #[cfg(target_os = "linux")]
    println!("{}", mylib::hello());

    #[cfg(not(target_os = "linux"))]
    println!("Not on Linux, mylib is not available.");
}
