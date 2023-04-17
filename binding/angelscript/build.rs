use cmake;

fn main() {
	let dst = cmake::build( "vendor/sdk/angelscript/projects/cmake" );

	println!( "cargo:warning={:?}!", dst );

	println!( "cargo:rerun-if-changed=src/lib.rs" );
	println!( "cargo:rustc-link-search=native={}/lib", dst.display() );
	println!( "cargo:rustc-link-lib=static=angelscript{}", if true { "d" } else { "" } );
}
