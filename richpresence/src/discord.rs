use std::{thread, time};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{Activity, Timestamps};
use tier0::config_file::{ConfigFile, Pair};
use crate::get_rp_config;

struct ActivityData {
	state: String,
	details: String,
	start_timestamp: i64,
	end_timestamp: i64,
	large_img_txt: String,
	small_img_txt: String,
	party_id: String,
	party_size: u8,
	party_max: u8,
	join_secret: String
}

pub struct RichPresence {
	activity: ActivityData,
	client_id: u64,
	last_update: i64,
	drpc: Option<DiscordIpcClient>
}

impl RichPresence {
	/*
	Constructs a RichPresence with placeholder data + app tkn
	*/
	pub fn new(client_id: u64) -> RichPresence {
		return Self {
			activity: ActivityData {
				state: "".to_string(),
				details: "".to_string(),
				start_timestamp: 0,
				end_timestamp: 0,
				large_img_txt: "".to_string(),
				small_img_txt: "".to_string(),
				party_id: "".to_string(),
				party_size: 0,
				party_max: 0,
				join_secret: "".to_string()
			},
			client_id,
			last_update: -1,
			drpc: None
		};
	}

	pub fn from_config( cfg: &ConfigFile ) -> Self {
		RichPresence {
			activity: ActivityData {
				state: cfg.get("rp_discord_state").unwrap().string().unwrap_or_else( || "".to_string() ),
				details: cfg.get("rp_discord_details").unwrap().string().unwrap_or_else( || "".to_string() ),
				start_timestamp: cfg.get("rp_discord_start_timestamp").unwrap().integer().unwrap_or_else( || 0 ),
				end_timestamp: cfg.get("rp_discord_end_timestamp").unwrap().integer().unwrap_or_else( || 0 ),
				large_img_txt: cfg.get("rp_discord_large_image_text").unwrap().string().unwrap_or_else( || "".to_string() ),
				small_img_txt: cfg.get("rp_discord_small_image_text").unwrap().string().unwrap_or_else( || "".to_string() ),
				party_id: cfg.get("rp_discord_party_id").unwrap().string().unwrap_or_else( || "".to_string() ),
				party_size: cfg.get("rp_discord_party_size").unwrap().integer().unwrap_or_else( || 0 ) as u8,
				party_max: cfg.get("rp_discord_party_max").unwrap().integer().unwrap_or_else( || 0 ) as u8,
				join_secret: cfg.get("rp_discord_join_secret").unwrap().string().unwrap_or_else( || "".to_string() )
			},
			client_id: cfg.get("rp_discord_token").unwrap().integer().unwrap_or_else( || 0 ) as u64,
			last_update: 0,
			drpc: None
		}
	}

	pub fn set_state(&mut self, state: &str) {
		self.activity.state = String::from( state );
	}

	pub fn set_client_id(&mut self, client_id: u64) {
		self.client_id = client_id;
	}

	pub fn tick( &mut self ) {
		match self.drpc {
			Some( ref mut drpc ) => {
				// TODO:  Convert to use engine variables
				drpc.set_activity( Activity::new()
					.state( self.activity.state.as_str() )
					.details( self.activity.details.as_str() )
					.timestamps(
						Timestamps::new()
							.start( self.activity.start_timestamp )
							.end( self.activity.end_timestamp )
					)
				).expect("Failed to set activity");
			}
			None => self.drpc = Some( DiscordIpcClient::new(&*self.client_id.to_string() ).unwrap() )
		}
	}
}

pub fn main() {
	let id: i64;
	let config = get_rp_config();

	// get discord richpresence token
	match config.get("rp_discord_token") {
		Some( pair ) => {
			match pair {
				Pair::Int { key: _key, value } => id = *value,
				_ => panic!( "Discord richpresence token is not a number!" )
			}
		},
		None => panic!( "Missing discord richpresence token in richpresence.cfg!" )
	}

	// Create the client
	let mut drpc = DiscordIpcClient::new(&*id.to_string() ).unwrap();

	// Start up the client connection, so that we can actually send and receive stuff
	drpc.connect().unwrap();

	// Set the activity
	drpc.set_activity( Activity::new()
		.state( config.get( "rp_state" ).unwrap().string().unwrap().as_str() )
		.details( config.get( "rp_details" ).unwrap().string().unwrap().as_str() )
	).expect("Failed to set activity");

	// Wait 10 seconds before exiting
	thread::sleep( time::Duration::from_secs(1000) );
	drpc.close();
}

pub fn main2() {
	let mut rpc = RichPresence::from_config( &get_rp_config() );
	loop {
		rpc.tick();
		thread::sleep( time::Duration::from_secs(10) );
	}
}