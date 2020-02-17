
use std::any::Any;
use std::sync::mpsc::{Sender, Receiver};
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::NetworkInterface;

pub type CoreMessage = (u8, Vec<u8>);
pub type CoreSender = Sender<CoreMessage>;
pub type CoreReceiver = Receiver<CoreMessage>;

pub trait Plugin: Any{ //} + Send + Sync{

    fn on_load(&mut self, iface:&NetworkInterface, tx:CoreSender);
    fn on_unload(&mut self);

    fn get_name(&self) -> &str;
    fn get_id(&self) -> u8;

    fn process(&self, data:&'_ EthernetPacket);
}

