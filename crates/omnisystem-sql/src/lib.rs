use bonsai_language_frontend::LanguageFrontend;
use frontend::SQLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "SQL",
        factory: || Box::new(SQLFrontend::new()),
    }
}
