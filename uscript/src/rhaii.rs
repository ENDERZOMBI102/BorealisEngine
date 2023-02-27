use rhai;
use rhai::{Engine, EvalAltResult};

const MAIN: &str = "printt(\"hello from rhai!\\n\");";


#[test]
fn testing() -> Result<(), Box<EvalAltResult>> {
	let mut engine = Engine::new();
	engine.register_fn( "printt", printt );
	engine.run( MAIN )
}


fn printt( string: &str ) {
	print!( "{string}" );
}
