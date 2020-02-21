
use std::any::Any;
use std::sync::mpsc::{SyncSender, Receiver};
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::NetworkInterface;

pub type CoreMessage = (u8, Vec<u8>);
pub type CoreSender = SyncSender<CoreMessage>;
pub type CoreReceiver = Receiver<CoreMessage>;

pub trait Plugin: Any{ //} + Send + Sync{

    fn on_load(&mut self, iface:&NetworkInterface, tx:CoreSender);
    fn on_unload(&mut self);

    fn get_name(&self) -> &str;
    fn get_id(&self) -> u8;

    fn process(&self, data:&'_ EthernetPacket);
}


///plugin id is also message id that plugin produces
pub const PLUGIN_ID_DEMO:u8 = 0;
pub const PLUGIN_ID_TRACEROUTE:u8 = 1;
pub const PLUGIN_ID_ALLETHER:u8 = 2;


/// reserved for error messages
pub const PLUGIN_ID_ERROR_MSG:u8 = std::u8::MAX;