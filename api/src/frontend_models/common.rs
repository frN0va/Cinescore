use crate::tmdb::client::IMAGE_BASE_URL;

/// A default placeholder image for images that are missing from API responses, such as profile
/// pictures or movie posters
const DEFAULT_PLACEHOLDER_IMAGE: &str =
    "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png";

/// Given an optional string, either unwrap it and prepend [`IMAGE_BASE_URL`], or return [`DEFAULT_PLACEHOLDER_IMAGE`]
///
/// # Arguments
///
/// * `path` - the path returned by TMDB to convert into an image URL
pub fn get_image_url(path: Option<String>) -> String {
    match path {
        Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
        None => DEFAULT_PLACEHOLDER_IMAGE.to_owned(),
    }
}
