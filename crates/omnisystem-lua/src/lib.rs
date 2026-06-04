use bonsai_language_frontend::LanguageFrontend;
use frontend::LuaFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Lua",
        factory: || Box::new(LuaFrontend::new()),
    }
}
