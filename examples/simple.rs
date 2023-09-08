// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT

use etpproto::uri::canonical_data_object_uris;

fn main() {
    println!("{:?}", canonical_data_object_uris())
}
