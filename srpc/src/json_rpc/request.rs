use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub jsonrpc: Version,
    pub method: String,
    pub params: Value,
    pub id: Option<Id>,
}

impl TryFrom<&[u8]> for Request {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice::<Request>(data).map_err(|e| {
            Error::new(
                ErrorKind::InvalidRequest,
                Some(serde_json::to_value(&e.to_string()).unwrap()),
            )
        })
    }
}

impl Request {
    pub fn new(method: String, params: Value, id: Option<Id>) -> Self {
        Self {
            jsonrpc: Version::V2,
            method,
            params,
            id,
        }
    }

    pub fn new_call(method: String, params: Value, id: Id) -> Self {
        Self {
            jsonrpc: Version::V2,
            method,
            params,
            id: Some(id),
        }
    }

    pub fn new_notif(method: String, params: Value) -> Self {
        Self {
            jsonrpc: Version::V2,
            method,
            params,
            id: None,
        }
    }
}
