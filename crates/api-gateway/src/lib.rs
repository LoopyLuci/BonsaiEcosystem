pub fn gateway_ready() -> bool { true }

#[cfg(test)]
mod tests { use super::*; #[test] fn smoke() { assert!(gateway_ready()); } }
