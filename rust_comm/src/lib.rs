// rust_comm::lib.rs

#![allow(unused_imports)]
#![allow(dead_code)]

/*-- module comm_traits: Traits Sndr<M>, Rcvr<M>, Process<M> --*/
use rust_traits::*;
use rust_message::*;
use rust_comm_processing::*;
use rust_blocking_queue::*;

// /*-- Comm Processing --*/
// mod comm_processing;
// use comm_processing::*;
// /*-- Comm Message --*/
// mod comm_message;
// use comm_message::*;

// /*-- component library rust_blocking_queue --*/

use std::fmt::*;
use std::sync::{Arc};
use std::net::{TcpStream, TcpListener};
use std::io::{Result, BufReader, BufWriter};
use std::thread;

pub type M = Message;
pub type P = CommProcessing<M>;

#[derive(Debug)]
pub struct Connector<P,M> where 
    M: Msg + Debug + Clone + Send + Default,
    // Q: Debug + Send + Sync + Default
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> 
{
    snd_queue: Arc<BlockingQueue<M>>,
    rcv_queue: Arc<BlockingQueue<M>>,
     _p: P,
}
impl<P,M> Connector<P,M> where
    M: Msg + Debug + Clone + Send + Default + 'static,
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> 
        // Q: Debug + Send + Sync + Default
{    
    pub fn new() -> Connector<P,M> {
        Connector {
          snd_queue: Arc::new(BlockingQueue::<M>::new()),
          rcv_queue: Arc::new(BlockingQueue::<M>::new()),
          _p: P::default(),
        }
    }
    pub fn post_message(&self, msg: M) {
        self.snd_queue.en_q(msg);
    }
    pub fn get_message(&self) -> M {
        self.rcv_queue.de_q()
    }
    pub fn has_msg(&self) -> bool {
        self.rcv_queue.len() > 0
    }
    pub fn start(&'static self, addr: &'static str) -> Result<()>
        where  P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> {
        let stream = TcpStream::connect(addr)?;
        let mut buf_writer = BufWriter::new(stream.try_clone()?);
        let mut buf_reader = BufReader::new(stream);

        let ssq = Arc::clone(&self.snd_queue);
        let _ = std::thread::spawn(move || {
            loop {
                let msg = ssq.de_q();
                let _ = P::buf_send_message(&msg, &mut buf_writer);
            }            
        });
        let srq = Arc::clone(&self.rcv_queue);
        let _ = std::thread::spawn(move || {
            loop {
                /*-- enqueue reply messages for application --*/
                // let _ = P::buf_recv_message(&mut buf_reader, &self.rcv_queue);
                let _ = P::buf_recv_message(&mut buf_reader, &srq);
                // if rslt.is_err() {
                //     print!("\n  receive error in Connector");
                // }
            }
        });
        Ok(())
    }
    // pub fn has_message(&self) -> bool {
    //     self.rcv_queue.len() > 0
    // }
    // pub fn get_message(&self) -> M {
    //     self.rcv_queue.de_q()
    // }
    // pub fn post_message(&self, msg: M) {
    //     self.snd_queue.en_q(msg);
    // }
}
#[derive(Debug)]
pub struct Listener<P,M> 
where 
P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M>,
M: Msg + Debug + Clone + Send + Default,
    // Q: Debug + Send + Sync + Default
{
    //snd_queue: Arc<BlockingQueue<M>>,
    rcv_queue: Arc<BlockingQueue<M>>,
    p: P,
    _m: M,
    // _q: Q,
}
impl<P,M> Listener<P,M> 
where 
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> + Process<M>,
    M: Msg + Debug + Clone + Send + Sync + Default,
    // Q: Debug + Send + Sync + Default
{    
    pub fn new() -> Listener<P,M> {
        Listener {
            // snd_queue: Arc::new(BlockingQueue::<M>::new()),
            rcv_queue: Arc::new(BlockingQueue::<M>::new()),
              p: P::default(),
            _m: M::default(),
            // _q: Q::default(),
        }
    }
    pub fn start(&'static self, addr: &str) -> Result<()> 
    // where 
    //     M: Debug + Copy + Clone + Send + Default,
    //     P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M,Q> + Process<M>
    {
        let tcpl = TcpListener::bind(addr)?;
        for stream in tcpl.incoming() {
            let strm = stream.unwrap();
            let mut buf_writer = BufWriter::new(strm.try_clone()?);
            let mut buf_reader = BufReader::new(strm.try_clone()?);
    
            let _ = std::thread::spawn(move || {
                loop {
                    let _ = P::buf_recv_message(&mut buf_reader, &self.rcv_queue);
                    let msg = self.rcv_queue.de_q();
                    let msg = self.p.process_message(&msg);
                    let _ = P::buf_send_message(msg, &mut buf_writer);
                    // P::send_message(msg, &mut strm);
                }            
            });
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
