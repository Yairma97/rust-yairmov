
use util::i18n::i18n;

#[derive(thiserror::Error, Debug)]
#[error("{}", i18n("something-wrong"))]
pub struct DomainError {
    #[from]
    source: anyhow::Error,
}
