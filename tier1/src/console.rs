use std::sync::OnceLock;
use log::{Level, LevelFilter, Metadata, Record};


pub struct Console;


impl log::Log for Console {
	fn enabled( &self, metadata: &Metadata ) -> bool {
		metadata.level() <= Level::Info
	}

	fn log( &self, record: &Record ) {
		if self.enabled( record.metadata() ) {
			println!( "{} - {}", record.level(), record.args() );
		}
	}

	fn flush( &self ) { }
}

pub fn console() -> &'static Console {
	let console = _CONSOLE.get_or_init( || Console { } );

	log::set_logger( &_CONSOLE ).unwrap();
	log::set_max_level( LevelFilter::Trace );

	console
}

static _CONSOLE: OnceLock<Console> = OnceLock::new();
