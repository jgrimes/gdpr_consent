// Copyright (c) 2018 The gdpr_consent authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate chrono;
extern crate gdpr_consent;

use gdpr_consent::vendor_consent::VendorConsent;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let vendor_consent = "BOEFEAyOEFEAyAHABDENAI4AAAB9vABAASA".parse()?;
    let VendorConsent::V1(mut v1) = vendor_consent;

    v1.last_updated = "2018-05-11T12:00:00.000Z".parse()?;
    v1.vendor_consent.remove(9); // remove consent for Vendor ID 10

    let serialized = VendorConsent::V1(v1).to_string()?;
    assert_eq!(serialized, "BOEFEAyONlzmAAHABDENAI4AAAB9vABgASABQA");

    Ok(())
}
