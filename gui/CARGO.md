<pre align="center">
Built by @nomadiz

███████╗██████╗ ███╗   ███╗ █████╗ 
██╔════╝██╔══██╗████╗ ████║██╔══██╗
█████╗  ██║  ██║██╔████╔██║███████║
██╔══╝  ██║  ██║██║╚██╔╝██║██╔══██║
███████╗██████╔╝██║ ╚═╝ ██║██║  ██║
╚══════╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝
-----------------------------------
Embedded Database Management for All
</pre>
<br/>

<p align="center">
 <a href="https://github.com/nomadiz/edma"><img src="https://img.shields.io/badge/built_with-Rust-dca282.svg?style=flat-square"></a>
<a href="https://crates.io/crates/edma"><img src="https://img.shields.io/crates/v/edma.svg?logo=rust"/></a>
<a href="https://github.com/nomadiz/edma"><img src="https://img.shields.io/github/v/release/nomadiz/edma?color=%23ff00a0&include_releases&label=version&sort=semver&style=flat-square"></a>
<a href="https://github.com/nomadiz/edma/blob/master/LICENSE">
<img src="https://img.shields.io/badge/license-MIT License-00bfff.svg?style=flat-square"></a>
<a href="https://github.com/nomadiz/edma/graphs/contributors" alt="Contributors">
<img src="https://img.shields.io/github/contributors/nomadiz/edma?color=green" /></a>
<a href="https://github.com/nomadiz/edma/pulse" alt="Activity">
	</a>
</p>
<p align="center">
<a href="https://www.producthunt.com/posts/edma?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-edma" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=370554&theme=dark" alt="EDMA - A&#0032;terminal&#0032;app&#0032;for&#0032;embedded&#0032;database&#0032;management&#0046; | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a>
</p>

## What is EDMA?

<p align="center">
<img width="80%" style="
box-shadow: 0px 4px 59px 22px rgba(0, 0, 0, 0.56);
border-radius: 10px;" src="https://user-images.githubusercontent.com/56880684/206833658-97091acd-11c7-4b78-b08b-9ce2aeb365a8.gif"/>
</p>

**EDMA: Embedded Database Management for All** is an open source project made to manage embedded key-value storages. EDMA is a TUI (Terminal User Interface) that is easy to install and configure. It allows engineer to traverse the embedded database and deserialize byte data with provided byte layout. This enhances the experience of working with low level database system like RocksDB or Redb.

## Features

-   Multi embedded database supported: `RocksDB`, `Redb`, `Sled
-   `
-   Cross-platform supported: `Windows`, `Linux` and `MacOS`
-   Custom byte layout deserialization
-   Execute database command directly in terminal
-   Interactive terminal interface with keyboard only control
-   Iterate key-value pairs from column family and table

## Roadmap

-   [ ] NEW: Universal Key Value Storage support (UKV)
-   [x] NEW: Sled support
-   [ ] NEW: LevelDB support
-   [ ] Adding consistent mode for editor view

## Supported Storages

EDMA supports multiple databases with easy plug-to-play architecture. Please check below list for supported databases and its features:

| Database name | Description                                                    | EDMA release                                                                | Pull request                                                |
| ------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------- | ----------------------------------------------------------- |
| RocksDB       | Support both non-column and column byte data viewer (`COLUMN`) | [v0.1.0-beta.4](https://github.com/nomadiz/edma/releases/tag/v0.1.0-beta.4) | N/A                                                         |
| ReDB          | Support default database (Will add `TABLE` view)               | [v0.1.0-beta.4](https://github.com/nomadiz/edma/releases/tag/v0.1.0-beta.4) | N/A                                                         |
| Sled          | Support both non-tree and tree byte data viewer (`TREE`)       | [v0.1.0-beta.5](https://github.com/nomadiz/edma/releases/tag/v0.1.0-beta.5) | [#8 Sled support](https://github.com/nomadiz/edma/issues/8) |

To create a PR for a database integration, please go to [`Issues > New Issue > Feature request`](https://github.com/nomadiz/edma/issues/new?assignees=&labels=&template=feature_request.md&title=)

## Getting Started

### Installation

#### With Cargo (Linux, macOS, Windows)

If you already have a Rust environment set up, you can use the `cargo install` command:

```
cargo install --version 0.1.0-beta.3 edma
```

#### From binaries (Linux, macOS, Windows)

-   Download the [latest release binary](https://github.com/nomadiz/edma/releases) for your system
-   Set the `PATH` environment variable

### Set a config path

Configuration file is where you identify path to databases and EDMA byte templates. To set a config path, using a CLI command

```
$ edma --config-path [PATH_TO_FILE]
```

Please view [EDMA Configuration file](https://github.com/nomadiz/edma#configuration) to learn more how configuration file works.

### Usage

Run EDMA terminal application

```powershell
$ edma
```

Set a config file path

```shell
$ edma --config-path [PATH]
```

Using `help` command

```shell
$ edma --help

edma 0.1.0
A cross-platform TUI database management tool written in Rust

USAGE:
    gui [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config-path <config-path>    Set the config file
```

## Why use EDMA?

### Compatible with multiple databases

<p align="center">
<img width="80%" style="
box-shadow: 0px 4px 59px 22px rgba(0, 0, 0, 0.56);
border-radius: 10px;" src="https://user-images.githubusercontent.com/56880684/206836453-3834a473-363b-4b7e-af27-fbbc6115a3bc.png"/>
</p>

**EDMA** is a very first database management tool designed for embedded databases. Applying adapter design pattern into database storage implementation, it makes integration with databases become easier. EDMA supports two databases by default: `RocksDB` and `ReDB`. To integrate with other embedded databases, you can add the implementation in [EDMA storage layer](https://github.com/nomadiz/edma/tree/master/db/src/storage/kvs)

### Interactive terminal UI

<p align="center">
<img width="80%" style="
box-shadow: 0px 4px 59px 22px rgba(0, 0, 0, 0.56);
border-radius: 10px;" src="https://user-images.githubusercontent.com/56880684/206836166-0699e5cd-e954-4499-9a7e-7a4aeb43eabc.png"/>
</p>

EDMA is built using Rust library `tui-rs` which makes it to be an app that can be run directly on your terminal. No startup time needed and it's extremely light weight. Run every where, every time, all at once

### Template management for byte deserializer

<p align="center">
<img width="80%" style="
box-shadow: 0px 4px 59px 22px rgba(0, 0, 0, 0.56);
border-radius: 10px;" src="https://user-images.githubusercontent.com/56880684/206836189-9de85d33-9a07-4e27-a182-d6cd0db83569.png"/>
</p>

Data in embedded database is different from data presented in relational databases. While relational databases label data with specific type and attributes when it is created, embedded database can't do that. The only data type that embedded database displays is byte array. For example, `[0 0 0 1 50 32 20]`. It is not human readable.

Using EDMA, byte data can be converted into human readable data using EDMA byte template system.

Instruction on how EDMA byte deserializer works: [What is EDMA templates?](https://github.com/nomadiz/edma#templates)

### Command editor

Command editor is one core feature of EDMA, it allows you to manage byte data using advanced commands. The image below shows how a database column family can iterated using command editor

<p align="center">
<img width="80%" style="
box-shadow: 0px 4px 59px 22px rgba(0, 0, 0, 0.56);
border-radius: 10px;" src="https://user-images.githubusercontent.com/56880684/206836218-8f115413-4b8d-4c88-a192-06e4eca3a697.png"/>
</p>

## Keymap

| Key                                                    | Description                      |
| ------------------------------------------------------ | -------------------------------- |
| <kbd>ENTER</kbd>                                       | Enter focused section            |
| <kbd>ESC</kbd>                                         | Escape from focused section      |
| <kbd>9</kbd>, <kbd>0</kbd>                             | Scroll up/down databases         |
| <kbd>h</kbd>, <kbd>j</kbd>                             | Scroll up/down key byte layout   |
| <kbd>k</kbd>, <kbd>l</kbd>                             | Scroll up/down value byte layout |
| <kbd>←</kbd>, <kbd>→</kbd>, <kbd>↑</kbd>, <kbd>↓</kbd> | Move focus to left/right/up/down |
| <kbd>h</kbd>, <kbd>d</kbd>, <kbd>l</kbd>               | Switch to home/databases/layouts |
| <kbd>q</kbd>                                           | Quit                             |

## EDMA Command

EDMA supports inline command to interact with embedded databases. The list of supported commands are

### - `COLUMN` or `TABLE`

Iterate with defined column famility or table

#### Arguments

-   `String`: Column family name

### - `PREFIX` or `SUFFIX`

Iterate filtered by prefix or suffix.

Note: This command is for key iteration not value iteration.

#### Arguments

-   `String`: Prefix value or suffix value

## Configuration

### Databases

Database name should be these two below

-   `rocksdb`: RocksDB
-   `redb`: Redb

Database path should be `String` type

### Templates

Byte template is an instruction combined by one or multiple byte layouts. It provides EDMA deserializer information about bytes data. To explain the use of byte template and byte layout, we have this example:

```
Original= edma2022github
Bytes= [65 64 6d 61 32 30 32 32 67 69 74 68 75 62]
```

Slicing data and labelling data as EDMA byte template, we have

```
[1]
original=edma
from=0
to=4
variant=String

[2]
original=2022
from=4
to=8
variant=Int32

[3]
original=github
from=8
to=13
variant=String
```

### Example

Configuration file example

```json
{
	"databases": [
		{
			"name": "rocksdb",
			"path": "/temp"
		},
		{
			"name": "sled",
			"path": "/temp/sled"
		}
	],
	"templates": [
		{
			"name": "Custom layout",
			"layouts": [
				{
					"name": "name",
					"from": 0,
					"to": 5,
					"variant": "String"
				},
				{
					"name": "id",
					"from": 5,
					"to": 10,
					"variant": "Int64"
				}
			]
		}
	]
}
```

## Tribute

Without these awesome open source projects, EDMA can't be complete. Please share the spotlight with these repo below:[`gobang`](https://github.com/TaKO8Ki/gobang), [`tui-rs`](https://github.com/fdehau/tui-rs), [`spotify-tui`](https://github.com/Rigellute/spotify-tui) and [`tui-re-tree-widget`](https://github.com/EdJoPaTo/tui-rs-tree-widget)

## Community

-   View Nomadic Engineers blog [Blog](https://nomadiz.hashnode.dev/)
-   Support the creator [Twitter](https://twitter.com/chasechung111)
