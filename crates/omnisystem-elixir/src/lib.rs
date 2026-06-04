use bonsai_language_frontend::LanguageFrontend;
use frontend::ElixirFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Elixir",
        factory: || Box::new(ElixirFrontend::new()),
    }
}
