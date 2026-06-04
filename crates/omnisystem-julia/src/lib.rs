use bonsai_language_frontend::LanguageFrontend;
use frontend::JuliaFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Julia",
        factory: || Box::new(JuliaFrontend::new()),
    }
}
