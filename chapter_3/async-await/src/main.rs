use futures::executor;
// use futures::{executor, future::join_all};
// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};

// struct User {}
// struct UserId(u32);
// struct Db{}
// impl Db {
//     async fn find_by_user_id(&self,user_id:UserId)->Option<User>{

//     }
// }

// struct CountDown(u32);
// impl Future for CountDown {
//     type Output = String;
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         if self.0 == 0 {
//             Poll::Ready("Zero!!!".to_string())
//         } else {
//             println!("{}", self.0);
//             self.0 -= 1;
//             cx.waker().wake_by_ref();
//             Poll::Pending
//         }
//     }
// }

// async fn hoge() {
//     println!("hoge")
// }

// fn main() {
//     println!("Hello, world!");

//     let countdown_future1 = CountDown(10);
//     let countdown_future2 = CountDown(20);
//     let cd_set = join_all(vec![countdown_future1, countdown_future2]);
//     let res = executor::block_on(cd_set);
//     for (i, s) in res.iter().enumerate() {
//         println!("{} : {}", i, s);
//     }
// }

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function()->i32{
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(4, 2).await;
    let ans3 = async_add(1, 1).await;
    let result = ans1+ans2+ans3;
    print!("{}",result);
  result
}

fn main() {
    executor::block_on(something_great_async_function());
}
