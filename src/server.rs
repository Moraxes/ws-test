extern crate ws;
#[macro_use] extern crate log;
extern crate env_logger;

use std::sync::{Arc, Mutex};
use std::string::ToString;

struct ConnHandler {
  pub out: ws::Sender,
  pub state: Arc<Mutex<i32>>,
}

impl ConnHandler {
  pub fn fork(&self, onto: ws::Sender) -> ConnHandler {
    ConnHandler {
      out: onto,
      state: self.state.clone(),
    }
  }
}

impl ws::Handler for ConnHandler {
  fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
    println!("> {}", msg);
    match msg.as_text().unwrap().trim() {
      "read" => {
        let resp = self.state.lock().unwrap().to_string();
        println!("< {}", msg);
        return self.out.send(resp);
      },
      "incr" => {
        let mut val = self.state.lock().unwrap();
        *val += 1;
      },
      "decr" => {
        let mut val = self.state.lock().unwrap();
        *val -= 1;
      },
      _ => {
        println!("no match")
      },
    }
    Ok(())
  }
}

fn main() {
  env_logger::init().unwrap();

  let mut root = None;
  if let Err(err) = ws::listen("127.0.0.1:3012", |out| {
    if let None = root {
      root = Some(ConnHandler {
        out: out.clone(),
        state: Arc::new(Mutex::new(0)),
      });
    }
    root.as_ref().unwrap().fork(out)
  }) {
    println!("could not start listening: {}", err);
  }
}
