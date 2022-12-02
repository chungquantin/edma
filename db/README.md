## SolomonDB: Embedded database library

### Description

SolomonDB provides an embedded database library that can be installed as Rust crate. This can be used to run an embedded graph database on top of other multiple storage engines

### Storage layer

| Name        | Type      | Concurrency     | Description                                                                                                        |
| ----------- | --------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| **RocksDB** | key-value | Multi-threaded  | OptimisticTransactionDB of RocksDB is applied into SolomonDB to allow ACID transaction with multithreaded feature. |
| **Redb**    | key-value | Single-threaded | Simple use case of Redb is efficient for simple on-disk store.                                                     |

### Installation guide

SolomonDB can be used as a Rust embedded storage. The database is published in crates: https://crates.io/crates/solomondb. To add SolomonDB to your project and start building on top of it, using this command

```
cargo add solomondb
```

Or add solomondb package into your toml file dependecy list

```toml
[dependencies]
solomondb = "0.0.1-beta.1"
```

### Getting started

The current version of **SolomonDB** allows you to work wit RocksDB or Redb using **Gremlin** query language. Hence, applying graph data structure to manage key-value pairs in those embedded storage layer. SolomonDB embedded storage is easy to set up. You only need to identify the path where your database will be located and it's good to go.

```rs
use solomondb::Datastore;

let datastore = Datastore::new(path);
let db = Database::new(datastore.borrow());
```

SolomonDB supports GQL (Gremlin Query Language) and it does not require Gremlin Server or Apache TinkerPop to operate. You can query data on top of embedded storages. Below are examples on how to create vertices, properties and traverse to retrieve data.

```rs
// Create two new vertices with properties
let t1 = db
.traverse()
.v(1)
.add_v("person")
.property("github", "tin-snowflake")
.property("name", "Tin Chung")
.property("age", 21)
.add_v("coder")
.property("github", "chungquantin")
.property("age", 30);

// Traverse vertices which have property "github" and label "person"
let t2 = t1.clone().has_key("github").has_label("person").exec().to_list().await.unwrap();
// Traverse vertices which have property "github"
let t3 = t1.clone().has_key("github").exec().to_list().await.unwrap();
// Traverse vertices which does not have property "name"
let t4 = t1.clone().has_not("name").exec().next().await.unwrap();
```
