use language_system::LanguageFrontend;
use frontend::LuaFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Lua",
        factory: || Box::new(LuaFrontend::new()),
    }
}
