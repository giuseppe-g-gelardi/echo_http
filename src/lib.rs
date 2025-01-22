pub mod echo_http;
pub mod request_config;
pub mod response;

pub use echo_http::Echo;
pub use request_config::RequestConfig;
pub use response::Response;

use once_cell::sync::Lazy;

/// Optional value `Nope`
/// ```rs
/// Option<()> = None;
/// ```
/// to circimvent passing
/// ```rs
/// None::<()>
/// ```
/// into a post request
/// with no body/data
#[allow(non_upper_case_globals)]
pub const Nope: Option<()> = None;

/// Default instance, quickly start making https requests without setting up
/// a default configuration.
/// # example:
/// ```rs
/// let res = echo.get("url").await?;
/// ```
/// works with get, post, put and delete however if you require more granular control,
/// it is suggested to setup your `RequestConfig`
#[allow(non_upper_case_globals)]
pub static echo: Lazy<Echo> = Lazy::new(|| Echo::configure(None));
