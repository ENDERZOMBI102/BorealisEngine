use filesystem::layered::LayeredFS;

pub(crate) fn clearHandler( fs: &mut LayeredFS, args: Vec<&str>, currentDir: &mut String ) {
	print!( "\x1B[2J" )
}
