pub mod client;
pub mod extractor;
pub mod extractors;

pub use client::{AuthzError, OpenFgaClient};
pub use extractor::{require_caller_id, CallerId};
pub use extractors::{
    BookReader, BookReaderByBookId, BookWriter, GroupAdmin, HasAuthz, ReviewWriter, SystemAdmin,
};
