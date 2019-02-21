
#![allow(unused_imports,unused_variables)]
use std::sync::{Arc, Mutex};
use std::thread;
use mt::*;
use mt::statests;
use std::num::Wrapping;


fn main() {
    let mut k = MT::new();
    k.seed(4);
    let mutex = Arc::new(Mutex::new(k));

    for i in 0..5 {
        let mutex_i = mutex.clone();
        thread::spawn(move || {
            let mut twister = mutex_i.lock().unwrap();
            println!("thread {} has value {}", i, twister.get_real()); 
        });
    }
    let mut y = MT::new();
    let mut r = Vec::new();
    y.seed(4);
    for i in 0..100000 {
       
        r.push(y.get_real());
    }
    println!("{:?}", y.take(4).collect::<Vec<u64>>());
    println!("{}", mt::statests::rngchi(&r, 10u32));
}
        

