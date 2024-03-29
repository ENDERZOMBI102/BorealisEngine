use log::error;

/**
* game executable
*/
fn main() {
	tier1::console::console();
	match std::env::var("EXEC").unwrap().as_str() {
		"renderer" => renderer::renderer::main(),
		"commandline" => tier0::commandline::main(),
		"discord_rp" => richpresence::discord::main(),
		"discord_rp2" => richpresence::discord::main2(),
		"steam_rp" => richpresence::steam::main(),
		"format" => tier0::format::main(),
		name => error!( "Unrecognized executable name: {}", name )
	}
}
