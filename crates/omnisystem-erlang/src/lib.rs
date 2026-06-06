use language_system::LanguageFrontend;
use frontend::ErlangFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Erlang",
        factory: || Box::new(ErlangFrontend::new()),
    }
}
