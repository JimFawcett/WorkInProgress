// processing.rs

#![allow(dead_code)]

/*-- Comm Message --*/

use rust_traits::*;
use rust_message::*;
use rust_blocking_queue::*;

//use crate::comm_message::{Message, MessageType, Message::get_type};
// use crate::comm_message::{Message};
use std::fmt::*;
use std::net::{TcpStream};
//use std::net::*;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};


type M = Message;
//type Que = BlockingQueue<M>;

#[derive(Debug)]
pub struct CommProcessing<M>
where M: Msg + std::fmt::Debug + Clone + Send + Default,
{
    que: BlockingQueue<M>,
}
impl<M> CommProcessing<M>
where M: Msg + std::fmt::Debug + Clone + Send + Default,
{
    pub fn new() -> CommProcessing<M> {
        CommProcessing {
            que: BlockingQueue::new(),
        }
    }
}
impl<M> Sndr<M> for CommProcessing<M>
where M: Msg + std::fmt::Debug + Clone + Send + Default,
{
    fn send_message(msg: M, stream: &mut TcpStream) -> std::io::Result<()>
    {
        let typebyte = msg.get_type();
        let buf = [typebyte];
        stream.write(&buf)?;
        let bdysz = msg.get_body_size();
        /*-- to_be_bytes() converts integral type to big-endian byte array --*/
        stream.write(&bdysz.to_be_bytes())?;
        stream.write(&msg.get_body_bytes())?;
        let _ = stream.flush();
        Ok(())
    }
    fn buf_send_message(msg: M, stream: &mut BufWriter<TcpStream>) -> std::io::Result<()>
    {
        let typebyte = msg.get_type();
        let buf = [typebyte];
        stream.write(&buf)?;
        let bdysz = msg.get_body_size();
        /*-- to_be_bytes() converts integral type to big-endian byte array --*/
        stream.write(&bdysz.to_be_bytes())?;
        stream.write(&msg.get_body_bytes())?;
        let _ = stream.flush();
        Ok(())
    }
}
impl<M> Rcvr<M> for CommProcessing<M>
where M: Msg + std::fmt::Debug + Clone + Send + Default,
{
    fn recv_message(stream: &mut TcpStream, q:BlockingQueue<M>) -> std::io::Result<()> 
    {
        let mut msg = M::default();
        /*-- get MessageType --*/
        let buf = &mut [0u8; 1];
        stream.read_exact(buf)?;
        let msgtype = buf[0];
        msg.set_type(msgtype);
        /*-- get body size --*/
        let buf = &mut [0u8; 4];
        stream.read_exact(buf)?;
        let bdysz = usize::from_be_bytes(*buf);
        /*-- get body bytes --*/
        let mut bdy = vec![0u8;bdysz];
        stream.read_exact(&mut bdy)?;
        msg.set_body_bytes(bdy);
        let mut mod_body = msg.get_body_str();
        mod_body.push_str(" reply");
        msg.clear();
        msg.set_body_str(&mod_body);
        q.en_q(msg);
        Ok(())
    }
    fn buf_recv_message(stream: &mut BufReader<TcpStream>, q: BlockingQueue<M>) -> std::io::Result<()> 
    where M: Msg + std::fmt::Debug + Clone + Send + Default,
    {
        let mut msg = M::default();
        /*-- get MessageType --*/
        let buf = &mut [0u8; 1];
        stream.read_exact(buf)?;
        let msgtype = buf[0];
        msg.set_type(msgtype);
        /*-- get body size --*/
        let buf = &mut [0u8; 4];
        stream.read_exact(buf)?;
        let bdysz = usize::from_be_bytes(*buf);
        /*-- get body bytes --*/
        let mut bdy = vec![0u8;bdysz];
        stream.read_exact(&mut bdy)?;
        msg.set_body_bytes(bdy);
        let mut mod_body = msg.get_body_str();
        mod_body.push_str(" reply");
        msg.clear();
        msg.set_body_str(&mod_body);
        q.en_q(msg);
        Ok(())
    }
}
impl<M> Process<M> for CommProcessing<M> 
where M: Msg + std::fmt::Debug + Clone + Send + Default,
{
    fn process_message(&self, m: M) -> M 
    {
        m.show_msg();
        let rply = M::default();
        // msg
        rply
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction() {
        let msg = Message::new();
        let _cp = CommProcessing::<Message>::default();
        let addr = "127.0.0.1:8080";
        let _lstnr = std::net::TcpListener::bind(addr);
        let mut stream = std::net::TcpStream::connect(addr).unwrap();
        let _ = CommProcessing::send_message(msg, &mut stream);
        assert_eq!(2 + 2, 4);
    }
}
