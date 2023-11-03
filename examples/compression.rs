// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_variables)]
#![allow(dead_code)]

use etptypes::energistics::etp::v12::protocol::store::put_data_objects::PutDataObjects;
use etptypes::energistics::etp::v12::datatypes::object::data_object::DataObject;
use etptypes::energistics::etp::v12::datatypes::object::resource::Resource;
use etptypes::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind;
use etpproto::message::decode_message;
use etpproto::message::EtpMessage;
use etpproto::message::MSG_FLAG_COMPRESSED;
use etpproto::message::MSG_FLAG_FINAL;

///////////////

use etptypes::energistics::etp::v12::datatypes::protocol::Protocol;
use etptypes::energistics::etp::v12::datatypes::supported_protocol::SupportedProtocol;
use etptypes::energistics::etp::v12::datatypes::uuid::random_uuid;
use etptypes::energistics::etp::v12::protocol::core::request_session::RequestSession;
use etptypes::helpers::ETP12VERSION;
use etptypes::helpers::time_to_etp;
use std::collections::HashMap;
use std::time::SystemTime;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let req_sess = get_request_session();
    
    let msg_compressed_req_sess = EtpMessage::create_message(0,1, MSG_FLAG_FINAL|MSG_FLAG_COMPRESSED, req_sess.as_protocol_message(), None).encode_message().unwrap();
    let msg_req_sess = EtpMessage::create_message(0, 1, MSG_FLAG_FINAL, req_sess.as_protocol_message(), None).encode_message().unwrap();

    println!("Not compressed : {:?}", msg_req_sess);
    println!("Compressed : {:?}", msg_compressed_req_sess);
    println!("Compressed and not are the same ? {:?}", msg_compressed_req_sess.iter().eq(msg_req_sess.iter()));
    println!("Not compressed : {:?}", decode_message(&msg_req_sess[0]));
    println!("Not compressed : {:?}", decode_message(&msg_compressed_req_sess[0]));

    let pdo = get_put_data_object();
    
    let msg_compressed_pdo = EtpMessage::create_message(0,1, MSG_FLAG_FINAL|MSG_FLAG_COMPRESSED, pdo.as_protocol_message(), None).encode_message().unwrap();
    let msg_pdo = EtpMessage::create_message(0, 1, MSG_FLAG_FINAL, pdo.as_protocol_message(), None).encode_message().unwrap();

    println!("Not compressed Size : {:?}", msg_pdo[0].len());
    println!("Compressed Size : {:?}", msg_compressed_pdo[0].len());
    //println!("Not compressed : {:?}", msg_pdo);
    //println!("Compressed : {:?}", msg_compressed_pdo);
    println!("Compressed and not are the same ? {:?}", msg_compressed_pdo.iter().eq(msg_pdo.iter()));
    /*println!("Not compressed : {:?}", decode_message(&msg_pdo[0]));
    println!("Not compressed : {:?}", decode_message(&msg_compressed_pdo[0]));*/
}


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


fn get_put_data_object()-> PutDataObjects {
    PutDataObjects{
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
    }
}