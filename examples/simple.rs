// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_variables)]
#![allow(dead_code)]

use etpproto::message::EtpMessageHandler;
use etpproto::message::MessageHeaderFlag;
use etptypes::protocols::ProtocolMessage;
use std::time::SystemTime;
use etptypes::helpers::time_to_etp;
use etptypes::energistics::etp::v12::protocol::core::ping::Ping;
use etptypes::energistics::etp::v12::protocol::core::pong::Pong;
use etptypes::energistics::etp::v12::datatypes::message_header::MessageHeader;
use serde_json;
use std::collections::HashMap;

use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;

use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;

use etpproto::capabilities_utils::negotiate_capabilities;
use etpproto::credentials::create_client_info;
use etpproto::uri::canonical_data_object_uris;

struct MyHandler{

}

impl EtpMessageHandler<Ping> for MyHandler{
    fn handle(header: MessageHeaderFlag, msg: Ping) -> Option<Vec<ProtocolMessage>>{
        Some(
            vec!(ProtocolMessage::Core_Pong(Pong::default()))
        )
        
    }
}

impl EtpMessageHandler<Pong> for MyHandler{
    fn handle(header: MessageHeaderFlag, msg: Pong) -> Option<Vec<ProtocolMessage>>{
        Some(
            vec!(ProtocolMessage::Core_Pong(Pong::default()))
        )
        
    }
}

fn test(){
    let now = SystemTime::now();
    let ping = Ping{
        current_date_time: time_to_etp(now)
    };

    println!("{:?}", MyHandler::handle(MessageHeaderFlag::default(), ping));
    println!("{:?}", MyHandler::handle(MessageHeaderFlag::default(), Pong::default()));
}


fn main() {
    println!("{:?}", canonical_data_object_uris());
    println!("{:?}", create_client_info(None, None, None,));
    println!("{:?}", create_client_info(None, None, None,));
    println!(
        "{:?}",
        format!("{}", EndpointCapabilityKind::ActiveTimeoutPeriod)
    );

    let ma = HashMap::from([
        (
            "ActiveTimeoutPeriod".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(1)),
            },
        ),
        (
            "Nimp".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(2)),
            },
        ),
        (
            "Nimp2".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(3)),
            },
        ),
    ]);

    let mb = HashMap::from([
        (
            "ActiveTimeoutPeriod".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(4)),
            },
        ),
        (
            "Nimp".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(6)),
            },
        ),
    ]);

    println!("{:?}", negotiate_capabilities(ma, mb));

    let a: Result<EndpointCapabilityKind, _> =
        serde_json::from_str(r#""MaxWebSocketMessagePayloadSize""#);
    print!("{:?}", a);

    let mh = MessageHeader{
        protocol:0,
        message_type:1,
        correlation_id:2,
        message_id:3,
        message_flags:4,
    };

    test();
}
