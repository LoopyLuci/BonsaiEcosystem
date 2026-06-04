use bonsai_language_frontend::LanguageFrontend;
use frontend::RubyFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Ruby",
        factory: || Box::new(RubyFrontend::new()),
    }
}
