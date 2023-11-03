// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct BasicCredential {
    pub login: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct BearerCredential {
    pub token: String,
    pub refresh_url: Option<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum Credentials {
    Basic(BasicCredential),
    Bearer(BearerCredential),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ClientInfo {
    pub uid: u64,
    pub ip: Option<String>,
    pub credentials: Option<Credentials>,
    pub capabilities: HashMap<String, DataValue>,
}

pub fn unique_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, SeqCst);
    assert_ne!(
        id,
        u64::MAX,
        "ID counter has overflowed and is no longer unique"
    );
    id
}

pub fn create_client_info(
    ip: Option<String>,
    credentials: Option<Credentials>,
    capabilities: Option<HashMap<String, DataValue>>,
) -> ClientInfo {
    ClientInfo {
        uid: unique_id(),
        ip,
        credentials,
        capabilities: match capabilities {
            Some(caps) => caps,
            None => HashMap::new(),
        },
    }
}
