pub mod echo_http;
pub mod headers;
pub mod request_config;
pub mod response;
pub mod echo_errors;

pub use echo_http::Echo;
pub use headers::Headers;
pub use request_config::RequestConfig;
pub use response::{Response, ResponseUnknown};
pub use echo_errors::EchoError;

use once_cell::sync::Lazy;

/// ```rs
/// Optional value `Nope` = Option<()> = None;
///
/// to circimvent passing `None::<()>` into a post request
/// with no body/data
/// ```
#[allow(non_upper_case_globals)]
pub const Nope: Option<()> = None;

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
