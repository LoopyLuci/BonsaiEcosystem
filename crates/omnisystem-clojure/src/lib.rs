use bonsai_language_frontend::LanguageFrontend;
use frontend::ClojureFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Clojure",
        factory: || Box::new(ClojureFrontend::new()),
    }
}
