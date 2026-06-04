use bonsai_language_frontend::LanguageFrontend;
use frontend::GraphQLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "GraphQL",
        factory: || Box::new(GraphQLFrontend::new()),
    }
}
