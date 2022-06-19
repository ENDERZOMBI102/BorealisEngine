use log::error;

pub mod kv;
pub mod e;


pub fn main() {
	match std::env::var("FORMAT").unwrap().as_str() {
		"kv" => kv::main(),
		"kv2" => {}
		"kv3" => {}
		"e" => e::main(),
		name => error!( "Unrecognized executable name: {}", name )
	}
}