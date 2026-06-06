use language_system::LanguageFrontend;
use frontend::JuliaFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Julia",
        factory: || Box::new(JuliaFrontend::new()),
    }
}
