use language_system::LanguageFrontend;
use frontend::SQLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "SQL",
        factory: || Box::new(SQLFrontend::new()),
    }
}
