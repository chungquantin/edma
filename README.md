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

<p>
 <a href="https://github.com/nomadiz/edma"><img src="https://img.shields.io/badge/built_with-Rust-dca282.svg?style=flat-square"></a>
&nbsp;
<a href="https://github.com/nomadiz/edma"><img src="https://img.shields.io/github/v/release/nomadiz/edma?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
&nbsp;
<a href="https://github.com/nomadiz/edma/blob/master/LICENSE">
<img src="https://img.shields.io/badge/license-MIT License-00bfff.svg?style=flat-square"></a>
&nbsp;
    <a href="https://github.com/nomadiz/edma/graphs/contributors" alt="Contributors">
        <img src="https://img.shields.io/github/contributors/nomadiz/edma" /></a>
    <a href="https://github.com/nomadiz/edma/pulse" alt="Activity">
        &nbsp;
        <img src="https://img.shields.io/github/commit-activity/m/nomadiz/edma" />
	</a>
</p>

## What is EDMA?

**EDMA: Embedded Database Management for All** is an open source project made to manage embedded key-value storages. EDMA is a TUI (Terminal User Interface) that is easy to install and configure. It allows engineer to traverse the embedded database and deserialize byte data with provided byte layout. This enhances the experience of working with low level database system like RocksDB or Redb.

## Features

-   Multi embedded database supported: `RocksDB`, `Redb`
-   Cross-platform supported: `Windows`, `Linux` and `MacOS`
-   Custom byte layout deserialization
-   Execute database command directly in terminal
-   Interactive terminal interface with keyboard only control

## Usage

Run EDMA terminal application

```powershell
$ edma
```

Set a config file path

```shell
$ edma --config-file [PATH]
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
