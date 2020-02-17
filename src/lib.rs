
use std::any::Any;
use std::sync::mpsc::Sender;
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::NetworkInterface;

pub type CoreMessage = (u8, Vec<u8>);
pub type CoreSender = Sender<CoreMessage>;

pub trait Plugin: Any + Send + Sync{

    fn on_load(&self, iface:&NetworkInterface);
    fn on_unload(&self);

    fn get_name(&self) -> &str;
    fn get_id(&self) -> u8;

    fn process(&self, tx:CoreSender, data:&'_ EthernetPacket);
}

