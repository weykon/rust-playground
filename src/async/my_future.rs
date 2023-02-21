// 来自定义一个Future

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("it's time for Future 1.");
            Poll::Ready(String::from("Future 1 has completed."))
        } else {
            println!("it's not yet time for Future 1, Go to sleep");
            let waker = ctx.waker().clone();
            let expiration_time = self.expiration_time;
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    std::thread::sleep(expiration_time - current_time);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}
