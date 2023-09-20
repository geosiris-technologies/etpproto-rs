// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]

use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;
use etptypes::energistics::etp::v12::datatypes::server_capabilities::ServerCapabilities;
use etptypes::energistics::etp::v12::datatypes::supported_data_object::SupportedDataObject;
use etptypes::energistics::etp::v12::datatypes::supported_protocol::SupportedProtocol;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::slice::Iter;
use std::str::FromStr;

/*
    _   __                 __  _       __  _
   / | / /__  ____ _____  / /_(_)___ _/ /_(_)___  ____
  /  |/ / _ \/ __ `/ __ \/ __/ / __ `/ __/ / __ \/ __ \
 / /|  /  __/ /_/ / /_/ / /_/ / /_/ / /_/ / /_/ / / / /
/_/ |_/\___/\__, /\____/\__/_/\__,_/\__/_/\____/_/ /_/
           /____/
*/

pub fn negotiate_endpoint_capabilities(
    ca: &HashMap<String, DataValue>,
    cb: &HashMap<String, DataValue>,
) -> HashMap<String, DataValue> {
    let mut nego: HashMap<String, DataValue> = HashMap::new();

    let mut caps_kinds_names: Vec<String> = vec![];
    for cap_kind in EndpointCapabilityKind::iter() {
        caps_kinds_names.push(format!("{}", cap_kind));
    }

    for (ca_key, ca_val) in ca {
        if caps_kinds_names.iter().any(|e| ca_key.contains(e)) {
            for (cb_key, cb_val) in cb {
                if ca_key == cb_key {
                    let eck = EndpointCapabilityKind::from_str(ca_key).unwrap();
                    let min_opt = eck.min();
                    let max_opt = eck.max();

                    let mut long_val_a: Option<i64> = None;
                    let mut long_val_b: Option<i64> = None;
                    let mut bool_val_a: Option<bool> = None;
                    let mut bool_val_b: Option<bool> = None;

                    match ca_val.item.as_ref().unwrap() {
                        U_TYPE::Long(v) => long_val_a = Some(*v),
                        U_TYPE::Int(v) => long_val_a = Some(*v as i64),
                        U_TYPE::Float(v) => long_val_a = Some(*v as i64),
                        U_TYPE::Double(v) => long_val_a = Some(*v as i64),
                        U_TYPE::String(v) => match v.parse::<i64>() {
                            Ok(v_long) => long_val_a = Some(v_long),
                            Err(_) => {
                                println!(
                                    "#Negotiation : {} string value not parsable to long : {}",
                                    ca_key, v
                                )
                            }
                        },
                        U_TYPE::Boolean(v) => bool_val_a = Some(*v),
                        _ => {}
                    }
                    match cb_val.item.as_ref().unwrap() {
                        U_TYPE::Long(v) => long_val_b = Some(*v),
                        U_TYPE::Int(v) => long_val_b = Some(*v as i64),
                        U_TYPE::Float(v) => long_val_b = Some(*v as i64),
                        U_TYPE::Double(v) => long_val_b = Some(*v as i64),
                        U_TYPE::String(v) => match v.parse::<i64>() {
                            Ok(v_long) => long_val_b = Some(v_long),
                            Err(_) => {
                                println!(
                                    "#Negotiation : {} string value not parsable to long : {}",
                                    ca_key, v
                                )
                            }
                        },
                        U_TYPE::Boolean(v) => bool_val_b = Some(*v),
                        _ => {}
                    }

                    if let Some(v_a) = long_val_a {
                        if let Some(v_b) = long_val_b {
                            let mut value = std::cmp::min(v_a, v_b);
                            if let Some(DataValue { item: ref dv }) = min_opt {
                                if let Some(U_TYPE::Long(min_value)) = dv {
                                    if min_value > &value {
                                        value = *min_value;
                                    }
                                }
                            }
                            if let Some(DataValue { item: ref dv }) = max_opt {
                                if let Some(U_TYPE::Long(max_value)) = dv {
                                    if max_value < &value {
                                        value = *max_value;
                                    }
                                }
                            }
                            nego.insert(
                                format!("{}", eck),
                                DataValue {
                                    item: Some(U_TYPE::Long(value)),
                                },
                            );
                        }
                    }

                    if let Some(v_a) = bool_val_a {
                        if let Some(v_b) = bool_val_b {
                            nego.insert(
                                format!("{}", eck),
                                DataValue {
                                    item: Some(U_TYPE::Boolean(v_a && v_b)),
                                },
                            );
                        }
                    }
                }
            }
        } else {
            println!("Not in EndpointCapabilityKind {:?}", ca_key);
            nego.insert(ca_key.to_string(), ca_val.clone());
        }
    }

    // TODO: here "cb" may overrite "ca" value if contains a property not from EndpointCapabilityKind with same str value as in "ca"
    for (cb_key, cb_val) in ca {
        if !caps_kinds_names.iter().any(|e| cb_key.contains(e)) {
            nego.insert(cb_key.to_string(), cb_val.clone());
        }
    }

    // Adding every properties that has a default value
    for not_found in caps_kinds_names
        .into_iter()
        .filter(|ckn| !nego.contains_key(ckn))
        .collect::<Vec<String>>()
    {
        match EndpointCapabilityKind::from_str(&not_found)
            .unwrap()
            .default()
        {
            Some(v) => {
                nego.insert(not_found, v);
            }
            _ => {}
        }
    }

    nego
}

pub fn negotiate_supported_data_object(
    cap_me: &Vec<SupportedDataObject>,
    cap_target: &Vec<SupportedDataObject>,
) -> Vec<SupportedDataObject> {
    let mut nego = vec![];

    for do_me in cap_me {
        for do_target in cap_target {
            if do_me.qualified_type == do_target.qualified_type {
                let mut doc_new: HashMap<String, DataValue> = HashMap::new();

                for dok in DataObjectCapabilities::iter() {
                    let mut dok_me: Option<bool> = None;
                    let mut dok_target: Option<bool> = None;
                    let dok_str = format!("{}", dok);

                    if do_me.data_object_capabilities.contains_key(&dok_str) {
                        if let Some(DataValue {
                            item: Some(U_TYPE::Boolean(bool_value)),
                        }) = do_me.data_object_capabilities.get(&dok_str)
                        {
                            dok_me = Some(*bool_value);
                        }
                    }
                    if do_target.data_object_capabilities.contains_key(&dok_str) {
                        if let Some(DataValue {
                            item: Some(U_TYPE::Boolean(bool_value)),
                        }) = do_target.data_object_capabilities.get(&dok_str)
                        {
                            dok_target = Some(*bool_value);
                        }
                    }

                    match (dok_me, dok_target) {
                        (Some(true), Some(true)) => {
                            doc_new.insert(
                                dok_str,
                                DataValue {
                                    item: Some(U_TYPE::Boolean(true)),
                                },
                            );
                        }
                        (Some(false), Some(false))
                        | (Some(false), Some(true))
                        | (Some(true), Some(false)) => {
                            doc_new.insert(
                                dok_str,
                                DataValue {
                                    item: Some(U_TYPE::Boolean(false)),
                                },
                            );
                        }
                        _ => {}
                    }
                }
                nego.push(SupportedDataObject {
                    qualified_type: do_me.qualified_type.clone(),
                    data_object_capabilities: doc_new,
                });
            }
        }
    }

    nego
}

pub fn negotiate_supported_protocol(
    cap_me: &Vec<SupportedProtocol>,
    cap_target: &Vec<SupportedProtocol>,
) -> Vec<SupportedProtocol> {
    let nego = vec![];

    /*for do_me in cap_me{
        for do_target in cap_target{
            if do_me.protocol == do_target.protocol
            && do_me.protocol_version == do_target.protocol_version
            && {
            }
        }
    }*/
    nego
}

pub fn negotiate_server_capabilities(
    cap_me: &ServerCapabilities,
    cap_target: &ServerCapabilities,
) -> ServerCapabilities {
    /* We keep target String values to save them because we know ours */
    ServerCapabilities {
        application_name: format!("{:?}", cap_target.application_name),
        application_version: format!("{:?}", cap_target.application_version),
        contact_information: cap_target.contact_information.clone(),
        supported_compression: {
            let set_me: HashSet<_> = cap_me.supported_compression.clone().into_iter().collect();
            let set_target: HashSet<_> = cap_target
                .supported_compression
                .clone()
                .into_iter()
                .collect();
            set_me
                .intersection(&set_target)
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
        },
        supported_encodings: {
            let set_me: HashSet<_> = cap_me.supported_encodings.clone().into_iter().collect();
            let set_target: HashSet<_> =
                cap_target.supported_encodings.clone().into_iter().collect();
            set_me
                .intersection(&set_target)
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
        },
        supported_formats: {
            let set_me: HashSet<_> = cap_me.supported_formats.clone().into_iter().collect();
            let set_target: HashSet<_> = cap_target.supported_formats.clone().into_iter().collect();
            set_me
                .intersection(&set_target)
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
        },
        supported_data_objects: negotiate_supported_data_object(
            &cap_me.supported_data_objects,
            &cap_target.supported_data_objects,
        ),
        supported_protocols: negotiate_supported_protocol(
            &cap_me.supported_protocols,
            &cap_target.supported_protocols,
        ),
        endpoint_capabilities: negotiate_endpoint_capabilities(
            &cap_me.endpoint_capabilities,
            &cap_target.endpoint_capabilities,
        ),
    }
}

/*
    ____        __        ____  __      _           __  ______                  __    _ ___ __  _
   / __ \____ _/ /_____ _/ __ \/ /_    (_)__  _____/ /_/ ____/___ _____  ____ _/ /_  (_) (_) /_(_)__  _____
  / / / / __ `/ __/ __ `/ / / / __ \  / / _ \/ ___/ __/ /   / __ `/ __ \/ __ `/ __ \/ / / / __/ / _ \/ ___/
 / /_/ / /_/ / /_/ /_/ / /_/ / /_/ / / /  __/ /__/ /_/ /___/ /_/ / /_/ / /_/ / /_/ / / / / /_/ /  __(__  )
/_____/\__,_/\__/\__,_/\____/_.___/_/ /\___/\___/\__/\____/\__,_/ .___/\__,_/_.___/_/_/_/\__/_/\___/____/
                                 /___/                         /_/
*/

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum DataObjectCapabilities {
    SupportsGet,
    SupportsPut,
    SupportsDelete,
}

impl fmt::Display for DataObjectCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataObjectCapabilities::SupportsGet => "SupportsGet",
                DataObjectCapabilities::SupportsPut => "SupportsPut",
                DataObjectCapabilities::SupportsDelete => "SupportsDelete",
            }
        )
    }
}
impl FromStr for DataObjectCapabilities {
    type Err = ();
    fn from_str(input: &str) -> Result<DataObjectCapabilities, Self::Err> {
        match input {
            "SupportsGet" => Ok(DataObjectCapabilities::SupportsGet),
            "SupportsPut" => Ok(DataObjectCapabilities::SupportsPut),
            "SupportsDelete" => Ok(DataObjectCapabilities::SupportsDelete),
            _ => Err(()),
        }
    }
}

impl DataObjectCapabilities {
    pub fn iter() -> Iter<'static, DataObjectCapabilities> {
        static VEC_ENUM: [DataObjectCapabilities; 3] = [
            DataObjectCapabilities::SupportsGet,
            DataObjectCapabilities::SupportsPut,
            DataObjectCapabilities::SupportsDelete,
        ];
        VEC_ENUM.iter()
    }
}

/*
    ______          __            _       __  ______                  __    _ ___ __        __ __ _           __
   / ____/___  ____/ /___  ____  (_)___  / /_/ ____/___ _____  ____ _/ /_  (_) (_) /___  __/ //_/(_)___  ____/ /
  / __/ / __ \/ __  / __ \/ __ \/ / __ \/ __/ /   / __ `/ __ \/ __ `/ __ \/ / / / __/ / / / ,<  / / __ \/ __  /
 / /___/ / / / /_/ / /_/ / /_/ / / / / / /_/ /___/ /_/ / /_/ / /_/ / /_/ / / / / /_/ /_/ / /| |/ / / / / /_/ /
/_____/_/ /_/\__,_/ .___/\____/_/_/ /_/\__/\____/\__,_/ .___/\__,_/_.___/_/_/_/\__/\__, /_/ |_/_/_/ /_/\__,_/
                 /_/                                 /_/                          /____/
*/

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
            EndpointCapabilityKind::ActiveTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(3600)),
            }),
            // EndpointCapabilityKind::AuthorizationDetails => Some(DataValue{item: Some(U_TYPE::Long(1))}), // ArrayOfString
            EndpointCapabilityKind::ChangePropagationPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(5)),
            }),
            EndpointCapabilityKind::ChangeRetentionPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(86400)),
            }),
            EndpointCapabilityKind::MaxConcurrentMultipart => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::ResponseTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(300)),
            }),
            EndpointCapabilityKind::RequestSessionTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(45)),
            }),
            EndpointCapabilityKind::SessionEstablishmentTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(3600)),
            }),
            EndpointCapabilityKind::SupportsAlternateRequestUris => Some(DataValue {
                item: Some(U_TYPE::Boolean(false)),
            }),
            EndpointCapabilityKind::SupportsMessageHeaderExtensions => Some(DataValue {
                item: Some(U_TYPE::Boolean(false)),
            }),
            _ => None,
        }
    }
    fn min(&self) -> Option<DataValue> {
        match self {
            EndpointCapabilityKind::ActiveTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            EndpointCapabilityKind::ChangePropagationPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::ChangeRetentionPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(86400)),
            }),
            EndpointCapabilityKind::MaxConcurrentMultipart => Some(DataValue {
                item: Some(U_TYPE::Long(1)),
            }),
            EndpointCapabilityKind::MaxDataObjectSize => Some(DataValue {
                item: Some(U_TYPE::Long(100000)),
            }),
            EndpointCapabilityKind::MaxPartSize => Some(DataValue {
                item: Some(U_TYPE::Long(10000)),
            }),
            EndpointCapabilityKind::MaxSessionClientCount => Some(DataValue {
                item: Some(U_TYPE::Long(2)),
            }),
            EndpointCapabilityKind::MaxSessionGlobalCount => Some(DataValue {
                item: Some(U_TYPE::Long(2)),
            }),
            EndpointCapabilityKind::ResponseTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            EndpointCapabilityKind::RequestSessionTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(5)),
            }),
            EndpointCapabilityKind::SessionEstablishmentTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            _ => None,
        }
    }
    fn max(&self) -> Option<DataValue> {
        match self {
            EndpointCapabilityKind::ChangePropagationPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(600)),
            }),
            EndpointCapabilityKind::MultipartMessageTimeoutPeriod => Some(DataValue {
                item: Some(U_TYPE::Long(60)),
            }),
            _ => None,
        }
    }
}
