use language_system::LanguageFrontend;
use frontend::GraphQLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "GraphQL",
        factory: || Box::new(GraphQLFrontend::new()),
    }
}
