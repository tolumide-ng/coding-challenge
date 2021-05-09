pub mod chapter_2;
use chapter_2::build_timer::TimerFuture;

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, Sender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
};

fn main() {
    println!("Hello, world!");
}
