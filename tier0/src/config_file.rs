use std::env::VarError;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;


pub enum Pair {
    Float { key: String, value: f64 },
    Int { key: String, value: i64 },
    String { key: String, value: String },
    Color { key: String, value: Vec<i8> }
}


pub struct ConfigFile {
    items: Vec<Pair>
}

impl ConfigFile {
    pub fn new( path: &Path ) -> ConfigFile {
        assert!( path.exists() );
        let mut file: String = String::new();
        let size = File::open( path ).unwrap().read_to_string(&mut file );
	    if size.is_err() {
	        panic!( "WTF! {}", size.err().unwrap() );
        }

        let mut config = ConfigFile { items: Vec::new() };

        for line in file.split( "\n" ) {
            let pair: Vec<&str> = line.split(" ").collect();
            let name: String = String::from( pair.get(0).unwrap().deref().trim() );
            let value: String = String::from( pair.get(1).unwrap().deref().trim() );

	        println!( "key: {}, value: {}", name.escape_default(), value.escape_default() );

	        let float_res = value.parse::<f64>();
	        if float_res.is_ok() && value.contains(".") {
				config.items.push( Pair::Float { key: name.clone(), value: float_res.unwrap() } );
	        } else {
		        let int_res = value.parse::<i64>();

		        if int_res.is_ok() {
			        config.items.push( Pair::Int { key: name.clone(), value: int_res.unwrap() } );
		        } else {
			        // its either color or a string
			        let color_res: Vec<i8> = value.split_whitespace().map( |value| value.parse::<i8>().unwrap() ).collect();
			        if color_res.len() == 3 || color_res.len() == 4 {
				        config.items.push( Pair::Color { key: name.clone(), value: color_res } );
			        } else {
				        config.items.push( Pair::String { key: name.clone(), value: value.clone() } );
			        }
		        }
	        }
        }

        return config;
    }

	pub fn iterator( &self ) -> impl Iterator< Item = &Pair > {
		return ( &self.items ).into_iter();
	}

	pub fn get( &self, key: &str ) -> Result< &Pair, VarError > {
		for pair in &self.items {
			match pair {
				Pair::Float { key: _key, value: _ } => {
					if _key == key {
						return Result::Ok( &pair );
					}
				}
				Pair::Int { key: _key, value: _ } => {
					if _key == key {
						return Result::Ok( &pair );
					}
				}
				Pair::String { key: _key, value: _ } => {
					if _key == key {
						return Result::Ok( &pair );
					}
				}
				Pair::Color { key: _key, value: _ } => {
					if _key == key {
						return Result::Ok( &pair );
					}
				}
			}

		}
		return Result::Err( VarError::NotPresent );
	}
}
