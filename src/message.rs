// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
//#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use etptypes::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException;
use etptypes::error::eunsupported_protocol;
use etptypes::helpers::AvroDeserializable;
use etptypes::helpers::AvroSerializable;
use etptypes::protocols::{avro_decode, ProtocolMessage};

use etptypes::energistics::etp::v12::datatypes::message_header::MessageHeader;
use etptypes::energistics::etp::v12::datatypes::message_header_extension::MessageHeaderExtension;
use etptypes::helpers::ETPMetadata;

pub type BytesEncodedMessage = Vec<u8>;

fn get_type_name<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub const MSG_FLAG_NONE: i32 = 0x00;
pub const MSG_FLAG_MULTIPART: i32 = 0x01;
pub const MSG_FLAG_FINAL: i32 = 0x02;
pub const MSG_FLAG_MULTIPART_AND_FINALPART: i32 = MSG_FLAG_FINAL | MSG_FLAG_MULTIPART;
pub const MSG_FLAG_NO_DATA: i32 = 0x04;
pub const MSG_FLAG_COMPRESSED: i32 = 0x08;
pub const MSG_FLAG_ACKNOWLEDGE: i32 = 0x10;
pub const MSG_FLAG_HEADER_EXTENSION: i32 = 0x20;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct MessageHeaderFlag {
    pub msg_final: bool,
    pub msg_multipart: bool,
    pub msg_compressed: bool,
    pub msg_no_data: bool,
    pub msg_aknowledge: bool,
    pub msg_has_header_extension: bool,
}

impl Default for MessageHeaderFlag {
    /* Protocol 0, MessageType : 2 */
    fn default() -> MessageHeaderFlag {
        MessageHeaderFlag {
            msg_final: false,
            msg_multipart: false,
            msg_compressed: false,
            msg_no_data: false,
            msg_aknowledge: false,
            msg_has_header_extension: false,
        }
    }
}

impl MessageHeaderFlag {
    pub fn as_i32(&self) -> i32 {
        (if self.msg_final { MSG_FLAG_FINAL } else { 0 })
            | (if self.msg_compressed {
                MSG_FLAG_COMPRESSED
            } else {
                0
            })
            | (if self.msg_multipart {
                MSG_FLAG_MULTIPART
            } else {
                0
            })
            | (if self.msg_no_data {
                MSG_FLAG_NO_DATA
            } else {
                0
            })
            | (if self.msg_aknowledge {
                MSG_FLAG_ACKNOWLEDGE
            } else {
                0
            })
            | (if self.msg_has_header_extension {
                MSG_FLAG_HEADER_EXTENSION
            } else {
                0
            })
    }

    pub fn parse(flag: i32) -> MessageHeaderFlag {
        MessageHeaderFlag {
            msg_final: (flag & MSG_FLAG_FINAL) != 0,
            msg_multipart: (flag & MSG_FLAG_MULTIPART) != 0,
            msg_no_data: (flag & MSG_FLAG_NO_DATA) != 0,
            msg_compressed: (flag & MSG_FLAG_COMPRESSED) != 0,
            msg_aknowledge: (flag & MSG_FLAG_ACKNOWLEDGE) != 0,
            msg_has_header_extension: (flag & MSG_FLAG_HEADER_EXTENSION) != 0,
        }
    }
}

pub trait EtpMessageHandler {
    fn handle(
        &mut self,
        header: MessageHeaderFlag,
        msg: &ProtocolMessage,
    ) -> Option<Vec<ProtocolMessage>> {
        Some(vec![ProtocolMessage::Core_ProtocolException(
            ProtocolException::default_with_params(Some(eunsupported_protocol())),
        )])
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub header: MessageHeader,
    pub header_extension: Option<MessageHeaderExtension>,
    pub body: ProtocolMessage,
}

impl Message {
    pub fn create_message(
        correlation_id: i64,
        message_id: i64,
        message_flags: i32,
        body: ProtocolMessage,
        header_extension: Option<MessageHeaderExtension>,
    ) -> Message {
        Message {
            header: MessageHeader {
                protocol: body.protocol(),
                message_type: body.message_type(),
                correlation_id,
                message_id,
                message_flags,
            },
            header_extension,
            body,
        }
    }

    pub fn encode_message(&self) -> Option<Vec<BytesEncodedMessage>> {
        let correlation_id = if self.header.correlation_id != 0 {
            self.header.correlation_id
        } else {
            self.header.message_id
        };

        let message_type = get_type_name(&self.body);
        let is_a_request = message_type.ends_with("Response");

        let encoded_header = self.header.avro_serialize().unwrap();
        let encoded_body = self.body.avro_serialize().unwrap();
        println!("Encoded header size : {}", &encoded_header.len());

        let first_encoded = vec![encoded_header, encoded_body].concat();

        Some(vec![first_encoded])
    }
}

pub fn decode_message(encoded: BytesEncodedMessage) -> (MessageHeader, Option<ProtocolMessage>) {
    let mut encoded_slice = &encoded[0..5];
    let mut encoded_mb = &encoded[5..];
    let mh = MessageHeader::avro_deserialize(&mut encoded_slice).unwrap();
    let mb = avro_decode(&mh, &mut encoded_mb);
    println!("{:?}", mh);
    println!("{:?}", mb);
    (mh, mb)
}
