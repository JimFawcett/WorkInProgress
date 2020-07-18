// rust_comm::test1.rs

#![allow(unused_imports)]
#![allow(dead_code)]

/*-- module comm_traits: Traits Sndr<M>, Rcvr<M>, Process<M> --*/
//mod comm_traits;
//use comm_traits::*;
/*-- Comm Processing --*/
//mod comm_processing;
//use comm_processing::*;
/*-- Comm Message --*/
//mod comm_message;
//use comm_message::*;

use std::io::prelude::*;

/*-- component library rust_blocking_queue --*/
use rust_blocking_queue::*;
use rust_message::*;
use rust_traits::*;
use rust_comm_processing::*;
use rust_comm::*;

type M = Message;
type P = CommProcessing;

fn main() {
    let addr = "127.0.0.1:8080";
    let mut lsnr = Listener::<P>::new();
    let handle = lsnr.start(addr).unwrap();
    // let handle = std::thread::spawn(move || {
    //     let _rslt = lsnr.start(addr);
    // });
    //let _rslt = lsnr.start(addr);
    let conn = Connector::<P,M>::new(addr);
    //conn.start(addr);
    let mut msg = Message::new();
    msg.set_type(MessageType::TEXT);
    msg.set_body_str("message #1");
    conn.post_message(msg);
    let msg = conn.get_message();
    print!("\n  mian received msg: {:?}",msg.get_body_str());
    let mut msg = Message::new();
    msg.set_type(MessageType::QUIT);
    conn.post_message(msg);
    let _ = std::io::stdout().flush();
    let _ = handle.join();
    // let mut reply = String::new();
    // let _ = std::io::stdin().read_to_string(&mut reply);
}