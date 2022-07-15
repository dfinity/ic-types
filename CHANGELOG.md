# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1] - 2022-07-15

### Changed
- Remove deprecation of `Principal::from_slice`.

## [0.4.0] - 2022-07-13

### Changed
- `PrincipalError` enum has different set of variants reflecting changes in `Principal::from_text` logic.
- `Principal::from_text` accepts input containing uppercase letters which results in Err before.
- `Principal::from_text` verifies CRC32 check sequence.
- `Principal::from_slice` is deprecated in favor of `Principal::try_from_slice`.

### Fixed
- Converting long bytes ending in `0x04` to `Principal` is ok.

## [0.3.0] - 2022-01-04

### Changed
- lookup_path uses an Iterator rather than an array.
- Principal, Label, and HashTree implement From instead of Into in order to remove needless borrowing.
