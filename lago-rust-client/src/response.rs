use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    current_page: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev_page: Option<i32>,
    total_pages: i32,
    total_count: i32,
}

#[cfg(test)]
mod response_tests {
    use super::*;

    const META: &str = r#"
    {
        "current_page": 1,
        "next_page": null,
        "prev_page": null,
        "total_pages": 1,
        "total_count": 1
    }
    "#;

    #[test]
    fn meta_deserialize_test() {
        let meta: Result<Metadata, serde_json::Error> = serde_json::from_str(&META);

        assert!(meta.is_ok());
    }
}
