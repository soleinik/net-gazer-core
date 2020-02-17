
use std::any::Any;
use std::sync::mpsc::Sender;

pub type CoreMessage = (u8, Vec<u8>);
pub type CoreSender = Sender<CoreMessage>;

pub trait Plugin<'p, T>: Any + Send + Sync{
    
    fn on_load(&self);
    fn on_unload(&self);

    fn get_name(&self) -> &'_ str;
    fn get_id(&self) -> u8;

    fn process(&mut self, tx:&'_ CoreSender, data:&'_ T);
}

