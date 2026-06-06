use language_system::LanguageFrontend;
use frontend::CypherFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Cypher",
        factory: || Box::new(CypherFrontend::new()),
    }
}
