use std::string::ToString;
use std::sync::Arc;

use dyon::{Dfn, Module, Runtime, Type};

const MAIN: &str = "fn main() { printt(\"hello from dyon!\n\") }";

#[test]
fn testing() -> Result<(), String> {
	let mut runtime = Runtime::new();
	let mut module = Module::new();

	module.add_str( "printt", printt, Dfn::nl( vec![ Type::Str ], Type::Void ) );
	dyon::load_str( "<code>", Arc::new( MAIN.to_string() ), &mut module )?;

	runtime.run( &Arc::new( module ) )?;
	Ok(())
}

fn printt( rt: &mut Runtime ) -> Result<(), String> {
	let string: String = rt.pop()?;
	print!( "{string}" );
	Ok(())
}
