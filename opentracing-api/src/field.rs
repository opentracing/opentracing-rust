use std::fmt;
use std::error::Error;
use std::str::FromStr;

const FIELD_ERROR_KIND: &str = "error.kind";
const FIELD_ERROR_OBJECT: &str = "error.object";
const FIELD_EVENT: &str = "event";
const FIELD_MESSAGE: &str = "message";
const FIELD_STACK: &str = "stack";

/// The following log fields are recommended for instrumentors who are trying to capture more
/// information about a logged event. Tracers may expose additional features based on these
/// standardized data points.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Fields {
    /// The type or "kind" of an error (only for event="error" logs). E.g., "Exception", "OSError".
    ErrorKind,
    /// The actual Throwable/Exception/Error object instance itself.
    ErrorObject,
    /// A stable identifier for some notable moment in the lifetime of a Span. For instance, a mutex
    /// lock acquisition or release or the sorts of lifetime events in a browser page load described
    /// in the Performance.timing specification. E.g., from Zipkin, "cs", "sr", "ss", or "cr". Or,
    /// more generally, "initialized" or "timed out". For errors, "error".
    Event,
    /// A concise, human-readable, one-line message explaining the event. E.g., "Could not connect
    /// to backend", "Cache invalidation succeeded".
    Message,
    /// A stack trace in platform-conventional format; may or may not pertain to an error.
    Stack,
}

impl Fields {
    /// Returns the string representation for the enum reference variant.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Fields::ErrorKind => FIELD_ERROR_KIND,
            Fields::ErrorObject => FIELD_ERROR_OBJECT,
            Fields::Event => FIELD_EVENT,
            Fields::Message => FIELD_MESSAGE,
            Fields::Stack => FIELD_STACK,
        }
    }
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Fields {
    type Err = ParseFieldsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            FIELD_ERROR_KIND => Ok(Fields::ErrorKind),
            FIELD_ERROR_OBJECT => Ok(Fields::ErrorObject),
            FIELD_EVENT => Ok(Fields::Event),
            FIELD_MESSAGE => Ok(Fields::Message),
            FIELD_STACK => Ok(Fields::Stack),
            _ => Err(ParseFieldsError::UnknownField),
        }
    }
}

/// Describes errors which can happen while parsing into the `Fields` enum.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum ParseFieldsError {
    /// The provided field is not known.
    UnknownField,
}

impl Error for ParseFieldsError {
    fn description(&self) -> &str {
        match *self {
            ParseFieldsError::UnknownField => "Unknown Field",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseFieldsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field_as_str() {
        assert_eq!("error.kind", Fields::ErrorKind.as_str());
        assert_eq!("error.object", Fields::ErrorObject.as_str());
        assert_eq!("event", Fields::Event.as_str());
        assert_eq!("message", Fields::Message.as_str());
        assert_eq!("stack", Fields::Stack.as_str());
    }

    #[test]
    fn test_field_as_string() {
        assert_eq!(String::from("error.kind"), Fields::ErrorKind.to_string());
    }

    #[test]
    fn test_field_from_string() {
        assert_eq!(Ok(Fields::Event), Fields::from_str("event"));
        assert_eq!(Ok(Fields::Message), "message".parse());
        assert_eq!(
            Err(ParseFieldsError::UnknownField),
            Fields::from_str("some_other_field")
        );
    }

}
