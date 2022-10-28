// ============================================================
// Copyright (C) 2022 - Midden - All Rights Reserved
// ============================================================
//
// Maintainer: John White <john@midden.io>
// ============================================================

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

///
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ErrorCode {
    #[serde(rename = "value_already_exist")]
    ErrorCodeAlreadyExist,
    ErrorCodeInvalidValue,
    ErrorTypeAssert,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorCodeAlreadyExist => write!(f, "value_already_exist"),
            Self::ErrorCodeInvalidValue => write!(f, "invalid_value"),
            Self::ErrorTypeAssert => write!(f, "error_type_assert"),
        }
    }
}

impl TryFrom<&str> for ErrorCode {
    type Error = &'static str;

    fn try_from(err: &str) -> Result<Self, Self::Error> {
        let temp = err.to_lowercase();
        println!("{}", temp);
        match temp.as_str() {
            "value_already_exist" => Ok(Self::ErrorCodeAlreadyExist),
            "invalid_value" => Ok(Self::ErrorCodeInvalidValue),
            "error_type_assert" => Ok(Self::ErrorTypeAssert),
            _ => Err("unknown error string"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LagoErrorDetail {
    #[serde(rename = "code")]
    pub error_code: Option<Vec<ErrorCode>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "error")]
pub struct LagoError {
    #[serde(rename = "status")]
    pub http_status_code: i32,
    #[serde(rename = "message")]
    pub msg: String,
    // #[serde(rename = "error_details")]
    // pub error_details: LagoErrorDetail,
}

impl Default for LagoError {
    fn default() -> Self {
        Self {
            http_status_code: 200,
            msg: "".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    error: LagoError,
}

#[cfg(test)]
mod api_tests {
    use super::*;

    const ERR404: &str = r#"
	{
        "error": {
		    "status": 404,
		    "message": "not found",
		    "error_details": {
			    "code": "value_already_exist"
		    }
        }
	}"#;

    #[test]
    fn api_error_deserialize() {
        let api_error: Result<Error, serde_json::Error> = serde_json::from_str(&ERR404);

        assert!(api_error.is_ok());
    }

    #[test]
    fn err_try_from() {
        let code = ErrorCode::try_from("help");

        assert!(code.is_err());
    }

    #[test]
    fn ok_try_from() {
        let code1 = ErrorCode::try_from("VALUE_already_EXist");

        assert!(code1.is_ok());
        assert_eq!(code1, Ok(ErrorCode::ErrorCodeAlreadyExist));
    }
}
