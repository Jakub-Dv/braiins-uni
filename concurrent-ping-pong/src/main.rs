use flume::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn ping_task(bool_tx: Sender<bool>, display_tx: Sender<&str>) {
    loop {
        bool_tx.send(true).unwrap();
        display_tx.send("ping").unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}

fn pong_task(bool_rx: Receiver<bool>, display_tx: Sender<&str>) {
    loop {
        if Ok(true) == bool_rx.recv() {
            display_tx.send("pong").unwrap();
        };
    }
}

fn display_task(display_rx: Receiver<&str>) {
    loop {
        let received = display_rx.recv().unwrap();
        println!("{}", received);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded::<&str>();
    let (tx2, rx2) = flume::unbounded::<bool>();

    let (tx1_clone, tx2_clone) = (tx.clone(), tx2);
    tokio::spawn(async {
        ping_task(tx2_clone, tx1_clone);
    });

    let (rx2_clone, tx1_clone) = (rx2, tx.clone());
    tokio::spawn(async {
        pong_task(rx2_clone, tx1_clone);
    });

    display_task(rx);
}
