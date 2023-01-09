use log::{Level, LevelFilter, Metadata, Record};

pub struct Console {
	initialized: bool
}

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
	unsafe {
		if !_CONSOLE.initialized {
			_CONSOLE = Console { initialized: true };
			log::set_logger( &_CONSOLE ).unwrap();
			log::set_max_level( LevelFilter::Trace );
		}
		&_CONSOLE
	}
}

static mut _CONSOLE: Console = Console { initialized: false };
