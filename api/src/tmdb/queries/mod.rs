pub mod common;
pub mod movie_details;
pub mod movie_lists;
pub mod people_details;

#[macro_export]
macro_rules! generate_request_struct {
    ($request_name:ident, $docs:expr) => {
        #[doc=$docs]
        #[derive(Default)]
        pub struct $request_name {
            params: std::collections::HashMap<&'static str, String>,
        }

        impl $request_name {
            pub fn new() -> Self {
                Self {
                    params: std::collections::HashMap::new(),
                }
            }
        }
    };
}
