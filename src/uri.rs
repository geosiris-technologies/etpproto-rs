// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
use regex::Regex;
use std::fmt;

#[inline(always)]
pub fn canonical_dataspace_uris_regex() -> String {
    r"^eml:\/\/\/(?:dataspace\('?(?<dataspace>[^']*?(?:''[^']*?)*)'?\))?$".to_string()
}

#[inline(always)]
pub fn uuid_regex() -> String {
    r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}".to_string()
}

#[inline(always)]
pub fn domain_names_regex() -> String {
    r"witsml|resqml|prodml|eml".to_string()
}

/*
"^eml:\\/\\/\\/(?:dataspace\\('(?<dataspace>[^']*?(?:''[^']*?)*)'\\)\\/)?(?<domain>witsml|resqml|prodml|eml)(?<domainVersion>[1-9]\\d+)\\.(?<objectType>\\w+)\\((?:(uuid=)?(?<uuid>[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}),version='(?<version>[^']*?(?:''[^']*?)*)')\\)$"
*/
#[inline(always)]
pub fn canonical_data_object_uris() -> String {
    r"^eml:///(?:dataspace\('(?P<dataspace>[^']*?(?:''[^']*?)*)'\)/?)?(?:(?P<object>(?P<domain>witsml|resqml|prodml|eml)(?P<domainVersion>[1-9]\d)\.(?P<objectType>\w+)\((?:(?P<uuid>[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})|uuid=(?P<uuid2>[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}),version='(?P<version>[^']*?(?:''[^']*?)*)')\))?(?P<request>(?:(?:(?:(/(?P<collectionDomain>witsml|resqml|prodml|eml)(?P<collectionDomainVersion>[1-9]\d)\.(?P<collectionType>\w+))?)|(?P<subPath>(/[\w]+)+))?(?:\?(?P<query>[^#\n]+))?))?)?$".to_string()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Uri{
	pub raw: String,
	pub dataspace: Option<String>,
	pub domain: Option<String>,
	pub domain_version: Option<String>,
	pub object_type: Option<String>,
	pub object_uuid: Option<String>,
	pub object_version: Option<String>,
	pub collection_domain: Option<String>,
	pub collection_domain_version: Option<String>,
	pub collection_type: Option<String>,
	pub sub_path: Option<String>,
	pub query: Option<String>,
}

impl Uri{
	pub fn parse(uri: &str) -> Uri{
		let re = Regex::new(&canonical_data_object_uris()).unwrap();
		let caps_uw = re.captures(uri);
		match caps_uw{
			Some(caps) => 
				Uri{
					raw: uri.to_string(),
					dataspace: match caps.name("dataspace"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					domain: match caps.name("domain"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					domain_version: match caps.name("domainVersion"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					object_type: match caps.name("objectType"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					object_uuid: match caps.name("uuid"){
						Some(w) => Some(w.as_str().to_string()),
						None => match caps.name("uuid2"){
							Some(w2) => Some(w2.as_str().to_string()),
							None => None
						}
					},
					object_version: match caps.name("version"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					collection_domain: match caps.name("collectionDomain"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					collection_domain_version: match caps.name("collectionDomainVersion"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					collection_type: match caps.name("collectionType"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					sub_path: match caps.name("subPath"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
					query: match caps.name("query"){
						Some(w) => Some(w.as_str().to_string()),
						None => None
					},
				},
			None => 
			Uri{
				raw: uri.to_string(),
				dataspace: None,
				domain: None,
				domain_version: None,
				object_type: None,
				object_uuid: None,
				object_version: None,
				collection_domain: None,
				collection_domain_version: None,
				collection_type: None,
				sub_path: None,
				query: None,
			}
		}
		
	}
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

pub fn find_uuid(input: &str) -> &str {
	let re = Regex::new(&uuid_regex()).unwrap();
	re.find(input).unwrap().as_str()
}
