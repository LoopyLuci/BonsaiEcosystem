use language_system::LanguageFrontend;
use frontend::ClojureFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Clojure",
        factory: || Box::new(ClojureFrontend::new()),
    }
}
