use bonsai_language_frontend::LanguageFrontend;
use frontend::FORTRAN90Frontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "FORTRAN 90",
        factory: || Box::new(FORTRAN90Frontend::new()),
    }
}
