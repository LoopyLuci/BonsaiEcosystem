use bonsai_language_frontend::LanguageFrontend;
use frontend::FORTRAN77Frontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "FORTRAN 77",
        factory: || Box::new(FORTRAN77Frontend::new()),
    }
}
