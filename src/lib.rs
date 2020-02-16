
use std::any::Any;
use std::sync::mpsc::Sender;
use pnet::packet::ethernet::EthernetPacket;


pub type CoreMessage = (u8, Vec<u8>);
pub type CoreTxSender = Sender<CoreMessage>;

pub trait Plugin<'p>: Any + Send + Sync {
    fn get_description() -> String;
    fn get_id() -> u8;
    fn process(tx:&'p CoreTxSender, pkt:&'p EthernetPacket);
}

