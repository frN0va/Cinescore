use crate::tmdb::client::IMAGE_BASE_URL;

const DEFAULT_PLACEHOLDER_IMAGE: &str =
    "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png";

pub fn get_image_url(path: Option<String>) -> String {
    match path {
        Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
        None => DEFAULT_PLACEHOLDER_IMAGE.to_owned(),
    }
}
