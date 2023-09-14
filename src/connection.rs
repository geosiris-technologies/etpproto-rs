// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]

use etptypes::energistics::etp::v12::datatypes::server_capabilities::ServerCapabilities;

//use std::collections::HashMap;

use crate::credentials::ClientInfo;

enum CommunicationProtocol {
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

enum ConnectionType {
    Client = 0,
    Server = 1,
}

struct EtpConnection {
    pub client_info: Option<ClientInfo>,
    pub connection_type: ConnectionType,
    // pub message_cache: HashMap<i64, ETPMetadata>,
    // pub error_cache: HashMap<i64, ETPMetadata>,
    pub capabilities: Option<ServerCapabilities>,

    message_id: i64,
}

impl EtpConnection {
    fn consume_message_id(mut self) -> i64 {
        self.message_id = self.message_id + 1;
        self.message_id - 1
    }

    /*fn send(mut self, msg: Option<>, err: Option<>) -> Vec<8>{

    }*/
}
