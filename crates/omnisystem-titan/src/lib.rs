pub mod parser;
pub mod ast;
pub mod typeck;
pub mod lower;
pub mod frontend;

pub fn register_titan() {
    tracing::info!("Titan systems language support initialized");
}
