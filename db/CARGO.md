## Embedded database library for EDMA

**NOTE:** (SolomonDB Forked)

### Description

Embedded database library that can be installed as Rust crate. This can be used to run an embedded graph database on top of other multiple storage engines

### Storage layer

| Name        | Type      | Concurrency     | Description                                                                                                        |
| ----------- | --------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| **RocksDB** | key-value | Multi-threaded  | OptimisticTransactionDB of RocksDB is applied into SolomonDB to allow ACID transaction with multithreaded feature. |
| **Redb**    | key-value | Single-threaded | Simple use case of Redb is efficient for simple on-disk store.                                                     |
| **Sled**    | key-value | Single-threaded | The champagne of beta embedded databases                                                                           |
