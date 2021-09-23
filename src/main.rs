use tier0::command_line;
/*
game executable
*/
fn main() {
    println!("Arguments:");
    let cmdline = command_line::get_instance();
    for arg in cmdline.get_params() {
        println!( " - key: {} value: {}", arg.get_key(), if arg.has_value() { arg.get_value() } else { "no value" } );
    }
}
