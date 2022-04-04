use std::thread;
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

#[derive(Debug, Copy, Clone)]
pub struct Song;

async fn learn_song() -> Song {
    Song
}

async fn sing_song(song: Song) {
    println!("Sing {:?}", song);
}

async fn dance() {
    println!("Dance!");
}

async fn learn_and_sing() {
    let song = learn_song().await;// like `block_on(learn_song())`
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}


fn main() {
    // let future = hello_world();
    // block_on(future);
    block_on(async_main());
    
}

// thread
fn get_two_sites() {
    //Spawn two threads to do work.
    let thread_one = thread::spawn(|| {println!("Hello, world!");});
    let thread_two = thread::spawn(|| {println!("Hello, Peter!");});

    thread_one.join().expect("thread one panicked!");
    thread_two.join().expect("thread two panicked!");
}

// async syntax, futures

