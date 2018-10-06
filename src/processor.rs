use backend::{
    prelude::{Capability, PathBuf}, stream,
};
use multiqueue::{BroadcastFutReceiver, BroadcastFutSender};
use std::{fmt, sync::Arc};

// sketch for processors:
//
// they live from the moment they're needed to the moment they're not
// often that will be the entirety of the program
// i.e. they're very much stateful
//
// prelims (processor declares):
// - whether it will operate on one backend's output or many/all
// - what capabilities it needs
// - what capabilities it provides
//
// methods:
//   - here's a new arc clone of watched paths
//   - finish up
//
// inputs:
// - stream of events
// - instruction channel
//
// outputs:
// - stream of events
// - instructions
//   - watch this
//   - unwatch this

pub trait Processor: fmt::Debug {
    fn needs_capabilities() -> Vec<Capability>;
    fn provides_capabilities() -> Vec<Capability>;

    fn new(
        events_in: BroadcastFutReceiver<stream::Item>,
        events_out: BroadcastFutSender<stream::Item>,
        instruct: BroadcastFutSender<Instruction>,
        // consider:
        // instruct_in: Receiver<Enum { UpdateWatches(Arc<Vec>), Finish }>
        // instead of the methods, then treat the entire thing as a Future
    ) -> Result<Box<Self>, stream::Error>;
    fn spawn(&mut self); // -> Future

    fn update_watches(&mut self, paths: Arc<Vec<PathBuf>>) -> Result<(), stream::Error>;
    fn finish(&mut self);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    AddWatch(Vec<PathBuf>),
    RemoveWatch(Vec<PathBuf>),
}

// the processor definition lives in the notify core
// because they're really only useful with notify,
// whereas the backend definition is split into a crate
// because it's feasible that something could use a
// backend directly without going through notify core.
