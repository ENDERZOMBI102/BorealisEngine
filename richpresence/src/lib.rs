use std::path::Path;
use tier0::config_file::ConfigFile;

pub mod discord;
pub mod steam;


pub enum RichPresenceType {
	Discord,
	Steam,
	Custom { name: String }
}

pub enum RichPresenceStatus {
	/**
	 * The client is connected to the richpresence server.
	 */
	Connected,
	/**
	 * The client is not connected to the richpresence server.
	 */
	NotConnected,
	/**
	 * The client tried to connect to the richpresence server but failed.
	 */
	FailedToConnect,
	/**
	 * The client tried to connect to the richpresence server but the server wasn't running.
	 */
	ServerNotRunning
}


pub trait RichPresence {
	fn tick();
	fn is_connected() -> bool;
	fn init();
	fn connect() -> RichPresenceStatus;
	fn get_status() -> RichPresenceStatus;
	fn get_server_type() -> RichPresenceType;
}

pub fn get_rp_config() -> ConfigFile {
	ConfigFile::new( Path::new("./cfg/richpresence.cfg") )
}
