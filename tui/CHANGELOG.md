# CHANGELOG

## [0.1.0-beta.5] - 2023-01-25

### Added

-   Migrate from fixed column family design to TagBucket datastructure
-   Allows more databases to be integrated into EDMA using TagBucket
-   Supported tags: `tree` (for Sled), `column_family` (for RocksDB)
-   Add Sled integration: Allows iterate and view database item stored globally and stored in a tree using command `TREE=<tree_name_goes_here>`

## [0.1.0-beta.4] - 2022-12-13

### Added

-   Support PREFIX and SUFFIX iteration

## [0.1.0-beta.3] - 2022-12-09

### Added

-   General navigation improvements
-   Config file management
-   EDMA cli to set config file
-   Add byte layout deserialization
-   Configure custom layout feature
-   View and select byte layout
-   Minor theme improvements

# What is this?

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
