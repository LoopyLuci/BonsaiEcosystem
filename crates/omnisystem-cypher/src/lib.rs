use bonsai_language_frontend::LanguageFrontend;
use frontend::CypherFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Cypher",
        factory: || Box::new(CypherFrontend::new()),
    }
}
