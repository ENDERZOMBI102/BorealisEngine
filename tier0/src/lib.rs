pub mod command_line;

#[crate_type = "dylib"]
pub mod tier0 {
    pub fn hello() -> &'static str {
        return "hello!"
    }
}
