use bonsai_language_frontend::LanguageFrontend;
use frontend::ErlangFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Erlang",
        factory: || Box::new(ErlangFrontend::new()),
    }
}
