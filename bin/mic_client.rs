use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;


use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:8000/mic";

fn main() {
	println!("Connecting to {}", CONNECTION);

	let client = ClientBuilder::new(CONNECTION)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_insecure()
		.unwrap();

	println!("Successfully connected");

	let (mut receiver, mut sender) = client.split().unwrap();

    let mut counter = 0;

	let receive_loop = thread::spawn(move || {
		// Receive loop
		for message in receiver.incoming_messages() {
			if let Ok(msg) = message {
                match msg {
                    OwnedMessage::Binary(data) => {
                        counter += 1;
                        println!("[{}] Chunk-Len: {}", counter, data.len());
                    }, 
                    _ => println!("Error!")
                }
            }
		}
	});

	// We're exiting

	println!("Waiting for child threads to exit");

	let _ = receive_loop.join();

	println!("Exited");
}