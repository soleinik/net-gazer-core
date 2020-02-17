
use std::any::Any;
use std::sync::mpsc::Sender;

pub type CoreMessage = (u8, Vec<u8>);
pub type CoreSender = Sender<CoreMessage>;

pub trait Plugin<T>: Any + Send + Sync{

    fn on_load(&self);
    fn on_unload(&self);

    fn get_name(&self) -> &str;
    fn get_id(&self) -> u8;

    fn process(&self, tx:&CoreSender, data:&T);
}

