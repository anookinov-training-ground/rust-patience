#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let read_from_terminal = std::thread::spawn(move || {
        let x = std::io::Stdin::lock(std::io::stdin());
        for line in x.lines() {
            // do something on user input
        }
    });

    let read_from_network = std::thread::spawn(move || {
        let mut x = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
        while let Ok(stream) = x.accept() {
            // do something on stream
            let handle = std::thread::spawn(move || {
                handle_connection(stream);
            });
        }
    });

    let network = read_from_network();
    let terminal = read_from_terminal();
    let mut foo = foo2();

    loop {
        select! {
            stream <- network.await => {
                // do something on stream
            }
            line <- terminal.await => {
                // do something with line
            }
            foo <- foo.await => {

            }
        };
    }

    // let x = foo2();
}

async fn read_to_string(s: &str) {}
fn expensive_function(_: ()) {}

async fn foo1() -> usize {
    println!("foo");
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {

        // let x = read_to_string("file").await;

        let fut = read_to_string("file");
        let x = loop {
            if let Some(result) = fut.try_check_completed() {
                break result;
            } else {
                fut.try_make_progress();
                yield;
            }
        }

        // First time:
        println!("foo1");
        read_to_string("file1").await;
        // let fut = read_to_string("file1"); // Wait here
        // while !fut.is_ready() {
        //     std::thread::yield_now();
        //     fut.try_complete();
        // }
        // let result = fut.take_result();
        // Second time:
        println!("foo1");
        read_to_string("file2").await; // Wait here
        println!("foo1");
        let x = /* waiting on */ read_to_string("file3").await;
        println!("foo1");
        expensive_function(x);
        /* yield again and wait on */ read_to_string("file4").await;
        println!("foo2");
        0
    }
}
