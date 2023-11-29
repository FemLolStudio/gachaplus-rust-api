use axum::response::{IntoResponse, Response};

pub struct ResponseManager<'f> {
    params: Vec<(&'f str, &'f str)>,
}

impl<'f> ResponseManager<'f> {
    pub fn new_ok() -> Self {
        Self {
            params: vec![("systemResult", "2")],
        }
    }
    pub fn new_error() -> Self {
        Self {
            params: vec![("systemResult", "3")],
        }
    }
    pub fn to_form_encoded(self) -> String {
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.params)
            .finish()
    }
    pub fn into_response(self) -> Response {
        let encoded_body = self.to_form_encoded();

        Response::builder()
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(encoded_body)
            .unwrap()
            .into_response()
    }
    pub fn add(mut self, name: &'f str, data: &'f str) -> Self {
        self.params.push((name, data));
        self
    }
}
