use bonsai_language_frontend::LanguageFrontend;
use frontend::MATLABFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "MATLAB",
        factory: || Box::new(MATLABFrontend::new()),
    }
}
