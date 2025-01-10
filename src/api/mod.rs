// Modules

mod post_route;
mod put_route;
mod delete_route;
mod get_route;


//mod get_api;
//pub(crate) use get_api::{get_route, DataTypes};



// Functions
pub use post_route::post_data;
pub use put_route::put_data;
pub use delete_route::delete_route;
pub use get_route::get_route;
