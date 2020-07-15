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

/*-- component library rust_blocking_queue --*/
use rust_blocking_queue::*;
use rust_message::*;
use rust_traits::*;
use rust_comm_processing::*;
use rust_comm::*;

type M = Message;
type P = CommProcessing<M>;

fn main() {
    let conn = Connector::<P,M>::new();
    let lsnr = Listener::<P,M>::new();
    lsnr.start("127.0.0.1:8080");
    conn.start("127.0.0.1:8080");
    let msg = Message::new();
    msg.set_type(MessageType::TEXT);
    msg.set_body_str("message #1");
    conn.post_message(msg);


}