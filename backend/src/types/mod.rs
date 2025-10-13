pub mod json;
pub(super) mod macros;
pub mod ron;
mod serde;

use mime::Mime;
pub use serde::Serde;

trait ContentType {
    const NAME: &str;
    fn content_type(mime: &Mime) -> bool;
}
trait SerializeResponder {
    type SerError;
    fn serialize(&self) -> Result<String, Self::SerError>;
}
trait DeserializeHandler<T>: Sized {
    type DeError;
    fn deserialize(value: &str) -> Result<T, Self::DeError>;
}
