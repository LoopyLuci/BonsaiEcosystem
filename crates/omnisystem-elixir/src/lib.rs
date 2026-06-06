use language_system::LanguageFrontend;
use frontend::ElixirFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Elixir",
        factory: || Box::new(ElixirFrontend::new()),
    }
}
