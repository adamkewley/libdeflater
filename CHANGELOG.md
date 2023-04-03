# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
