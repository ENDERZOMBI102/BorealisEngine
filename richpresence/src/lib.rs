#![crate_type = "dylib"]

extern crate discord_rpc_client;

use std::{env, thread, time};
use std::path::Path;
use discord_rpc_client::Client;
use std::str::FromStr;
use tier0::config_file::{ConfigFile, Pair};

pub(crate) struct ActivityData {
	state: String,
	details: String,
	start_timestamp: u64,
	end_timestamp: u64,
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
	drpc: Option<Client>
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
			drpc: None
		};
	}

	pub fn set_state(&mut self, state: &str) {
		self.activity.state = String::from_str( state ).unwrap();
	}

	pub fn set_client_id(&mut self, client_id: u64) {
		self.client_id = client_id;
	}

	pub fn tick(&mut self) {
		match self.drpc {
			Some( ref mut _drpc ) => {

			}
			None => {

			}
		}
	}
}

pub fn main() {
	// Get our main status message
	let state_message = env::args().nth(1).expect("Requires at least one argument");

	let mut id: i64;

	match ConfigFile::new( Path::new("C:/Programming/CLionProjects/Ungine.rs/run/test/cfg/richpresence.cfg") ).get("rp_token") {
		Ok( pair ) => {
			match pair {
				Pair::Int {  key, value } => id = *value,
				_ => {
					panic!( "Richpresence token is not a number!" );
				}
			}
		},
		Err( _ ) => {
			panic!( "Missing richpresence token in richpresence.cfg!" );
		}

	}

	// Create the client
	let mut drpc = Client::new( id as u64 );

	// Start up the client connection, so that we can actually send and receive stuff
	drpc.start();

	// Set the activity
	drpc.set_activity( |act| act.state(state_message) ).expect("Failed to set activity");

	// Wait 10 seconds before exiting
	thread::sleep(time::Duration::from_secs(1000));
}