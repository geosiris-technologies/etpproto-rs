// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT

use etpproto::uri::*;

#[test]
fn test_uri_dataspace_0() {
    let uri = Uri::parse("eml:///dataspace('/folder-name/project-name')").unwrap();
    assert_eq!(
        uri.dataspace.unwrap(),
        "/folder-name/project-name".to_string()
    );
}

#[test]
fn test_uri_dataspace_1() {
    let uri = Uri::parse("eml:///dataspace('alwyn')").unwrap();
    assert_eq!(uri.dataspace.unwrap(), "alwyn".to_string());
}

#[test]
fn test_uri_dataspace_2() {
    let uri = Uri::parse("eml:///").unwrap();
    assert!(uri.dataspace.is_none());
}

#[test]
fn test_uri_data_object_0() {
    let uri = Uri::parse("eml:///witsml20.ChannelSet(2c0f6ef2-cc54-4104-8523-0f0fbaba3661)").unwrap();
    assert!(uri.dataspace.is_none());
    assert_eq!(uri.domain.unwrap(), "witsml");
    assert_eq!(uri.domain_version.unwrap(), "20");
    assert_eq!(uri.object_type.unwrap(), "ChannelSet");
    assert_eq!(
        uri.object_uuid.unwrap(),
        "2c0f6ef2-cc54-4104-8523-0f0fbaba3661"
    );
    assert!(uri.object_version.is_none());
}

#[test]
fn test_uri_data_object_1() {
    let uri = Uri::parse(
        "eml:///dataspace('alwyn')/witsml20.ChannelSet(2c0f6ef2-cc54-4104-8523-0f0fbaba3661)",
    ).unwrap();
    assert_eq!(uri.dataspace.unwrap(), "alwyn".to_string());
    assert_eq!(uri.domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.object_type.unwrap(), "ChannelSet".to_string());
    assert_eq!(
        uri.object_uuid.unwrap(),
        "2c0f6ef2-cc54-4104-8523-0f0fbaba3661".to_string()
    );
    assert!(uri.object_version.is_none());
}

#[test]
fn test_uri_data_object_2() {
    let uri = Uri::parse(
        "eml:///dataspace('rdms-db')/resqml20.obj_HorizonInterpretation(uuid=421a7a05-033a-450d-bcef-051352023578,version='2.0')"
    ).unwrap();
    assert_eq!(uri.dataspace.unwrap(), "rdms-db".to_string());
    assert_eq!(uri.domain.unwrap(), "resqml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(
        uri.object_type.unwrap(),
        "obj_HorizonInterpretation".to_string()
    );
    assert_eq!(
        uri.object_uuid.unwrap(),
        "421a7a05-033a-450d-bcef-051352023578".to_string()
    );
    assert_eq!(uri.object_version.unwrap(), "2.0".to_string());
}

#[test]
fn test_raw_uri() {
    let uri = Uri::parse("eml:///").unwrap();
    assert_eq!(uri.raw, "eml:///");
}

#[test]
fn test_find_uuid() {
    assert_eq!(find_uuid(
            "eml:///dataspace('rdms-db')/resqml20.obj_HorizonInterpretation(uuid=421a7a05-033a-450d-bcef-051352023578,version='2.0')"
        ), "421a7a05-033a-450d-bcef-051352023578"
    );
    assert_eq!(
        find_uuid("my test TExt421a7a05-033a-450d-bcef-051352023578aa)"),
        "421a7a05-033a-450d-bcef-051352023578"
    );
}

#[test]
fn test_complex_uri_0() {
    let uri = Uri::parse("eml:///dataspace('/folder-name/project-name')/witsml20.Well(uuid=ec8c3f16-1454-4f36-ae10-27d2a2680cf2,version='1.0')/witsml20.Wellbore?query").unwrap();
    assert_eq!(
        uri.dataspace.unwrap(),
        "/folder-name/project-name".to_string()
    );
    assert_eq!(uri.domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.object_type.unwrap(), "Well".to_string());
    assert_eq!(
        uri.object_uuid.unwrap(),
        "ec8c3f16-1454-4f36-ae10-27d2a2680cf2".to_string()
    );
    assert_eq!(uri.object_version.unwrap(), "1.0".to_string());
    assert_eq!(uri.collection_domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.collection_domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.collection_type.unwrap(), "Wellbore".to_string());
    assert_eq!(uri.query.unwrap(), "query".to_string());
}

#[test]
fn test_complex_uri_1() {
    let uri = Uri::parse("eml:///witsml20.Well(uuid=ec8c3f16-1454-4f36-ae10-27d2a2680cf2,version='1.0')/witsml20.Wellbore?query").unwrap();
    assert!(uri.dataspace.is_none());
    assert_eq!(uri.domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.object_type.unwrap(), "Well".to_string());
    assert_eq!(
        uri.object_uuid.unwrap(),
        "ec8c3f16-1454-4f36-ae10-27d2a2680cf2".to_string()
    );
    assert_eq!(uri.object_version.unwrap(), "1.0".to_string());
    assert_eq!(uri.collection_domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.collection_domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.collection_type.unwrap(), "Wellbore".to_string());
    assert_eq!(uri.query.unwrap(), "query".to_string());
}

#[test]
fn test_complex_uri_2() {
    let uri = Uri::parse("eml:///dataspace('/folder-name/projectname')/resqml20.obj_HorizonInterpretation(uuid=421a7a05-033a-450d-bcef-051352023578,version='2.0')?query").unwrap();
    assert_eq!(
        uri.dataspace.unwrap(),
        "/folder-name/projectname".to_string()
    );
    assert_eq!(uri.domain.unwrap(), "resqml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(
        uri.object_type.unwrap(),
        "obj_HorizonInterpretation".to_string()
    );
    assert_eq!(
        uri.object_uuid.unwrap(),
        "421a7a05-033a-450d-bcef-051352023578".to_string()
    );
    assert_eq!(uri.object_version.unwrap(), "2.0".to_string());
    assert!(uri.collection_domain.is_none());
    assert!(uri.collection_domain_version.is_none());
    assert!(uri.collection_type.is_none());
    assert_eq!(uri.query.unwrap(), "query".to_string());
}

#[test]
fn test_complex_uri_3() {
    let uri = Uri::parse(
        "eml:///witsml20.Channel(53b3bf2b-3aa3-458d-b40c-9a4cb754210e)/ChannelClass/Title",
    ).unwrap();
    assert!(uri.dataspace.is_none());
    assert_eq!(uri.domain.unwrap(), "witsml".to_string());
    assert_eq!(uri.domain_version.unwrap(), "20".to_string());
    assert_eq!(uri.object_type.unwrap(), "Channel".to_string());
    assert_eq!(
        uri.object_uuid.unwrap(),
        "53b3bf2b-3aa3-458d-b40c-9a4cb754210e".to_string()
    );
    assert!(uri.object_version.is_none());
    assert!(uri.collection_domain.is_none());
    assert!(uri.collection_domain_version.is_none());
    assert!(uri.collection_type.is_none());
    assert_eq!(uri.sub_path.unwrap(), "/ChannelClass/Title".to_string());
    assert!(uri.query.is_none());
}


#[test]
fn test_not_an_uri_0() {
    let uri = Uri::parse(
        "eml://",
    );
    assert!(uri.is_err());
}

#[test]
fn test_not_an_uri_1() {
    let uri = Uri::parse(
        "not an uri",
    );
    assert!(uri.is_err());
}
