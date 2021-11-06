#![allow(dead_code, unused_variables)]

use std::{future::Future, net::TcpStream, sync::Arc};

// #[tokio::main]
// async fn main() {}
fn main() {
    // let runtime = tokio::runtime::Runtime::worker_threads(1);
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
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

        let mut f1 = tokio::fs::File::open("foo");
        let mut f2 = tokio::fs::File::create("bar");
        let copy = tokio::io::copy(&mut f1, &mut f2);

        loop {
            select! {
                stream <- (&mut network).await => {
                    // do something on stream
                }
                line <- (&mut terminal).await => {
                    // do something with line
                    break;
                }
                foo <- (&mut foo).await => {

                }
                _ <- copy.await => {

                }
            };
        }

        // _some_ bytes have been copied from foo to bar, but not all
        // copy.await;

        // let x = foo2();

        let files: Vec<_> = (0..3).map(|i| tokio::fs::read_to_string(format!("file{}", i))).collect();

        // compare
        let file1 = files[0].await;
        let file2 = files[1].await;
        let file3 = files[2].await;
        // files[0].then(files[1]).then(files[2])
        // to this
        let (file1, file2, file3) = join!(files[0], files[1], files[2]);
        // let file_bytes = try_join_all(files);
        let file_bytes = join_all(files);
        // file_bytes[0] == files[0]

        let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        while let Ok(stream) = accept.await {
            tokio::spawn(handle_connection(stream));
        }
        // let mut connections = futures::future::FuturesUnordered::new();
        // loop {
        //     select! {
        //         stream <- (&mut accept).await => {
        //             connections.push(handle_connection(stream));
        //         }
        //         _ <- (&mut connections).await => {}
        //     }
        // }
    });
}

async fn handle_connection(_: TcpStream) { 
    // todo!()
    let x = Arc::new(Mutex::new(vec![]));
    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        deserialize();
        x1.lock();
        let x: Result<_, _> = definitely_errors();
        // 
        0
    });
    join_handle.await;
    assert_eq!(join_handle.await, 0);
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock();
        // 
    })
}

async fn matrix_multiply() {}

async fn read_to_string(s: &str) {}
fn expensive_function(_: ()) {}

async fn foo1() -> usize {
    println!("foo");
    0
}

fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
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
        race! {
            done <- read_to_string("file2").await => {
                // continue; fall-through to println below
            }
            cancel <- cancel.await => {
                return 0;
            }
        }
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
