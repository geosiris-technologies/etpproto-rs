// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::decode_message;
use crate::message::BytesEncodedMessage;
use crate::message::EtpMessageHandler;
use crate::message::Message;
use crate::message::MessageHeaderFlag;
use etptypes::energistics::etp::v12::datatypes::message_header::MessageHeader;
use etptypes::energistics::etp::v12::datatypes::server_capabilities::ServerCapabilities;
use etptypes::energistics::etp::v12::protocol::core::acknowledge::Acknowledge;
use etptypes::energistics::etp::v12::protocol::core::close_session::CloseSession;
use etptypes::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException;
use etptypes::error::*;
use etptypes::helpers::ETPMetadata;
use etptypes::protocols::ProtocolMessage;
use std::collections::HashMap;

//use std::collections::HashMap;

use crate::credentials::ClientInfo;

pub enum CommunicationProtocol {
    /*
    ETP Specification, Section 2 - ETP Published Protocols
    */
    // Creates and manages ETP sessions.
    Core = 0,

    // Provides "simple streamer" functionality, for example, a sensor streaming data.
    ChannelStreaming = 1,

    // Gets channel data from a store in "rows".
    ChannelDataFrame = 2,

    // Enables store customers to enumerate and understand the contents of a store of data objects.
    Discovery = 3,

    // Performs CRUD operations (create, retrieve, update and delete) on data objects in a store.
    Store = 4,

    // Allows store customers to receive notification of changes to data objects in the store in an event-driven manner, resulting from operations in Protocol 4.
    StoreNotification = 5,

    // Manages the growing parts of data objects that are index-based (i.e., time and depth) other than channels.
    GrowingObject = 6,

    // Allows a store customer to receive notifications of changes to the growing parts of growing data objects in a store, in an event-driven manner, resulting from operations in Protocol 6.
    GrowingObjectNotification = 7,

    // Transfers large, binary arrays of heterogeneous data values, which Energistics domain standards typically store using HDF5.
    DataArray = 9,

    // Query behavior appended to discovery functionality (which is defined in Protocol 3).
    DiscoveryQuery = 13,

    // Query behavior appended to store/CRUD functionality (which is defined in Protocol 4).
    StoreQuery = 14,

    // Query behavior appended to growing object behavior (which is defined in Protocol 6).
    GrowingObjectQuery = 16,

    // Handles messages associate with software application transactions, for example, end messages for applications that may have long, complex transactions (typically associated with earth modeling/RESQML).
    Transaction = 18,

    // Provides standard publish/subscribe behavior for customers to connect to a store (server) and receive new channel data as available (streaming).
    ChannelSubscribe = 21,

    // Enables one server (with the ETP customer role) to push (stream) data to another server (with the ETP store role).
    ChannelDataload = 22,

    // Used to discover dataspaces in a store. After discovering dataspaces, use Discovery (Protocol 3) to discover objects in the store.
    Dataspace = 24,

    // Enables store customers to discover a store's data model, to dynamically understand what object types are possible in the store at a given location in the data model (though the store may have no data for these object types), without prior knowledge of the overall data model and graph connectivity.
    SupportedTypes = 25,

    // In ETP v1.1, this protocol was published as Protocol 8. It is now a custom protocol published by an Energistics member company.
    WitsmlSoap = 2000,
}

pub enum ConnectionType {
    Client = 0,
    Server = 1,
}

pub struct EtpConnection {
    pub is_connected: bool,
    pub client_info: Option<ClientInfo>,
    pub connection_type: ConnectionType,
    pub message_cache: HashMap<i64, Vec<BytesEncodedMessage>>,
    // pub error_cache: HashMap<i64, Message<dyn ETPMetadata>>,
    pub capabilities: Option<ServerCapabilities>,

    msg_handler: Box<dyn EtpMessageHandler>,

    message_id: i64,
}

/* TODO definir un type EtpConnection a utilisé en dehors qui défini U toujours comme RequestSession,
l'appel au handler retournera une erreur lors d'un appel a un truc qui n'est pas génré (a priori)
*/
impl EtpConnection {
    pub fn new(
        client_info: Option<ClientInfo>,
        connection_type: ConnectionType,
        capabilities: Option<ServerCapabilities>,
        msg_handler: impl EtpMessageHandler + 'static,
    ) -> Self {
        let message_id = match connection_type {
            ConnectionType::Server => 1,
            ConnectionType::Client => 2,
        };
        Self {
            client_info,
            connection_type,
            capabilities,
            is_connected: false,
            message_id,
            message_cache: HashMap::new(),
            msg_handler: Box::new(msg_handler),
        }
    }

    fn consume_message_id(&mut self) -> i64 {
        self.message_id = self.message_id + 1;
        self.message_id - 1
    }

    /*fn send(mut self, msg: Option<>, err: Option<>) -> Vec<8>{

    }*/

    pub fn handle_encoded(&mut self, encoded: BytesEncodedMessage) -> Option<Vec<Message>> {
        let (mh, mb): (MessageHeader, Option<ProtocolMessage>) = decode_message(encoded);
        self.handle_message(&mh, &mb.unwrap())
    }

    pub fn handle_message(
        &mut self,
        mh: &MessageHeader,
        mb: &ProtocolMessage,
    ) -> Option<Vec<Message>> {
        let mut answer = vec![];
        let mh_flags = MessageHeaderFlag::parse(mh.message_flags);

        if mh_flags.msg_aknowledge {
            answer.push(Message::create_message(
                mh.message_id,
                self.consume_message_id(),
                MessageHeaderFlag::default().as_i32(),
                Acknowledge {}.as_protocol_message(),
                None,
            ))
        }

        if !self.is_connected {
            match self.connection_type {
                ConnectionType::Server => {
                    /*
                       _____                              ____             __
                      / ___/___  ______   _____  _____   / __ \____ ______/ /_
                      \__ \/ _ \/ ___/ | / / _ \/ ___/  / /_/ / __ `/ ___/ __/
                     ___/ /  __/ /   | |/ /  __/ /     / ____/ /_/ / /  / /_
                    /____/\___/_/    |___/\___/_/     /_/    \__,_/_/   \__/
                    */
                    if let ProtocolMessage::Core_RequestSession(pm_rq) = mb {
                        println!("{:?}", pm_rq.application_name);
                        if let Some(vvv) = self
                            .msg_handler
                            .handle(MessageHeaderFlag::parse(mh.message_flags), mb)
                        {
                            for handled in vvv {
                                if let ProtocolMessage::Core_OpenSession(_) = &handled {
                                    self.is_connected = true;
                                }
                                answer.push(Message::create_message(
                                    mh.message_id,
                                    self.consume_message_id(),
                                    MessageHeaderFlag::default().as_i32(),
                                    handled,
                                    None,
                                ))
                            }
                        }
                    } else {
                        answer.push(Message::create_message(
                            mh.message_id,
                            self.consume_message_id(),
                            MessageHeaderFlag::default().as_i32(),
                            ProtocolException::default_with_params(Some(erequest_denied()))
                                .as_protocol_message(),
                            None,
                        ));
                    }
                }
                ConnectionType::Client => {
                    /*
                       _________            __     ____             __
                      / ____/ (_)__  ____  / /_   / __ \____ ______/ /_
                     / /   / / / _ \/ __ \/ __/  / /_/ / __ `/ ___/ __/
                    / /___/ / /  __/ / / / /_   / ____/ /_/ / /  / /_
                    \____/_/_/\___/_/ /_/\__/  /_/    \__,_/_/   \__/
                    */
                    if let ProtocolMessage::Core_OpenSession(pm_os) = mb {
                        self.is_connected = true;
                        /*println!("{:?}", pm_os.application_name);*/
                        //self.capabilities = ServerCapabilities
                        if let Some(vvv) = self
                            .msg_handler
                            .handle(MessageHeaderFlag::parse(mh.message_flags), mb)
                        {
                            for handled in vvv {
                                if let ProtocolMessage::Core_OpenSession(_) = &handled {
                                    self.is_connected = true;
                                }
                                answer.push(Message::create_message(
                                    mh.message_id,
                                    self.consume_message_id(),
                                    MessageHeaderFlag::default().as_i32(),
                                    handled,
                                    None,
                                ))
                            }
                        }
                    } else {
                        /* Only check  for ping / pong */
                        /* match mb{
                            ProtocolMessage::Core_Ping | ProtocolMessage::Core_Pong => {

                            }
                        }*/
                    }
                }
            }
        } else {
            /*if self.is_connected || mb.protocol() == (CommunicationProtocol::Core as i32){*/

            if let ProtocolMessage::Core_CloseSession(_) = &mb {
                self.is_connected = false;
                self.capabilities = None;
                match self.connection_type {
                    ConnectionType::Server => {
                        answer.push(Message::create_message(
                            mh.message_id,
                            self.consume_message_id(),
                            MessageHeaderFlag::default().as_i32(),
                            CloseSession {
                                reason: "Answer to client CloseSession message".to_string(),
                            }
                            .as_protocol_message(),
                            None,
                        ));
                    }
                    _ => {}
                };
            } else {
                if let Some(vvv) = self
                    .msg_handler
                    .handle(MessageHeaderFlag::parse(mh.message_flags), mb)
                {
                    for handled in vvv {
                        if let ProtocolMessage::Core_OpenSession(_) = &handled {
                            self.is_connected = true;
                        }
                        answer.push(Message::create_message(
                            mh.message_id,
                            self.consume_message_id(),
                            MessageHeaderFlag::default().as_i32(),
                            handled,
                            None,
                        ))
                    }
                    /* TODO : for each answer, check if an error contains one that requires to close the session (e.g.  EAUTHORIZATION_EXPIRED)  if yes , add a closeSession message*/
                }
            }
        }
        Some(answer)
    }
}
