pub mod json;
pub(super) mod macros;
pub mod ron;

use mime::Mime;

trait ContentType {
    const NAME: &str;
    fn content_type() -> Mime;
}
trait SerializeResponder {
    type SerError;
    fn serialize(&self) -> Result<String, Self::SerError>;
}
trait DeserializeHandler<T>: Sized {
    type DeError;
    fn deserialize(value: &str) -> Result<T, Self::DeError>;
}
