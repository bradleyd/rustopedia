use std::env;
pub fn get_model_name() -> String {
    match env::var("RUSTOPEDIA_MODEL_NAME") {
        Ok(val) => val,
        Err(_) => "openhermes".to_string(),
    }
}
