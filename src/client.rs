extern crate ws;
#[macro_use] extern crate log;
extern crate env_logger;

use std::io;
use std::io::Write;
use std::thread;

struct Client {
  pub out: ws::Sender,
}

impl Client {
  fn repl(out: ws::Sender) {
    loop {
      print!("> ");
      let _ = std::io::stdout().flush();
      let mut line = String::new();
      let _ = io::stdin().read_line(&mut line);
      match out.send(line) {
        Ok(_) => {},
        Err(err) => println!("Error sending message to server: {}", err),
      }
    }
  }
}

impl ws::Handler for Client {
  fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
    println!("< {}", msg);
    Ok(())
  }
}

fn main() {
  env_logger::init().unwrap();

  // let mut handle = None;
  let guard = thread::spawn(|| {
    ws::connect("ws://127.0.0.1:3012", |out| {
      println!("inside");
      let out_clone = out.clone();
      // handle = Some(thread::spawn(move || Client::repl(out_clone)));
      thread::spawn(move || Client::repl(out_clone));
      Client {
        out: out,
      }
    }).unwrap();
  });
  println!("outside");
  let _ = std::io::stdout().flush();
  thread::sleep(std::time::Duration::from_millis(500));
  // if let Err(err) = handle.unwrap().join() {
  //   println!("child thread panicked: {:?}", err);
  // }
  guard.join();
}