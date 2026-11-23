// SPDX-FileCopyrightText: MailCraft contributors
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Which hosts the owner agreed to older encryption for (`allow_older_encryption`).

use super::{allow_older_encryption, older_encryption_allowed};

#[test]
fn a_host_is_allowed_only_once_the_owner_agreed_and_whatever_its_case_or_final_dot() {
    let host = "older-a.example.org";
    assert!(!older_encryption_allowed(host));
    allow_older_encryption(host, true);
    assert!(older_encryption_allowed("Older-A.Example.ORG"));
    assert!(older_encryption_allowed(" older-a.example.org. "));
    allow_older_encryption("OLDER-A.example.org", false);
    assert!(!older_encryption_allowed(host));
}

#[test]
fn the_agreement_is_for_one_host_alone() {
    allow_older_encryption("older-b.example.org", true);
    assert!(!older_encryption_allowed("older-c.example.org"));
    assert!(!older_encryption_allowed("example.org"));
    assert!(!older_encryption_allowed("sub.older-b.example.org"));
    allow_older_encryption("older-b.example.org", false);
}

#[test]
fn nothing_is_kept_for_an_empty_name() {
    allow_older_encryption("", true);
    allow_older_encryption(" . ", true);
    assert!(!older_encryption_allowed(""));
    assert!(!older_encryption_allowed("."));
}
