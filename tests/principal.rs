use ic_types::Principal;
use ic_types::PrincipalError;

const MANAGEMENT_CANISTER_BYTES: [u8; 0] = [];
const MANAGEMENT_CANISTER_TEXT: &str = "aaaaa-aa";

const ANONYMOUS_CANISTER_BYTES: [u8; 1] = [4u8];
const ANONYMOUS_CANISTER_TEXT: &str = "2vxsx-fae";

const TEST_CASE_BYTES: [u8; 9] = [0xef, 0xcd, 0xab, 0, 0, 0, 0, 0, 1];
const TEST_CASE_TEXT: &str = "2chl6-4hpzw-vqaaa-aaaaa-c";

mod convert_from_bytes {
    use super::*;

    #[test]
    fn try_from_test_case_ok() {
        assert!(Principal::try_from_slice(&TEST_CASE_BYTES).is_ok());
    }

    #[test]
    fn try_from_slice_length_0_29_ok() {
        let array = [0u8; 29];
        for len in 0..30 {
            assert!(Principal::try_from_slice(&array[..len]).is_ok());
        }
    }

    #[test]
    fn try_from_slice_length_30_err() {
        assert_eq!(
            Principal::try_from_slice(&[0u8; 30]),
            Err(PrincipalError::BufferTooLong())
        );
    }

    #[test]
    fn from_test_case_ok() {
        Principal::from_slice(&TEST_CASE_BYTES);
    }

    #[test]
    fn from_slice_length_0_29_ok() {
        let array = [0u8; 29];
        for len in 0..30 {
            Principal::from_slice(&array[..len]);
        }
    }

    #[test]
    #[should_panic]
    fn from_slice_length_30_err() {
        Principal::from_slice(&[0u8; 30]);
    }
}

mod convert_from_text {
    use super::*;

    #[test]
    fn convert_from_test_case_ok() {
        // input must be 8~63 chars including `-`s
        // using empty blob as shortest test case
        assert!(Principal::from_text(TEST_CASE_TEXT).is_ok());
    }

    #[test]
    fn convert_from_8_chars_ok() {
        // input must be 8~63 chars including `-`s
        // using empty blob as shortest test case
        assert!(Principal::from_text(MANAGEMENT_CANISTER_TEXT).is_ok());
    }

    #[test]
    fn convert_from_7_chars_err() {
        assert_eq!(
            Principal::from_text("aaaaa-a"),
            Err(PrincipalError::TextTooSmall())
        );
    }

    #[test]
    fn convert_from_63_chars_ok() {
        // input must be 8~63 chars including `-`s
        // using [0u8; 29] as longest test case
        assert!(Principal::from_text(
            "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaa"
        )
        .is_ok());
    }

    #[test]
    fn convert_from_64_chars_err() {
        // TODO: The exact error type will be changed
        assert!(matches!(
            Principal::from_text(
                "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaa"
            ),
            Err(PrincipalError::AbnormalTextualFormat(..))
        ));
    }
}

mod convert_to_bytes {
    use super::*;

    #[test]
    fn managements_canister_converts_to_empty_slice() {
        assert_eq!(
            Principal::management_canister().as_slice(),
            &MANAGEMENT_CANISTER_BYTES
        );
    }

    #[test]
    fn anonymouse_converts_to_single_byte_04() {
        assert_eq!(Principal::anonymous().as_slice(), &ANONYMOUS_CANISTER_BYTES);
    }

    #[test]
    fn test_case_to_bytes_correct() {
        assert_eq!(
            Principal::from_text(TEST_CASE_TEXT).unwrap().as_slice(),
            &TEST_CASE_BYTES
        );
    }
}

mod convert_to_text {
    use super::*;

    #[test]
    fn managements_canister_to_text_correct() {
        assert_eq!(
            Principal::management_canister().to_text(),
            MANAGEMENT_CANISTER_TEXT.to_string()
        );
    }

    #[test]
    fn anonymous_to_text_correct() {
        assert_eq!(
            Principal::anonymous().to_text(),
            ANONYMOUS_CANISTER_TEXT.to_string()
        );
    }

    #[test]
    fn test_case_to_text_correct() {
        assert_eq!(
            Principal::try_from_slice(&TEST_CASE_BYTES)
                .unwrap()
                .to_text(),
            TEST_CASE_TEXT.to_string()
        );
    }
}

mod ser_de {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    #[test]
    fn management_canister_serde_match() {
        let p = Principal::management_canister();
        assert_tokens(&p.compact(), &[Token::Bytes(&MANAGEMENT_CANISTER_BYTES)]);
        assert_tokens(&p.readable(), &[Token::Str(MANAGEMENT_CANISTER_TEXT)]);
    }

    #[test]
    fn test_case_serde_match() {
        let p = Principal::try_from_slice(&TEST_CASE_BYTES).unwrap();
        assert_tokens(&p.compact(), &[Token::Bytes(&TEST_CASE_BYTES)]);
        assert_tokens(&p.readable(), &[Token::Str(TEST_CASE_TEXT)]);
    }
}

#[test]
fn impl_traits() {
    use serde::{Deserialize, Serialize};
    use std::convert::TryFrom;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;
    use std::str::FromStr;

    assert!(impls::impls!(
        Principal: Debug & Display & Clone & Copy & Eq & PartialOrd & Ord & Hash
    ));

    assert!(
        impls::impls!(Principal: FromStr & TryFrom<&'static str> & TryFrom<Vec<u8>> & TryFrom<&'static Vec<u8>> & TryFrom<&'static [u8]> & AsRef<[u8]>)
    );

    assert!(impls::impls!(Principal: Serialize & Deserialize<'static>));
}
