// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(dead_code)]

use std::collections::HashMap;
use std::str::FromStr;
use etptypes::energistics::etp::v12::datatypes::data_value::DataValue;
use etptypes::energistics::etp::v12::datatypes::data_value::UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray as U_TYPE;
use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;

pub fn negotiate_capabilities(
    ca: HashMap<String, DataValue>,
    cb: HashMap<String, DataValue>,
) -> HashMap<String, DataValue> {
    let mut nego: HashMap<String, DataValue> = HashMap::new();

    let mut caps_kinds_names: Vec<String> = vec![];
    for cap_kind in EndpointCapabilityKind::iter(){
        caps_kinds_names.push( format!("{}", cap_kind));
    }

    for (ca_key, ca_val) in &ca {
        if caps_kinds_names.iter().any(|e| ca_key.contains(e)) {
            for (cb_key, cb_val) in &cb {
                if ca_key == cb_key {
                    let eck = EndpointCapabilityKind::from_str(ca_key).unwrap();
                    let min_opt = eck.min();
                    let max_opt = eck.max();

                    match (ca_val.item.as_ref().unwrap(), cb_val.item.as_ref().unwrap(), ){
                        (U_TYPE::Long(long_val_a), U_TYPE::Long(long_val_b)) => {
                            let mut value = *std::cmp::min(long_val_a, long_val_b);
                            if let Some(DataValue { item: dv }) = min_opt {
                                if let Some(U_TYPE::Long(min_value)) = dv {
                                    if  min_value > value{
                                        value = min_value;
                                    }
                                }
                            }
                            if let Some(DataValue { item: dv }) = max_opt {
                                if let Some(U_TYPE::Long(max_value)) = dv {
                                    if  max_value < value{
                                        value = max_value;
                                    }
                                }
                            }
                            nego.insert(format!("{}", eck), DataValue{item: Some(U_TYPE::Long(value))});
                        },
                        (U_TYPE::Boolean(bool_val_a), U_TYPE::Boolean(bool_val_b)) => {
                            nego.insert(format!("{}", eck), DataValue{item: Some(U_TYPE::Boolean(*bool_val_a && *bool_val_b))});
                        }
                        _ => {
                            println!("\tNego {:?}", ca_val.item.as_ref().unwrap());
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
    for (cb_key, cb_val) in &ca {
        if !caps_kinds_names.iter().any(|e| cb_key.contains(e)) {
            nego.insert(cb_key.to_string(), cb_val.clone());
        }
    }

    // Adding every properties that has a default value
    for not_found in caps_kinds_names.into_iter().filter(|ckn| !nego.contains_key(ckn)).collect::<Vec<String>>(){
        match EndpointCapabilityKind::from_str(&not_found).unwrap().default(){
            Some(v) => {
                nego.insert(not_found, v);
            },
            _ => {}
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
