use std::path::Path;

use filesystem::layered::{LayeredFS, LayeredFSError};

pub(crate) fn layerHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	args.remove(0); // remove "layer" prefix
	match args.as_slice() {
		[ "help" ] => {
			println!( "Layer manager v1.1" );
			println!( "usage: layer ( help | list | reverse | ( append | prepend ) $PATH )" );
			println!( "\thelp              prints this message" );
			println!( "\tappend     $PATH  Adds a layer to the end of the fs, may be a path to a folder or .upkf/.vpk file" );
			println!( "\tprepend    $PATH  Adds a layer to the start of the fs, may be a path to a folder or .upkf/.vpk file" );
			println!( "\tlist              list all available layers" );
			println!( "\treverse           reverse the order of the layer list" );
		}
		// [ "list" ] => println!( "Layer in pos {}: {}", fs.layers.as_slice().index_of( layer ), layer.meta().filename ),
		[ "reverse" ] => {
			fs.layers.reverse();
			println!("Layer list successfully reversed.")
		}
		[ action @ ( "append" | "prepend" ), rawPath ] => {
			let path = Path::new(rawPath).canonicalize().unwrap();

			if let Err(err) = fs.add_layer( path, *action == "prepend" ) {
				match err {
					LayeredFSError::NoExtension => eprintln!("ERROR: If adding a file, please make sure it has a valid extension."),
					LayeredFSError::Unsupported(ext) => eprintln!( "ERROR: Unsupported file type: {ext}" )
				}
				return;
			}

			println!( "{action}ded {:?} as new layer", path )
		}
		_ => eprintln!( "usage: layer ( help | list | reverse | ( append | prepend ) $PATH )" )
	}
}
