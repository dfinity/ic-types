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
