use filesystem::layered::LayeredFS;

pub(crate) fn clearHandler( fs: &mut LayeredFS, argv: Vec<&str>, cwd: &mut String ) {
	print!( "\x1B[2J" )
}
