pub mod backend;
pub mod extractor;
pub mod extractors;
pub mod opa;
pub mod openfga;

pub use backend::{AuthzBackend, AuthzError};
pub use extractor::{require_caller_id, CallerId};
pub use extractors::{
    BookReader, BookReaderByBookId, BookWriter, GroupAdmin, HasAuthz, ReviewWriter, SystemAdmin,
};
pub use opa::OpaClient;
pub use openfga::OpenFgaClient;
