// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT

use etpproto::credentials::create_client_info;
use etpproto::credentials::BasicCredential;
use etpproto::credentials::Credentials;

#[test]
fn test_client_info_uid() {
    let ci_0 = create_client_info(None, None, None);
    let ci_1 = create_client_info(None, None, None);

    assert_ne!(ci_0.uid, ci_1.uid);
}

#[test]
fn test_client_info_values() {
    let ci_0 = create_client_info(
        Some("192.168.0.1".to_string()),
        Some(Credentials::Basic(BasicCredential {
            login: "admin".to_string(),
            password: "password".to_string(),
        })),
        None,
    );
    assert_eq!(ci_0.ip.unwrap(), "192.168.0.1".to_string());
    match ci_0.credentials.unwrap() {
        Credentials::Basic(basic) => {
            assert_eq!(basic.login, "admin".to_string());
            assert_eq!(basic.password, "password".to_string())
        }
        Credentials::Bearer(_) => panic!("Wrong type"),
    };
    assert_eq!(ci_0.capabilities.len(), 0);
}
