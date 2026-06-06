use language_system::LanguageFrontend;
use frontend::RubyFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Ruby",
        factory: || Box::new(RubyFrontend::new()),
    }
}
