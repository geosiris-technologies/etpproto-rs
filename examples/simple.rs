// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
use serde_json;
use std::collections::HashMap;

use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;

use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;

use etpproto::capabilities_utils::negotiate_capabilities;
use etpproto::credentials::create_client_info;
use etpproto::uri::canonical_data_object_uris;

fn main() {
    println!("{:?}", canonical_data_object_uris());
    println!("{:?}", create_client_info(None, None, None,));
    println!("{:?}", create_client_info(None, None, None,));
    println!(
        "{:?}",
        format!("{}", EndpointCapabilityKind::active_timeout_period)
    );

    let ma = HashMap::from([
        (
            "ActiveTimeoutPeriod".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(1)),
            },
        ),
        (
            "Nimp".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(2)),
            },
        ),
        (
            "Nimp2".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(3)),
            },
        ),
    ]);

    let mb = HashMap::from([
        (
            "ActiveTimeoutPeriod".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(4)),
            },
        ),
        (
            "Nimp".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(6)),
            },
        ),
    ]);

    negotiate_capabilities(ma, mb);

    let a: Result<EndpointCapabilityKind, _> =
        serde_json::from_str(r#""MaxWebSocketMessagePayloadSize""#);
    print!("{:?}", a);
}
