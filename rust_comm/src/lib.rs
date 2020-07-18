// rust_comm::lib.rs

#![allow(unused_imports)]
#![allow(dead_code)]

/*-- module comm_traits: Traits Sndr<M>, Rcvr<M>, Process<M> --*/
use rust_traits::*;
use rust_message::*;
use rust_comm_processing::*;
use rust_blocking_queue::*;

/*-- std library facilities --*/
use std::fmt::*;
use std::sync::{Arc};
use std::net::{TcpStream, TcpListener};
use std::io::{Result, BufReader, BufWriter, stdout, Write};
use std::io::prelude::*;
use std::thread;
use std::thread::{JoinHandle};

pub type M = Message;
pub type P = CommProcessing;

#[derive(Debug)]
pub struct Connector<P,M> where 
    M: Msg + Debug + Clone + Send + Default,
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> 
{
    snd_queue: Arc<BlockingQueue<M>>,
    rcv_queue: Arc<BlockingQueue<M>>,
     _p: P,
     connected: bool,
}
impl<P,M> Connector<P,M> where
    M: Msg + Debug + Clone + Send + Default + 'static,
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> 
{    
    // pub fn new() -> Connector<P,M> {
    //     Connector {
    //       snd_queue: Arc::new(BlockingQueue::<M>::new()),
    //       rcv_queue: Arc::new(BlockingQueue::<M>::new()),
    //       _p: P::default(),
    //     }
    // }
    pub fn is_connected(&self) -> bool {
        self.connected
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
    pub fn new(addr: &'static str) -> Connector<P,M>
        where  P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> {

        let mut connected = false;
        let rslt = TcpStream::connect(addr);
        if rslt.is_err() {
             print!("\n  -- connection to {:?} failed --", addr);
        }
        else {
            connected = true;
        }
        let _ = stdout().flush();
        let stream = rslt.unwrap();
        let mut buf_writer = BufWriter::new(stream.try_clone().unwrap());
        let mut buf_reader = BufReader::new(stream);
        
        let send_queue = Arc::new(BlockingQueue::<M>::new());
        let recv_queue = Arc::new(BlockingQueue::<M>::new());
        
        let sqm = Arc::clone(&send_queue);
        let _ = std::thread::spawn(move || {
            loop {
                let ssq = Arc::clone(&sqm);
                print!("\n  -- dequing send msg --");
                let msg = ssq.de_q();
                print!("\n  sending msg");
                let msg_type = msg.get_type();
                let _ = P::buf_send_message(msg, &mut buf_writer);
                if msg_type == MessageType::QUIT {
                    print!("\n  -- terminating connector send thread --");
                    break;
                }
            }            
        });
        let rqm = Arc::clone(&recv_queue);
        let _ = std::thread::spawn(move || {
            loop {
                let srq = Arc::clone(&rqm);
                let rslt = P::buf_recv_message(&mut buf_reader, &srq);
                if rslt.is_err() {
                    print!("\n  -- terminating connector receive thread --");
                    break;
                }
                // print!("\n  -- attempting to deque reply message --");
                // print!("\n  rcv_queue size = {:?}",srq.len());
                //let msg = srq.de_q();
                // print!("\n  connector received msg: {:?}",msg.get_body_str());

            }
        });
        Self {
            _p: P::default(),
            snd_queue: send_queue,
            rcv_queue: recv_queue,
            connected: connected,
        }
    }
}
#[derive(Debug)]
pub struct Listener<P> 
where 
P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M>,
{
    p: P,
}
impl<P> Listener<P> 
where 
    P: Debug + Copy + Clone + Send + Sync + Default + Sndr<M> + Rcvr<M> + Process<M>,
{    
    pub fn new() -> Listener<P> {
        Listener {
              p: P::default(),
        }
    }
    pub fn start(&mut self, addr: &'static str) -> Result<JoinHandle<()>> 
    {
        print!("\n  -- starting listener --");
        let handle = std::thread::spawn(move || {
            let tcpl = TcpListener::bind(addr).unwrap();
            let rcv_queue = Arc::new(BlockingQueue::<M>::new());
            for stream in tcpl.incoming() {
                let sq = Arc::clone(&rcv_queue);
                let strm = stream.unwrap();
                let mut buf_writer = BufWriter::new(strm.try_clone().unwrap());
                let mut buf_reader = BufReader::new(strm.try_clone().unwrap());
                let _ = std::thread::spawn(move || {
                    loop {
                        let _ = P::buf_recv_message(&mut buf_reader, &sq);
                        let msg = sq.de_q();
                        if msg.get_type() == MessageType::QUIT {
                            break;
                        }
                        let msg = P::process_message(msg);
                        let _ = P::buf_send_message(msg, &mut buf_writer);
                        
                        // P::send_message(msg, &mut strm);
                    }            
                });
            }    
        });
        // }
            // let tcpl = TcpListener::bind(addr)?;
            // let rcv_queue = Arc::new(BlockingQueue::<M>::new());
            // for stream in tcpl.incoming() {
            //     let sq = Arc::clone(&rcv_queue);
            //     let strm = stream.unwrap();
            //     let mut buf_writer = BufWriter::new(strm.try_clone()?);
            //     let mut buf_reader = BufReader::new(strm.try_clone()?);
            //     let _ = std::thread::spawn(move || {
            //         loop {
            //             let _ = P::buf_recv_message(&mut buf_reader, &sq);
            //             let msg = sq.de_q();
            //             let msg = P::process_message(msg);
            //             let _ = P::buf_send_message(msg, &mut buf_writer);
            //             // P::send_message(msg, &mut strm);
            //         }            
            //     });
            // }
        Ok(handle)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
