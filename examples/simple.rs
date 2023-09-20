// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_variables)]
#![allow(dead_code)]

use etpproto::capabilities_utils::DataObjectCapabilities;
use etpproto::connection::{ConnectionType, EtpConnection};
use etpproto::credentials::create_client_info;
use etpproto::message::{
    decode_message, EtpMessageHandler, Message, MessageHeaderFlag, MSG_FLAG_FINAL,
};

use etptypes::energistics::etp::v12::datatypes::contact::Contact;
use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;
use etptypes::energistics::etp::v12::datatypes::message_header::MessageHeader;
use etptypes::energistics::etp::v12::datatypes::protocol::Protocol;
use etptypes::energistics::etp::v12::datatypes::server_capabilities::ServerCapabilities;
use etptypes::energistics::etp::v12::datatypes::supported_protocol::SupportedProtocol;
use etptypes::energistics::etp::v12::datatypes::uuid::random_uuid;
use etptypes::energistics::etp::v12::protocol::core::open_session::OpenSession;
use etptypes::energistics::etp::v12::protocol::core::ping::Ping;
use etptypes::energistics::etp::v12::protocol::core::pong::Pong;
use etptypes::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException;
use etptypes::energistics::etp::v12::protocol::core::request_session::RequestSession;
use etptypes::error::eunsupported_protocol;
use etptypes::helpers::ETPMetadata;
use etptypes::helpers::{time_to_etp, ETP12VERSION};
use etptypes::protocols::ProtocolMessage;
use serde_json;
use std::collections::HashMap;

use std::time::SystemTime;

struct MyHandler {}

impl EtpMessageHandler for MyHandler {
    fn handle(
        &mut self,
        header: MessageHeaderFlag,
        msg: &ProtocolMessage,
    ) -> Option<Vec<ProtocolMessage>> {
        println!("{:?} <=== ", msg);
        match msg {
            ProtocolMessage::Core_RequestSession(rq) => Some(vec![OpenSession {
                application_name: "A server".to_string(),
                application_version: "1.0.1".to_string(),
                server_instance_id: random_uuid(),
                supported_protocols: vec![],
                supported_data_objects: vec![],
                supported_compression: "".to_string(),
                supported_formats: vec!["xml".to_string()],
                current_date_time: time_to_etp(SystemTime::now()),
                earliest_retained_change_time: time_to_etp(SystemTime::now()),
                session_id: random_uuid(),
                endpoint_capabilities: HashMap::new(),
            }
            .as_protocol_message()]),
            ProtocolMessage::Core_Ping(ping) => Some(vec![Pong::default().as_protocol_message()]),
            ProtocolMessage::Core_Pong(pong) => None,
            _ => Some(vec![ProtocolMessage::Core_ProtocolException(
                ProtocolException::default_with_params(Some(eunsupported_protocol())),
            )]),
        }
    }
}

fn get_request_session() -> RequestSession {
    let protocols = vec![
        SupportedProtocol {
            protocol: Protocol::Core as i32,
            protocol_version: ETP12VERSION,
            role: "Server".to_string(),
            protocol_capabilities: HashMap::new(),
        },
        SupportedProtocol {
            protocol: 3,
            protocol_version: ETP12VERSION,
            role: "Server".to_string(),
            protocol_capabilities: HashMap::new(),
        },
        SupportedProtocol {
            protocol: 4,
            protocol_version: ETP12VERSION,
            role: "Server".to_string(),
            protocol_capabilities: HashMap::new(),
        },
    ];

    let now = SystemTime::now();

    RequestSession {
        application_name: "etp-rs Client Library Application".to_string(),
        application_version: "0.1".to_string(),
        client_instance_id: random_uuid(),
        requested_protocols: protocols,
        supported_data_objects: vec![],
        supported_compression: vec!["gzip".to_string()],
        supported_formats: vec!["xml".to_string(), "json".to_string()],
        current_date_time: time_to_etp(now),
        earliest_retained_change_time: time_to_etp(now),
        server_authorization_required: false,
        endpoint_capabilities: HashMap::new(),
    }
}

/*fn test() {
    let now = SystemTime::now();
    let ping = Ping {
        current_date_time: time_to_etp(now),
    };
    let handler = MyHandler{};
    println!(
        "{:?}",
        handler.handle(MessageHeaderFlag::default(), &ping)
    );
    println!(
        "{:?}",
        handler.handle(MessageHeaderFlag::default(), &Pong::default())
    );
    println!(
        "==> {:?}",
        handle_message(&handler, MessageHeaderFlag::default(), &Pong::default())
    );
}*/

fn main() {
    /* println!("{:?}", canonical_data_object_uris());
    println!("{:?}", create_client_info(None, None, None,));
    println!("{:?}", create_client_info(None, None, None,));
    println!(
        "{:?}",
        format!("{}", EndpointCapabilityKind::ActiveTimeoutPeriod)
        );*/

    let ma = HashMap::from([
        (
            "ActiveTimeoutPeriod".to_string(),
            DataValue {
                item: Some(U_TYPE::Int(666)),
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
                item: Some(U_TYPE::String("142".to_string())),
            },
        ),
        (
            "Nimp".to_string(),
            DataValue {
                item: Some(U_TYPE::Long(6)),
            },
        ),
    ]);

    //println!("{:?}", negotiate_capabilities(ma.clone(), mb));

    let a: Result<EndpointCapabilityKind, _> =
        serde_json::from_str(r#""MaxWebSocketMessagePayloadSize""#);
    print!("{:?}", a);

    let mh = MessageHeader {
        protocol: 0,
        message_type: 1,
        correlation_id: 2,
        message_id: 3,
        message_flags: 4,
    };

    /*test();*/

    let msg = Message::create_message(
        1,
        2,
        MSG_FLAG_FINAL,
        get_request_session().as_protocol_message(),
        None,
    );
    let msg_encoded = msg.encode_message().unwrap();
    // println!("{:?}", msg);
    // println!("{:?}", msg_encoded);
    // println!("{:?}", mem::size_of::<MessageHeader>());

    for m in msg_encoded.iter() {
        //let mut slice = m.as_slice();
        //println!("{:?}", slice);
        decode_message(m.to_vec());
    }

    let mut connection: EtpConnection = EtpConnection::new(
        Some(create_client_info(None, None, None)),
        ConnectionType::Server,
        Some(ServerCapabilities {
            application_name: "etpproto-rs".to_string(),
            application_version: "1.0.0+1.2".to_string(),
            contact_information: Contact {
                organization_name: "Geosiris".to_string(),
                contact_name: "Valentin Gauthier".to_string(),
                contact_phone: "007".to_string(),
                contact_email: "valentin.gauthier@geosiris.com".to_string(),
            },
            supported_compression: vec![],
            supported_encodings: vec![],
            supported_formats: vec!["xml".to_string()],
            supported_data_objects: vec![],
            supported_protocols: vec![],
            endpoint_capabilities: ma,
        }),
        MyHandler {},
    );

    /* Messages */
    let now = SystemTime::now();
    let rq = RequestSession::default();
    let ping = Ping {
        current_date_time: time_to_etp(now),
    };
    let pong = Pong::default();

    println!(
        "Response for ping : {:?} \n => Connected? {:?}",
        connection.handle_message(
            &MessageHeader {
                protocol: rq.protocol(),
                message_type: rq.message_type(),
                correlation_id: 1,
                message_id: 1,
                message_flags: 0,
            },
            &ping.as_protocol_message()
        ),
        connection.is_connected
    );

    println!(
        "Response for pong : {:?} \n => Connected? {:?}",
        connection.handle_message(
            &MessageHeader {
                protocol: rq.protocol(),
                message_type: rq.message_type(),
                correlation_id: 1,
                message_id: 1,
                message_flags: 0,
            },
            &pong.as_protocol_message()
        ),
        connection.is_connected
    );

    println!(
        "Response for RequestSession : {:?} \n => Connected? {:?}",
        connection.handle_message(
            &MessageHeader {
                protocol: rq.protocol(),
                message_type: rq.message_type(),
                correlation_id: 1,
                message_id: 1,
                message_flags: 0,
            },
            &rq.as_protocol_message()
        ),
        connection.is_connected
    );

    println!(
        "Response for ping : {:?} \n => Connected? {:?}",
        connection.handle_message(
            &MessageHeader {
                protocol: rq.protocol(),
                message_type: rq.message_type(),
                correlation_id: 1,
                message_id: 1,
                message_flags: 0,
            },
            &ping.as_protocol_message()
        ),
        connection.is_connected
    );

    println!("{}", DataObjectCapabilities::SupportsGet);
}
