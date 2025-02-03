pub mod client;
pub mod echo_errors;
pub mod headers;
pub mod request;
pub mod response;

pub use client::Echo;
pub use headers::Headers;
pub use request::RequestConfig;
pub use response::Response;

use echo_errors::EchoError;
pub use response::ResponseUnknown;
pub use response::ParsedResponse;

use once_cell::sync::Lazy;

/// ```rs
/// Default instance, quickly start making https requests without setting up
///
/// let res = echo.get("url").await?;
///
/// works with get, post, put and delete however if you require more granular control,
/// it is suggested to setup your `RequestConfig`
/// ```
#[allow(non_upper_case_globals)]
pub static echo: Lazy<Echo> = Lazy::new(|| Echo::configure(None));
