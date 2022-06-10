use std::convert::TryFrom;

use ic_types::Principal;
use ic_types::PrincipalError;

mod convert_from_bytes {
    use super::*;

    #[test]
    fn try_from_slice_length_0_29_ok() {
        // Principal::try_from_slice(bytes: &[u8])
        let array = [0u8; 29];
        for len in 0..30 {
            assert!(Principal::try_from_slice(&array[..len]).is_ok());
        }
    }

    #[test]
    fn try_from_slice_length_30_err() {
        // Principal::try_from_slice(bytes: &[u8])
        assert_eq!(
            Principal::try_from_slice(&[0u8; 30]),
            Err(PrincipalError::BufferTooLong())
        );
    }

    #[test]
    fn try_from_vec_u8_length_0_29_ok() {
        // impl TryFrom<Vec<u8>> for Principal
        for len in 0..30 {
            assert!(Principal::try_from(vec![0u8; len]).is_ok());
        }
    }

    #[test]
    fn try_from_vec_u8_length_30_err() {
        // impl TryFrom<Vec<u8>> for Principal
        assert_eq!(
            Principal::try_from(vec![0u8; 30]),
            Err(PrincipalError::BufferTooLong())
        );
    }

    #[test]
    fn try_from_ref_vec_u8_length_0_29_ok() {
        // impl TryFrom<&Vec<u8>> for Principal
        for len in 0..30 {
            assert!(Principal::try_from(&vec![0u8; len]).is_ok());
        }
    }

    #[test]
    fn try_from_ref_vec_u8_length_30_err() {
        // impl TryFrom<&Vec<u8>> for Principal
        assert_eq!(
            Principal::try_from(&vec![0u8; 30]),
            Err(PrincipalError::BufferTooLong())
        );
    }

    #[test]
    fn try_from_u8_slice_length_0_29_ok() {
        // impl TryFrom<[u8]> for Principal
        let array = [0u8; 29];
        for len in 0..30 {
            assert!(Principal::try_from(&array[..len]).is_ok());
        }
    }

    #[test]
    fn try_from_u8_slice_length_30_err() {
        // impl TryFrom<[u8]> for Principal
        let array = [0u8; 30];
        assert_eq!(
            Principal::try_from(&array[..]),
            Err(PrincipalError::BufferTooLong())
        );
    }
}

mod convert_from_text {
    use super::*;

    mod from_text {
        use super::*;

        #[test]
        fn convert_from_8_chars_ok() {
            // input must be 8~63 chars including `-`s
            // using empty blob as shortest test case
            assert!(Principal::from_text("aaaaa-aa").is_ok());
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

    mod try_from_ref_str {
        use super::*;

        #[test]
        fn convert_from_8_chars_ok() {
            // input must be 8~63 chars including `-`s
            // using empty blob as shortest test case
            assert!(Principal::try_from("aaaaa-aa").is_ok());
        }

        #[test]
        fn convert_from_7_chars_err() {
            assert_eq!(
                Principal::try_from("aaaaa-a"),
                Err(PrincipalError::TextTooSmall())
            );
        }

        #[test]
        fn convert_from_63_chars_ok() {
            // input must be 8~63 chars including `-`s
            // using [0u8; 29] as longest test case
            assert!(Principal::try_from(
                "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaa"
            )
            .is_ok());
        }

        #[test]
        fn convert_from_64_chars_err() {
            // TODO: The exact error type will be changed
            assert!(matches!(
                Principal::try_from(
                    "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaa"
                ),
                Err(PrincipalError::AbnormalTextualFormat(..))
            ));
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn convert_from_8_chars_ok() {
            // input must be 8~63 chars including `-`s
            // using empty blob as shortest test case
            assert!("aaaaa-aa".parse::<Principal>().is_ok());
        }

        #[test]
        fn convert_from_7_chars_err() {
            assert_eq!(
                "aaaaa-a".parse::<Principal>(),
                Err(PrincipalError::TextTooSmall())
            );
        }

        #[test]
        fn convert_from_text_63_chars_ok() {
            // input must be 8~63 chars including `-`s
            // using [0u8; 29] as longest test case
            assert!(
                "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaa"
                    .parse::<Principal>()
                    .is_ok()
            );
        }

        #[test]
        fn convert_from_text_64_chars_err() {
            // TODO: The exact error type will be changed
            assert!(matches!(
                "bnkmk-jqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaa"
                    .parse::<Principal>(),
                Err(PrincipalError::AbnormalTextualFormat(..))
            ));
        }
    }
}

mod convert_to_bytes {
    use super::*;

    #[test]
    fn managements_canister_converts_to_empty_slice() {
        assert_eq!(Principal::management_canister().as_slice(), &[]);
    }

    #[test]
    fn anonymouse_converts_to_single_byte_04() {
        assert_eq!(Principal::anonymous().as_slice(), &[04]);
    }

    #[test]
    fn managements_canister_as_ref_empty_slice() {
        assert_eq!(Principal::management_canister().as_ref(), &[]);
    }

    #[test]
    fn anonymouse_converts_as_ref_single_byte_04() {
        assert_eq!(Principal::anonymous().as_ref(), &[04]);
    }
}

mod convert_to_text {
    use super::*;

    #[test]
    fn managements_canister_to_text_correct() {
        assert_eq!(
            Principal::management_canister().to_text(),
            "aaaaa-aa".to_string()
        );
    }

    #[test]
    fn anonymous_to_text_correct() {
        assert_eq!(Principal::anonymous().to_text(), "2vxsx-fae".to_string());
    }

    #[test]
    fn managements_canister_display_correct() {
        // impl Display
        assert_eq!(
            format!("{}", Principal::management_canister()),
            "aaaaa-aa".to_string()
        );
    }

    #[test]
    fn anonymous_display_correct() {
        assert_eq!(
            format!("{}", Principal::anonymous()),
            "2vxsx-fae".to_string()
        );
    }

    #[test]
    fn managements_canister_to_string_correct() {
        // ToString trait was automatically implemented because impl Display trait
        assert_eq!(
            Principal::management_canister().to_string(),
            "aaaaa-aa".to_string()
        );
    }

    #[test]
    fn anonymous_to_string_correct() {
        assert_eq!(Principal::anonymous().to_string(), "2vxsx-fae".to_string());
    }
}

mod ser_de {
    use ic_types::Principal;
    use serde_test::{assert_tokens, Configure, Token};

    #[test]
    fn management_canister_serde_match() {
        let p = Principal::management_canister();
        assert_tokens(&p.compact(), &[Token::Bytes(&[])]);
        assert_tokens(&p.readable(), &[Token::Str("aaaaa-aa")]);
    }
}

mod derive_traits {
    use super::*;

    #[test]
    fn impl_copy_and_partial_eq() {
        // Eq cannot be checked at
        let x = Principal::management_canister();
        let y = x;
        assert!(x.eq(&y));
    }

    #[test]
    fn impl_ord() {
        // Ording
        let x = Principal::management_canister();
        let y = Principal::anonymous();
        assert_eq!(x.cmp(&y), std::cmp::Ordering::Less);
    }

    #[test]
    fn impl_hash() {
        use std::hash::Hash;
        let x = Principal::management_canister();
        let y = x;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        assert_eq!(x.hash(&mut hasher), y.hash(&mut hasher));
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
