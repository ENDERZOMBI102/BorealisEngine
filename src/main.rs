/**
* game executable
*/
fn main() {
	match std::env::var("EXEC").unwrap().as_str() {
		"compressor" => filesystem::compressor::main(),
		"decompressor" => filesystem::decompressor::main(),
		"renderer" => renderer::renderer::main(),
		"commandline" => tier0::commandline::main(),
		"richpresence" => richpresence::main(),
		name => eprintln!( "Unrecognized executable name: {}", name )
	}
}
