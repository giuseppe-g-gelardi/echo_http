pub mod echo;
pub mod response;
pub mod request_config;

pub use request_config::RequestConfig;
pub use echo::Echo;
pub use response::Response;


/// Optional value `Nope`
/// `Option<()> = None;` to circimvent passing `None::<()>` into a post request 
/// with no body/data
#[allow(non_upper_case_globals)]
pub const Nope: Option<()> = None;
