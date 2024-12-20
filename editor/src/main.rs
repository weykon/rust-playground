use std::{
    io::BufRead, thread, time::{Duration, Instant}
};

use termion::is_tty;
use tokio::{io::AsyncReadExt, time::interval};

#[tokio::main]
async fn main() {
    println!("3 seconds later load the editor ...");

    use tokio::time::sleep;
    sleep(Duration::from_secs(2)).await;
    println!("loading ... ");

    sleep(Duration::from_secs(1)).await;
    println!("sorry, the editor is not ready yet ... ");

    let start_timestamp = Instant::now();

    let mut interval = interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        let elapsed = start_timestamp.elapsed();
        println!("{} seconds have passed", elapsed.as_secs());
        if elapsed.as_secs() >= 2 {
            println!("ok! continue ... ");
            break;
        }
    }
    println!("now plz type in \"ok\"...");

    let new_thread_handle = thread::spawn(|| {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut input_str = String::new();

        println!("plz type in \"ok\"");

        if let Ok(_) = stdin.read_line(&mut input_str) {
            if input_str.trim() == "ok" {
                println!("ok! you can now use the editor ...");
            } else {
                println!("you typed in: {}", input_str);
            }
        }
    });

    new_thread_handle.join().unwrap();


    use termion::get_tty;
    let tty = get_tty().unwrap();



}
