use cmake;

fn main() -> miette::Result<()> {
	let dst = cmake::build( "vendor/sdk/angelscript/projects/cmake" );

	println!( "cargo:warning={:?}!", dst );

	let mut b = autocxx_build::Builder::new( "src/lib.rs", &[ format!("{}/include", dst.display() ) ] ).build()?;
	// This assumes all your C++ bindings are in main.rs
	b.flag_if_supported("-std=c++14").compile("as-cxx");


	println!( "cargo:rerun-if-changed=src/lib.rs" );
	println!( "cargo:rustc-link-search=native={}/lib", dst.display() );
	#[cfg(debug_assertions)]
	println!( "cargo:rustc-link-lib=static=angelscript{}", if std::env::consts::OS == "windows" { "d" } else { "" } );
	#[cfg(not(debug_assertions))]
	println!( "cargo:rustc-link-lib=static=angelscript" );

	Ok(())
}
