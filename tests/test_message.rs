// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
use etptypes::energistics::etp::v12::datatypes::object::resource::Resource;
use etptypes::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind;
use etptypes::energistics::etp::v12::datatypes::object::data_object::DataObject;

use etptypes::energistics::etp::v12::protocol::store::put_data_objects::PutDataObjects;
use etpproto::message::*;
use etptypes::energistics::etp::v12::datatypes::object::context_info::ContextInfo;
use etptypes::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind;
use etptypes::energistics::etp::v12::datatypes::object::relationship_kind::RelationshipKind;
use etptypes::energistics::etp::v12::datatypes::protocol::Protocol;
use etptypes::energistics::etp::v12::datatypes::supported_protocol::SupportedProtocol;
use etptypes::energistics::etp::v12::datatypes::uuid::random_uuid;
use etptypes::energistics::etp::v12::protocol::core::request_session::RequestSession;
use etptypes::energistics::etp::v12::protocol::discovery::get_resources::GetResources;
use etptypes::helpers::ETP12VERSION;
use etptypes::helpers::time_to_etp;
use std::collections::HashMap;
use std::time::SystemTime;


/*
    ________
   / ____/ /___ _____ ______
  / /_  / / __ `/ __ `/ ___/
 / __/ / / /_/ / /_/ (__  )
/_/   /_/\__,_/\__, /____/
              /____/
*/

#[test]
fn test_message_flags_0() {
    let full_0: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: false,
        msg_compressed: false,
        msg_aknowledge: false,
        msg_has_header_extension: false,
        msg_multipart: false,
        msg_no_data: false,
    };

    assert_eq!(full_0, full_0);

    let int_rep: i32 = full_0.as_i32();

    assert_eq!(int_rep, 0);
}

#[test]
fn test_message_flags_0_parse() {
    let full_0: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: false,
        msg_compressed: false,
        msg_aknowledge: false,
        msg_multipart: false,
        msg_no_data: false,
        msg_has_header_extension: false,
    };
    assert_eq!(full_0, MessageHeaderFlag::parse(full_0.as_i32()));
}

#[test]
fn test_message_flags_1() {
    let full_1: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: true,
        msg_compressed: true,
        msg_aknowledge: true,
        msg_has_header_extension: true,
        msg_multipart: false,
        msg_no_data: false,
    };

    assert_eq!(full_1, full_1);

    let int_rep: i32 = full_1.as_i32();

    assert_eq!(
        int_rep,
        MSG_FLAG_FINAL | MSG_FLAG_COMPRESSED | MSG_FLAG_ACKNOWLEDGE | MSG_FLAG_HEADER_EXTENSION
    );
}

#[test]
fn test_message_flags_1_parse() {
    let full_1: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: true,
        msg_compressed: true,
        msg_aknowledge: true,
        msg_has_header_extension: true,
        msg_multipart: false,
        msg_no_data: false,
    };
    assert_eq!(full_1, MessageHeaderFlag::parse(full_1.as_i32()));
}

#[test]
fn test_message_flags_mix_as_int32() {
    let full_1: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: true,
        msg_compressed: false,
        msg_aknowledge: true,
        msg_multipart: false,
        msg_no_data: false,
        msg_has_header_extension: false,
    };
    let int_rep = full_1.as_i32();

    assert_eq!((int_rep & MSG_FLAG_FINAL), MSG_FLAG_FINAL);
    assert_eq!((int_rep & MSG_FLAG_COMPRESSED), 0);
    assert_eq!((int_rep & MSG_FLAG_ACKNOWLEDGE), MSG_FLAG_ACKNOWLEDGE);
    assert_eq!((int_rep & MSG_FLAG_HEADER_EXTENSION), 0);
}

#[test]
fn test_message_flags_mix_parse() {
    let full_1: MessageHeaderFlag = MessageHeaderFlag {
        msg_final: true,
        msg_compressed: false,
        msg_aknowledge: true,
        msg_has_header_extension: false,
        msg_multipart: false,
        msg_no_data: false,
    };
    let int_rep = full_1.as_i32();
    let full_1_parsed: MessageHeaderFlag = MessageHeaderFlag::parse(int_rep);

    assert_eq!(full_1_parsed.msg_final, true);
    assert_eq!(full_1_parsed.msg_compressed, false);
    assert_eq!(full_1_parsed.msg_aknowledge, true);
    assert_eq!(full_1_parsed.msg_has_header_extension, false);
}

/*
   ______                                          _
  / ____/___  ____ ___  ____  ________  __________(_)___  ____
 / /   / __ \/ __ `__ \/ __ \/ ___/ _ \/ ___/ ___/ / __ \/ __ \
/ /___/ /_/ / / / / / / /_/ / /  /  __(__  |__  ) / /_/ / / / /
\____/\____/_/ /_/ /_/ .___/_/   \___/____/____/_/\____/_/ /_/
                    /_/
*/

#[test]
fn test_message_compression_request_session() {
    let req_sess = get_request_session();
    
    let msg_compressed = &EtpMessage::create_message(0,1, MSG_FLAG_FINAL|MSG_FLAG_COMPRESSED, req_sess.as_protocol_message(), None).encode_message().unwrap()[0];
    let msg = &EtpMessage::create_message(0, 1, MSG_FLAG_FINAL, req_sess.as_protocol_message(), None).encode_message().unwrap()[0];

    assert_eq!(msg_compressed.len(), msg.len());
    assert_eq!(msg_compressed[4], msg[4]); // msg flags is the same : not compressed because no compression on protocol 0
    assert!(msg_compressed.iter().eq(msg.iter()));
}


#[test]
fn test_message_compression_get_resources() {
    let getress = GetResources::default_with_params(
        ContextInfo {
            uri: "eml:///".to_string(),
            depth: 1 as i32,
            data_object_types: vec![],
            navigable_edges: RelationshipKind::Both,
            include_secondary_targets: false,
            include_secondary_sources: false,
        },
        ContextScopeKind::Self_,
        None,
        None,
    );
    
    let msg_compressed = &EtpMessage::create_message(0,1, MSG_FLAG_FINAL|MSG_FLAG_COMPRESSED, getress.as_protocol_message(), None).encode_message().unwrap()[0];
    let msg = &EtpMessage::create_message(0, 1, MSG_FLAG_FINAL, getress.as_protocol_message(), None).encode_message().unwrap()[0];

    assert!(msg_compressed[4] != msg[4]); // msg flags is different
}

#[test]
fn test_message_compression_put_dataobject() {
    let pdo = PutDataObjects{
        data_objects: HashMap::from(
            [("0".to_string(), DataObject{
                resource: Resource::default_with_params("eml:///resqml22.HorizonInterpretation(489f074b-744e-45ce-a03b-521459f35415)".to_string(), Some(0), Some(0), 0,0,0, ActiveStatusKind::Active),
                format: "xml".to_string(),
                blob_id: Some(random_uuid()),
                data: br#"
<ns2:HorizonInterpretation xmlns="http://www.energistics.org/energyml/data/commonv2" xmlns:ns2="http://www.energistics.org/energyml/data/resqmlv2" xmlns:ns3="http://www.w3.org/1999/xlink" xmlns:ns4="http://www.opengis.net/gml/3.2" xmlns:ns5="http://www.isotc211.org/2005/gco" xmlns:ns6="http://www.isotc211.org/2005/gmd" schemaVersion="2.0" uuid="489f074b-744e-45ce-a03b-521459f35415">
    <Citation>
        <Title>Interp_Hugin_Fm_Top</Title>
        <Originator>ATsoblefack</Originator>
        <Creation>2019-01-03T16:24:18Z</Creation>
        <Format>Paradigm SKUA-GOCAD 19 Alpha 2 Build://skua-gocad/Production/trunk - 20190322-cl867561 for Win_x64_6.1_v15</Format>
        <Editor>ATsoblefack</Editor>
        <LastUpdate>2022-08-03T07:31:38.106Z</LastUpdate>
    </Citation>
    <ns2:ExtraMetadata>
        <ns2:Name>pdgm/dx/resqml/creatorGroup</ns2:Name>
        <ns2:Value>ATsoblefack</ns2:Value>
    </ns2:ExtraMetadata>
    <ns2:ExtraMetadata>
        <ns2:Name>pdgm/dx/resqml/project</ns2:Name>
        <ns2:Value>79ae8a84-c896-46f8-81cf-c9a689c5352d</ns2:Value>
    </ns2:ExtraMetadata>
    <ns2:Domain>depth</ns2:Domain>
    <ns2:InterpretedFeature>
        <ContentType>application/x-resqml+xml;version=2.0;type=obj_GeneticBoundaryFeature</ContentType>
        <Title>Hugin_Fm_Top</Title>
        <UUID>5fa99eb4-b11f-4f08-b1be-2d64ff14286f</UUID>
        <UuidAuthority>pdgm</UuidAuthority>
    </ns2:InterpretedFeature>
    <ns2:BoundaryRelation>conformable</ns2:BoundaryRelation>
</ns2:HorizonInterpretation>
"#.to_vec(),
            })]
        ),
        prune_contained_objects: true,
    };
    
    let msg_compressed = &EtpMessage::create_message(0,1, MSG_FLAG_FINAL|MSG_FLAG_COMPRESSED, pdo.as_protocol_message(), None).encode_message().unwrap()[0];
    let msg = &EtpMessage::create_message(0, 1, MSG_FLAG_FINAL, pdo.as_protocol_message(), None).encode_message().unwrap()[0];

    assert!(msg_compressed[4] != msg[4]); // msg flags is different
}


/*
   __  ____  _ __
  / / / / /_(_) /____
 / / / / __/ / / ___/
/ /_/ / /_/ / (__  )
\____/\__/_/_/____/
*/

fn get_request_session() -> RequestSession {
    let protocols: Vec<SupportedProtocol> = vec![
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

    let now: SystemTime = SystemTime::now();

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