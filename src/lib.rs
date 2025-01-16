pub mod config;
pub mod echo;
pub mod method;
pub mod request;
pub mod response;

pub use config::Config;
pub use echo::Echo;
pub use request::Request;
pub use response::Response;


/// Optional value `Nope`
/// `Option<()> = None;` to circimvent passing `None::<()>` into a post request 
/// with no body/data
#[allow(non_upper_case_globals)]
pub const Nope: Option<()> = None;
// let nope = None::<()>;
