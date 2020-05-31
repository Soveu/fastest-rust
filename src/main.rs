use futures::stream::{StreamExt, FuturesUnordered};
use tokio::process::{Command};
use std::fs::File;

async fn cat(file_name: &str) -> Result<std::process::Output, std::io::Error> {
    let file = File::open(file_name)?;
    let handle = Command::new("cat")
        .arg("-")
        .kill_on_drop(true)
        .stdin(file)
        .output();

    return handle.await;
}

const N: usize = 4;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut children = FuturesUnordered::new();
    let mut outputs = Vec::with_capacity(N);

    for i in 0..N {
        println!("Spawning child number {}", i);
        children.push(cat("/dev/shm/input.txt"));
    }

    while let Some(ret) = children.next().await {
        match ret {
            Err(e) => println!("{:?}", e),
            Ok(child) => outputs.push(child),
        }
    }

    for i in 1..N {
        assert!(outputs[i].stdout == outputs[0].stdout);
    }
    println!("Read {} x4 bytes", outputs[0].stdout.len());

    Ok(())
}

