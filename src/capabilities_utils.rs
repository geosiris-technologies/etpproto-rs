// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]

use std::collections::HashMap;

use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;

pub fn negotiate_capabilities(
    ca: HashMap<String, DataValue>,
    cb: HashMap<String, DataValue>,
) -> HashMap<String, DataValue> {
    let mut nego = HashMap::new();

    let caps_kinds_names: Vec<String> = vec![
        format!("{}", EndpointCapabilityKind::active_timeout_period),
        format!("{}", EndpointCapabilityKind::authorization_details),
        format!("{}", EndpointCapabilityKind::change_propagation_period),
        format!("{}", EndpointCapabilityKind::change_retention_period),
        format!("{}", EndpointCapabilityKind::max_concurrent_multipart),
        format!("{}", EndpointCapabilityKind::max_data_object_size),
        format!("{}", EndpointCapabilityKind::max_part_size),
        format!("{}", EndpointCapabilityKind::max_session_client_count),
        format!("{}", EndpointCapabilityKind::max_session_global_count),
        format!(
            "{}",
            EndpointCapabilityKind::max_web_socket_frame_payload_size
        ),
        format!(
            "{}",
            EndpointCapabilityKind::max_web_socket_message_payload_size
        ),
        format!(
            "{}",
            EndpointCapabilityKind::multipart_message_timeout_period
        ),
        format!("{}", EndpointCapabilityKind::response_timeout_period),
        format!("{}", EndpointCapabilityKind::request_session_timeout_period),
        format!(
            "{}",
            EndpointCapabilityKind::session_establishment_timeout_period
        ),
        format!(
            "{}",
            EndpointCapabilityKind::supports_alternate_request_uris
        ),
        format!(
            "{}",
            EndpointCapabilityKind::supports_message_header_extensions
        ),
    ];

    for (ca_key, ca_val) in &ca {
        if caps_kinds_names.iter().any(|e| ca_key.contains(e)) {
            for (cb_key, cb_val) in &cb {
                if ca_key == cb_key {
                    println!("NEGO {:?} - {:?}", ca_val, cb_val);
                }
            }
        } else {
            println!("Not in EndpointCapabilityKind {:?}", ca_key);
        }
    }
    nego
}

trait DataValueProperties {
    fn default(&self) -> Option<DataValue> {
        None
    }
    fn min(&self) -> Option<DataValue> {
        None
    }
    fn max(&self) -> Option<DataValue> {
        None
    }
}

impl DataValueProperties for EndpointCapabilityKind {
    fn default(&self) -> Option<DataValue> {
        match self {
            EndpointCapabilityKind::active_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(3600)),
            }),
            // EndpointCapabilityKind::authorization_details => Some(DataValue{item: Some(U_TYPE::Long(1))}), // ArrayOfString
            EndpointCapabilityKind::change_propagation_period => Some(DataValue {
                item: Some(U_TYPE::Long(5)),
            }),
            EndpointCapabilityKind::change_retention_period => Some(DataValue {
                item: Some(U_TYPE::Long(86400)),
            }),
            EndpointCapabilityKind::max_concurrent_multipart => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::response_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(300)),
            }),
            EndpointCapabilityKind::request_session_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(45)),
            }),
            EndpointCapabilityKind::session_establishment_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(3600)),
            }),
            EndpointCapabilityKind::supports_alternate_request_uris => Some(DataValue {
                item: Some(U_TYPE::Boolean(false)),
            }),
            EndpointCapabilityKind::supports_message_header_extensions => Some(DataValue {
                item: Some(U_TYPE::Boolean(false)),
            }),
            _ => None,
        }
    }
    fn min(&self) -> Option<DataValue> {
        match self {
            EndpointCapabilityKind::active_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            EndpointCapabilityKind::change_propagation_period => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::change_retention_period => Some(DataValue {
                item: Some(U_TYPE::Long(86400)),
            }),
            EndpointCapabilityKind::max_concurrent_multipart => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::max_data_object_size => Some(DataValue {
                item: Some(U_TYPE::Long(100000)),
            }),
            EndpointCapabilityKind::max_part_size => Some(DataValue {
                item: Some(U_TYPE::Long(10000)),
            }),
            EndpointCapabilityKind::max_session_client_count => Some(DataValue {
                item: Some(U_TYPE::Long(2)),
            }),
            EndpointCapabilityKind::max_session_global_count => Some(DataValue {
                item: Some(U_TYPE::Long(2)),
            }),
            EndpointCapabilityKind::response_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            EndpointCapabilityKind::request_session_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(5)),
            }),
            EndpointCapabilityKind::session_establishment_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            _ => None,
        }
    }
    fn max(&self) -> Option<DataValue> {
        match self {
            EndpointCapabilityKind::change_propagation_period => Some(DataValue {
                item: Some(U_TYPE::Long(600)),
            }),
            EndpointCapabilityKind::multipart_message_timeout_period => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            _ => None,
        }
    }
}
