use language_system::LanguageFrontend;
use frontend::MATLABFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "MATLAB",
        factory: || Box::new(MATLABFrontend::new()),
    }
}
