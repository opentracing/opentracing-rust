use std::fmt;
use std::error::Error;
use std::str::FromStr;

const TAG_SPAN_KIND_CLIENT: &str = "client";
const TAG_SPAN_KIND_SERVER: &str = "server";
const TAG_SPAN_KIND_PRODUCER: &str = "producer";
const TAG_SPAN_KIND_CONSUMER: &str = "consumer";
const TAG_HTTP_URL: &str = "http.url";
const TAG_HTTP_STATUS_CODE: &str = "http.status_code";
const TAG_HTTP_METHOD: &str = "http.method";
const TAG_PEER_IPV4: &str = "peer.ipv4";
const TAG_PEER_IPV6: &str = "peer.ipv6";
const TAG_PEER_SERVICE: &str = "peer.service";
const TAG_PEER_HOSTNAME: &str = "peer.hostname";
const TAG_PEER_PORT: &str = "peer.port";
const TAG_SAMPLING_PRIORITY: &str = "sampling.priority";
const TAG_SPAN_KIND: &str = "span.kind";
const TAG_COMPONENT: &str = "component";
const TAG_ERROR: &str = "error";
const TAG_DB_TYPE: &str = "db.type";
const TAG_DB_INSTANCE: &str = "db.instance";
const TAG_DB_USER: &str = "db.user";
const TAG_DB_STATEMENT: &str = "db.statement";
const TAG_MSGBUS_DEST: &str = "message_bus.destination";

/// The following span tags are recommended for instrumentors who are trying to capture more
/// semantic information about the spans. Tracers may expose additional features based on these
/// standardized data points. Tag names follow a general structure of namespacing.
///
/// See also [Semantic Conventions](https://github.com/opentracing/specification/blob/master/semantic_conventions.md)
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Tags {
    /// A constant for setting the span kind to indicate that it represents a client span.
    SpanKindClient,
    /// A constant for setting the span kind to indicate that it represents a server span.
    SpanKindServer,
    /// A constant for setting the span kind to indicate that it represents a producer span,
    /// in a messaging scenario.
    SpanKindProducer,
    /// A constant for setting the span kind to indicate that it represents a consumer span,
    /// in a messaging scenario.
    SpanKindConsumer,
    /// HTTP_URL records the url of the incoming request.
    HttpUrl,
    /// HTTP_STATUS records the http status code of the response.
    HttpStatus,
    /// HTTP_METHOD records the http method. Case-insensitive.
    HttpMethod,
    /// PEER_HOST_IPV4 records IPv4 host address of the peer.
    PeerHostIpv4,
    /// PEER_HOST_IPV6 records the IPv6 host address of the peer.
    PeerHostIpv6,
    /// PEER_SERVICE records the service name of the peer.
    PeerService,
    /// PEER_HOSTNAME records the host name of the peer.
    PeerHostname,
    /// PEER_PORT records the port number of the peer.
    PeerPort,
    /// SAMPLING_PRIORITY determines the priority of sampling this Span.
    SamplingPriority,
    /// SPAN_KIND hints at the relationship between spans, e.g. client/server.
    SpanKind,
    /// COMPONENT is a low-cardinality identifier of the module, library, or
    /// package that is instrumented.
    Component,
    /// ERROR indicates whether a Span ended in an error state.
    Error,
    /// DB_TYPE indicates the type of Database.
    DbType,
    /// DB_INSTANCE indicates the instance name of Database.
    DbInstance,
    /// DB_USER indicates the user name of Database, e.g. "readonly_user" or
    /// "reporting_user"
    DbUser,
    /// DB_STATEMENT records a database statement for the given database type.
    DbStatement,
    /// MESSAGE_BUS_DESTINATION records an address at which messages can be
    /// exchanged.
    MessageBusDestination,
}

impl Tags {
    /// Returns the string representation for the enum reference variant.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Tags::SpanKindClient => TAG_SPAN_KIND_CLIENT,
            Tags::SpanKindServer => TAG_SPAN_KIND_SERVER,
            Tags::SpanKindProducer => TAG_SPAN_KIND_PRODUCER,
            Tags::SpanKindConsumer => TAG_SPAN_KIND_CONSUMER,
            Tags::HttpUrl => TAG_HTTP_URL,
            Tags::HttpStatus => TAG_HTTP_STATUS_CODE,
            Tags::HttpMethod => TAG_HTTP_METHOD,
            Tags::PeerHostIpv4 => TAG_PEER_IPV4,
            Tags::PeerHostIpv6 => TAG_PEER_IPV6,
            Tags::PeerService => TAG_PEER_SERVICE,
            Tags::PeerHostname => TAG_PEER_HOSTNAME,
            Tags::PeerPort => TAG_PEER_PORT,
            Tags::SamplingPriority => TAG_SAMPLING_PRIORITY,
            Tags::SpanKind => TAG_SPAN_KIND,
            Tags::Component => TAG_COMPONENT,
            Tags::Error => TAG_ERROR,
            Tags::DbType => TAG_DB_TYPE,
            Tags::DbInstance => TAG_DB_INSTANCE,
            Tags::DbUser => TAG_DB_USER,
            Tags::DbStatement => TAG_DB_STATEMENT,
            Tags::MessageBusDestination => TAG_MSGBUS_DEST,
        }
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Tags {
    type Err = ParseTagsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TAG_SPAN_KIND_CLIENT => Ok(Tags::SpanKindClient),
            TAG_SPAN_KIND_SERVER => Ok(Tags::SpanKindServer),
            TAG_SPAN_KIND_PRODUCER => Ok(Tags::SpanKindProducer),
            TAG_SPAN_KIND_CONSUMER => Ok(Tags::SpanKindConsumer),
            TAG_HTTP_URL => Ok(Tags::HttpUrl),
            TAG_HTTP_STATUS_CODE => Ok(Tags::HttpStatus),
            TAG_HTTP_METHOD => Ok(Tags::HttpMethod),
            TAG_PEER_IPV4 => Ok(Tags::PeerHostIpv4),
            TAG_PEER_IPV6 => Ok(Tags::PeerHostIpv6),
            TAG_PEER_SERVICE => Ok(Tags::PeerService),
            TAG_PEER_HOSTNAME => Ok(Tags::PeerHostname),
            TAG_PEER_PORT => Ok(Tags::PeerPort),
            TAG_SAMPLING_PRIORITY => Ok(Tags::SamplingPriority),
            TAG_SPAN_KIND => Ok(Tags::SpanKind),
            TAG_COMPONENT => Ok(Tags::Component),
            TAG_ERROR => Ok(Tags::Error),
            TAG_DB_TYPE => Ok(Tags::DbType),
            TAG_DB_INSTANCE => Ok(Tags::DbInstance),
            TAG_DB_USER => Ok(Tags::DbUser),
            TAG_DB_STATEMENT => Ok(Tags::DbStatement),
            TAG_MSGBUS_DEST => Ok(Tags::MessageBusDestination),
            _ => Err(ParseTagsError::UnknownTag),
        }
    }
}

/// Describes errors which can happen while parsing into the `Tags` enum.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum ParseTagsError {
    /// The provided tag is not known.
    UnknownTag,
}

impl Error for ParseTagsError {
    fn description(&self) -> &str {
        match *self {
            ParseTagsError::UnknownTag => "Unknown Tag",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tag_as_str() {
        assert_eq!("client", Tags::SpanKindClient.as_str());
    }

    #[test]
    fn test_tag_as_string() {
        assert_eq!(String::from("client"), Tags::SpanKindClient.to_string());
    }

    #[test]
    fn test_tag_from_string() {
        assert_eq!(Ok(Tags::Error), Tags::from_str("error"));
        assert_eq!(Ok(Tags::PeerHostIpv4), "peer.ipv4".parse());
        assert_eq!(
            Err(ParseTagsError::UnknownTag),
            Tags::from_str("some_other_field")
        );
    }

}
