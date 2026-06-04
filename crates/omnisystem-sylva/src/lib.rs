pub mod parser;
pub mod compiler;
pub mod vm;
pub mod frontend;

pub fn register_sylva() {
    tracing::info!("Sylva scripting language support initialized");
}
