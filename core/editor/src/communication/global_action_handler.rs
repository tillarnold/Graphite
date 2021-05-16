use proc_macros::MessageImpl;

use super::{AsMessage, Message, MessageDiscriminant, MessageHandler};

#[derive(MessageImpl, PartialEq, Clone)]
#[message(Message, Message, Global)]
pub enum GlobalMessage {
	LogInfo,
	LogDebug,
	LogTrace,
}

#[derive(Debug, Default)]
pub struct GlobalActionHandler {}

impl GlobalActionHandler {
	pub fn new() -> Self {
		Self::default()
	}
}

impl MessageHandler<GlobalMessage, ()> for GlobalActionHandler {
	fn process_action(&mut self, message: GlobalMessage, _data: (), _responses: &mut Vec<Message>) {
		// process action before passing them further down
		use GlobalMessage::*;
		match message {
			LogInfo => {
				log::set_max_level(log::LevelFilter::Info);
				log::info!("set log verbosity to info");
			}
			LogDebug => {
				log::set_max_level(log::LevelFilter::Debug);
				log::info!("set log verbosity to debug");
			}
			LogTrace => {
				log::set_max_level(log::LevelFilter::Trace);
				log::info!("set log verbosity to trace");
			}
		}
	}
	actions_fn!(GlobalMessageDiscriminant::LogInfo, GlobalMessageDiscriminant::LogDebug, GlobalMessageDiscriminant::LogTrace,);
}
