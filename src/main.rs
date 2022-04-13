use zmq;
use std::env;
use std::f64;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use gethostname::gethostname;
use std::os::unix::ffi::OsStrExt;

const ZMQ_PREFIX: &str = "tcp://";

fn seconds(d: &Duration) -> f64 {
    d.as_secs() as f64 + (f64::from(d.subsec_nanos()) / 1e9)
}

fn run(ctx: &mut zmq::Context, url: &str) -> Result<(), zmq::Error> {
    let mut msg = zmq::Message::new();
    let server_url = format!("{}{}", ZMQ_PREFIX, url);
    let socket = ctx.socket(zmq::DEALER).unwrap();
    socket.set_identity(gethostname().as_bytes())?;
    socket.connect(&server_url)?;

    println!("Connected to {}", &server_url);
    socket.send("Hello ROUTER this is DEALER", 0)?;
    loop {
        {
            let mut items = [socket.as_poll_item(zmq::POLLIN)];
            zmq::poll(&mut items, -1)?;
            if !items[0].is_readable() {
                println!("ERROR - poll item unreadable!");
                return Ok(());
            }

            socket.recv(&mut msg, 0)?;
            println!("Received message: {:?}", &msg.as_str());
  
        }
    }
}

//  The DEALER will connect with the ROUTERs at the specified endpoints
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_ne!(args.len(), 1);

    let mut ctx = zmq::Context::new();
    for i in 1..args.len() {
        let server_url = &args[i];
        match run(&mut ctx, &server_url) {
            Ok(_) => {
                println!("OK - shutting down");
            }
            Err(_) => {
                println!("Error");
            }
        }
    }

}
