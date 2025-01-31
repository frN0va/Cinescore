pub mod movie_details;
pub mod movie_lists;

#[macro_export]
macro_rules! generate_request_struct {
    ($request_name:ident) => {
        #[derive(Default)]
        pub struct $request_name {
            params: HashMap<&'static str, String>,
        }

        impl $request_name {
            pub fn new() -> Self {
                Self {
                    params: HashMap::new(),
                }
            }
        }
    };
}
