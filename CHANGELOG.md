# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.23.0]

- Updated libdeflate to v1.23 (#40, thanks @musicinmybrain)
- Fixed minor linting issue (#39, thanks @musicinmybrain)

## [1.22.0]

- Updated libdeflate to v1.22 (#38, thanks @musicinmybrain)

## [1.21.0]

- Updated libdeflate to v1.21 (#37, thanks @musicinmybrain)

## [1.20.0]

- Updated libdeflate to v1.20 (#34, thanks @musicinmybrain)
- Some files (e.g. benchmark inputs) are now excluded from the built crate (#33, thanks @musicinmybrain)

## [1.19.3]

- The mechanism where libdeflate is found via `pkgconfig` is now behind a `dynamic`
  feature flag, which lets downstream package users configure whether they want to
  use the in-tree copy of libdeflate or one provided from `pkgconfig` (thanks
  @joshtoik1, #32)

## [1.19.2]

- Fixed a packaging issue where libdeflate's sources weren't packaged in the cargo crate
  (thanks for reporting, @Brooooooklyn, #31)

## [1.19.1]

- Libdeflate-sys now finds libdeflate via `pkgconfig` when it's available, rather than
  using the in-tree version (#30, thanks @musicinmybrain)

## [1.19.0]

- Updated libdeflate to v1.19 (#28)
- Because the API of `libdeflater` has been (effectively) frozen for several
  years, the versioning of the library was changed to match upstream `libdeflate`

## [0.14.0]

- Updated libdeflate to v1.18 (#27)

## [0.13.0]

- Added `Adler32` struct and `adler32` helper functions, which expose libdeflate's high-performance
  adler32 checksum algorithm to library users (#26 - thanks @peterdk)

## [0.12.0]

- Updated libdeflate to v1.17

## [0.11.0]

- Updated libdeflate to v1.14 (thanks @nickbabcock)

## [0.10.0]

- Fixed `CompressionLvl::iter` not returning compression lvl 0 as
  its first element (#21)

## [0.9.0]

- Added support for compression lvl 0 (#21)

## [0.8.0]

- Updated libdeflate from around v1.6 to v1.10
