use std::{thread, time};
use steamworks::{Client};
use tier0::config_file::Pair;
use crate::get_rp_config;

// TODO: this is broken, the steam chat doesn't show the status, both inside and outside of the game, find why.
pub fn main() {
	let appid: i64;
	let config = get_rp_config();

	// get steam appid
	match config.get("steam_appid") {
		Some( pair ) => {
			match pair {
				Pair::Int { key: _key, value } => appid = *value,
				_ => panic!( "Discord richpresence token is not a number!" )
			}
		},
		None => panic!( "Missing discord richpresence token in richpresence.cfg!" )
	}

	let ( client, single ) = Client::init_app(appid as u32 ).unwrap();

	let value = config.get( "rp_state" ).unwrap().string().unwrap();
	client.friends().set_rich_presence( "steam_display", Some( value.as_str() ) );
	client.friends().set_rich_presence( "status", Some( value.as_str() ) );

	single.run_callbacks();
	thread::sleep( time::Duration::from_secs(1000) );
}