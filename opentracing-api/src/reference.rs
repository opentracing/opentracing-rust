use std::fmt;
use std::error::Error;
use std::str::FromStr;

const REF_CHILD_OF: &str = "child_of";
const REF_FOLLOWS_FROM: &str = "follows_from";

/// References provide a namespace for official OpenTracing reference types.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum References {
    /// See http://opentracing.io/spec/#causal-span-references for
    /// more information about ChildOf references.
    ChildOf,
    /// See http://opentracing.io/spec/#causal-span-references for
    /// more information about FollowsFrom references.
    FollowsFrom,
}

impl References {
    /// Returns the string representation for the enum reference variant.
    pub fn as_str(&self) -> &'static str {
        match *self {
            References::ChildOf => REF_CHILD_OF,
            References::FollowsFrom => REF_FOLLOWS_FROM,
        }
    }
}

impl fmt::Display for References {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for References {
    type Err = ParseReferencesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            REF_CHILD_OF => Ok(References::ChildOf),
            REF_FOLLOWS_FROM => Ok(References::FollowsFrom),
            _ => Err(ParseReferencesError::UnknownReference),
        }
    }
}

/// Describes errors which can happen while parsing into the `References` enum.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum ParseReferencesError {
    /// The provided reference is not known.
    UnknownReference,
}

impl Error for ParseReferencesError {
    fn description(&self) -> &str {
        match *self {
            ParseReferencesError::UnknownReference => "Unknown Reference",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseReferencesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reference_as_str() {
        assert_eq!("child_of", References::ChildOf.as_str());
        assert_eq!("follows_from", References::FollowsFrom.as_str());
    }

    #[test]
    fn test_reference_as_string() {
        assert_eq!(String::from("child_of"), References::ChildOf.to_string());
        assert_eq!(
            String::from("follows_from"),
            References::FollowsFrom.to_string()
        );
    }

    #[test]
    fn test_reference_from_string() {
        assert_eq!(Ok(References::ChildOf), References::from_str("child_of"));
        assert_eq!(Ok(References::FollowsFrom), "follows_from".parse());
        assert_eq!(
            Err(ParseReferencesError::UnknownReference),
            References::from_str("some_other_field")
        );
    }

}
