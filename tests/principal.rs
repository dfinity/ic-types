use std::convert::TryFrom;

use ic_types::Principal;

#[test]
fn try_from_succeed_with_valid_vec_u8() {
    let p = Principal::try_from(vec![0x04]);
    assert!(p.is_ok());
}

#[test]
fn try_from_falied_with_none_valid_vec_u8() {
    let p = Principal::try_from(vec![0x00, 0x04]);
    dbg!(&p);
    assert!(p.is_err());
}