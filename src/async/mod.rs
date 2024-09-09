use std::{thread::sleep, time::Duration, future::Future};
mod my_future;
#[tokio::main]
async fn main() {
    println!("starting ... ");
    let h1 = tokio::spawn(async {
        let content1 = mock_read_file_1().await;
    });
    let h2 = tokio::spawn(async {
        let content2 = mock_read_file_2().await;
    });
    let h3 = tokio::spawn(async {
        let content3  = mock_read_file_3_Future().await;
    });
    let _ = tokio::join!(h1, h2);
}

async fn mock_read_file_1() -> String {
    sleep(Duration::new(4, 0));
    println!("{:?}", "file 1 loading");
    String::from("Hello file 1. ")
}

async fn mock_read_file_2() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}", "file 2 loading");
    String::from("Hello file 2. ")
}

// 上面的是语法糖
fn mock_read_file_3_Future () ->impl Future<Output = String> {
    async { 
        sleep(Duration::new(1,0));
        println!("{:?}", "reading file 3 in \"Future\"");
        String::from("hello file 3.")
    }
}