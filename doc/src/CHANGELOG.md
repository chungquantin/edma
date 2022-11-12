# What is it about?

A place to record the progress of building Solomon DB. This can be tracked as a commit changelog with detailed explanation.

**Note**: `Commits` section won't include commit related to updating DEVLOG or CHANGELOG.

# Commit history

## Commits on Oct 26, 2022

### Description

A soft beginning. I just spend some time to read graph database book and learning concepts, requirements needed to construct Solomon DB.

### Detail explanation

There are a few interesting learning outcomes acquired here:

These concepts are not too complicated actually. Not talking about native graph database, non-native graph database is more like a database wrapper which has internals built on top of other databases.

```md
-   Underlying storage
-   Processing engine
-   Graph compute engine
```

_Learnt and noted down these concepts in **WIKI.md**._

As this is the first commit, it only includes code to initialize a codebase and its basic layout for Rust project using `cargo new`.

```rust
// A simple hello world program (db/src/main.rs)
fn main() {
    println!("Hello, world!");
}
```

---

## Commits on Oct 28, 2022

### Description

Add `StorageDriver`, new type `RocksDBDriver`, macro `impl_driver_core` and misc implementation for `StorageErr` and `Status`.

### Detail explanation

Things getting more exciting when we touch advanced syntaxes in Rust: [`New Type`](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) design pattern and [`Macro`](https://doc.rust-lang.org/book/ch19-06-macros.html). Before jumping right into the main point of today changes, one of a missing features in Rust compared to other OOP languages must be mentioned first is the ability to do **inheritance**.

With what Rust provides so far, it is quite tricky to construct an inheritable `struct` object. At first, you may think doing OOP in Rust is quite bad as inheritance is a core component of any OOP design. However, if you learn more about OOP, you will know `Inheritance < Composition`. This is quite off topic, but there is a few discussions and articles related to the way Rust do OOP that you should read:

-   [Rust Internals | Discussion - Why not inheritance?](https://internals.rust-lang.org/t/why-not-inheritance/5738/14)
-   [Where Rust enum shines?](http://smallcultfollowing.com/babysteps/blog/2015/05/05/where-rusts-enum-shines/)

Briefly, even though Rust does not have an obvious OOP feature like inheritance, it still provides several approaches: `Trait`, `New type`, `Macro` or `Enum` to do thing in a inheritance way but cleaner as things are separated elegantly.

**New Type design pattern for RocksDB Driver**

"_The `newtype` idiom gives compile time guarantees that the right type of value is supplied to a program_" - Rust book.

I apply the design to implement RocksDB Driver. The idea is to have a parent `struct` called `StorageDriver` and we will build a new type `RocksDBDriver` from that.

```rs
pub struct StorageDriver<T> {
 pub db_instance: Option<T>,
 pub path: String,
}

pub struct RocksDBDriver(StorageDriver<DB>);
```

However, `NewType` design does not exactly like inheritance. For example, if you want to get field `db_instance` from `RocksDBDriver`, you can't do `self.db_instance` but `self.0.db.instance`. Can imagine `db_instance` is inside `StorageDriver` while `StorageDriver` is wrapped inside new type `RocksDBDriver`. This would make the syntax looks a bit dirty. The solution I used to handle this case is `Macro`.

Macro provides an ability to maximize the power of Rust syntax. I wrote a macro to implement all below methods used inside any `impl` calling to `impl_driver_core`. Then to get the inner of new type, we just have to use `get_core`.

```rs
macro_rules! impl_driver_core {
 ($DbType: ty) => {
  fn get_core(self: &mut Self) -> &mut StorageDriver<$DbType> {
   &mut self.0
  }
 };
}

pub(crate) use impl_driver_core;
```

Struct `RocksDBDriver` now becomes:

```rs
impl RocksDBDriver {
 super::core::impl_driver_core!(DB);

 /// # Initializing RocksDB driver
 pub fn initialize(self: &mut Self, cf_name: &String, options: rocksdb::Options) {
  let core = self.get_core();
  ...
 }
}
```

---

## Commits on Nov 1, 2022

### Description

The prior implementation of `RocksDBDriver` was using the database instance initialized using `DB` object. And all methods are called from the object `DB` instance too. However, this approach has some limitations in terms of concurrency operations. Briefly, I migrate it from using `DB` to handling operations using transactions - a better control flow approach. Instead of using `TransactionDB`, I choose `OptimisticTransactionDB` which is more efficient for read contention database, correct me if I am wrong.

You can read more about this in [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki/Transactions#optimistictransactiondb)

### Detail explanation

Even though there is only one commit and only today, this commit has a lot of code refractors. An obvious refractor is migrating from `Driver` to `Adapter` which follows adapter pattern.

-   `StorageDriver` to `StorageAdapter`
-   `RocksDBDriver` to `RocksDBAdapter`

Walk through the implementation of `StorageAdapter`, we have

```rs
pub struct StorageAdapter<T> {
    pub name: StorageAdapterName,
    pub db_instance: Pin<Arc<T>>,
    pub variant: StorageVariant,
}
```

You may notice the use of generic type `Pin<Arc<T>>`. Explain more about why I wrote this. `Pin` trait will pin the object in a memory, which means it can't be move to a different location in a memory, for example, using `std::mem::swap`. Inside the `Pin` type, we have `Arc` or **Atomically Reference Counted**.`Arc` type shares ownership between threads, which is different from single threaded type `Rc`. Hence, this is usually used for handling multi-threaded operations and it is suitable for our distributed database design.

New method added to `RocksDBAdapter` to create a transaction

```rs
pub fn transaction(
        self: &'static Self,
        w: bool,
        r: bool,
    ) -> Result<Transaction<TxType>, Error> {
        let inner = self.get_inner();
        let db_instance = &inner.db_instance;
        let tx = db_instance.transaction();

        // The database reference must always outlive
        // the transaction. If it doesn't then this
        // is undefined behaviour. This unsafe block
        // ensures that the transaction reference is
        // static, but will cause a crash if the
        // datastore is dropped prematurely.
        let tx = unsafe {
            std::mem::transmute::<
                rocksdb::Transaction<'_, OptimisticTransactionDB>,
                rocksdb::Transaction<'static, OptimisticTransactionDB>,
            >(tx)
        };

        Ok(Transaction::<TxType>::new(tx, w, r))
    }
```

There is a head-aching concept in this method, we have an unsafe method use `std::mem::transmute`. This is not recommended to use as it transform the lifetime of an object. The reason why we use this method here is because, we need to cast the original lifetime of `OptimisticTransactionDB` to static as we want the transaction remains as long as it can until the program stops. This is referenced from the source code of **SurrealDB**.

On the other hand, we have an implementation for internal transaction

```rs
impl Transaction<TxType>
```

Any storage adapter can be understand as a bridge to create transaction, it does not store any transaction value. This separation provides the ability to control a single transaction each operation instead of a whole `db_instance`.

---

## Commits on Nov 2, 2022

### Description

-   Enhancing the development experience by configuring the CI/CD pipeline using Github Actions.
-   Implement basic methods of RocksDB OptimisticDB transaction

### Detail explanation

There are two workflows added:

-   Formatter (check + apply) workflow: Use `cargo clippy` and `cargo fmt` to format and lint the repo.
-   Test: Run `cargo test` on the workspace whenever there's an update to `master` branch

Every datastore transaction will be marked generically as DBTransaction or Distributed Database Transaction. This is implied by Solomon DB technical directory. Transaction will implement a trait that requires these below method

```rs
// Check if closed
fn closed(&self) -> bool;
// Cancel a transaction
fn cancel(&mut self) -> Result<(), Error>;
// Commit a transaction
fn commit(&mut self) -> Result<(), Error>;
// Check if a key exists
fn exi<K>(&mut self, key: K) -> Result<bool, Error>
where
K: Into<Key>;
// Fetch a key from the database
fn get<K>(&mut self, key: K) -> Result<Option<Val>, Error>;
// Insert or update a key in the database
fn set<K, V>(&mut self, key: K, val: V) -> Result<(), Error>;
// Insert a key if it doesn't exist in the database
fn put<K, V>(&mut self, key: K, val: V) -> Result<(), Error>;
// Delete a key
fn del<K>(&mut self, key: K) -> Result<(), Error>;
```

---

## Commits on Nov 4, 2022

### Description

Write a new macro to auto generate test code for datastore adapter. Whenever there's a new datastore implemented, we can add it to the test suite easily by

```rs
full_test_impl!(RocksDBAdapter::default());
```

This code implementation is referenced from [IndraDB](https://github.com/indradb/indradb/blob/master/lib/src/tests/macros.rs). On the other hand, these commits add a new feature tag called `test-suite` which must be declared to allow all test runs.

```diff
[features]
default = ["kv-rocksdb"]
kv-rocksdb = ["dep:rocksdb"]
+ test-suite = []
```

To run `cargo test` or `cargo nextest` enabling the `test-suite` feature, can follow these commands to run

```powershell
cargo test --features test-suite
```

```powershell
cargo nextest run --features test-suite
```

### Detail explanations

The logic behind the new macro is not too complicated, the macro `define_test!` receive any `datastore_adapter` as an input along with the name for the test. This name is also a name of methods exported from crate `tests`. This approach is required to overpass the type strictness of `DatastoreAdapter` as we will support multiple types of datastore adapter.

```rs
/// Defines a unit test function.
#[macro_export]
macro_rules! define_test {
	($name:ident, $datastore_adapter:expr) => {
		#[tokio::test]
		async fn $name() {
			let datastore_adapter = $datastore_adapter;
			$crate::tests::$name(datastore_adapter).await;
		}
	};
}

/// Use this macro to enable the entire standard test suite.
#[macro_export]
macro_rules! full_test_impl {
	($code:expr) => {
		#[cfg(test)]
		define_test!(should_delete_key, $code);
	};
}
```

---

## Commits on Nov 5, 2022

### Description

Introducing a new struct `DatastoreManager` which is generated using macro `impl_datastore_manager`. The idea behinds this implementation is to manage all datastore adapter in the cleanest way.

Adding models for graph related struct: `Node - Label - Property - Relationship`

### Detail explanation

Not sure but I remember this has been mentioned in a very first commit log, our graph database will follow the structure of `Property Graph`. There's a very concise explanation for this model in this repo for openCypher: https://github.com/opencypher/openCypher/blob/master/docs/property-graph-model.adoc

-   **Vertex**: Common object in every graph model. In Property Graph, vertex is more versatile. It has multiple properties, it can be considered as a document in NoSQL database. Vertex can also have multiple labels to identify itself.
-   **Relationship**: Or edge in single relational database. Indicates the link between two vertex. However, relationship can have properties too. It also has a type, for example, if two `LOVER` nodes connect, the relationship type should be `LOVE`. Defined as `source_vertex -> relationship -> target_vertex`.

-   **Property**: Define attribute type and name of object field. For example, vertex (or node) can have name, age, birthday and relationship can also have name. Structure of `property` is `uuid | name | type`. Property of each core objects (node and relationship) will be stored in a `HashMap<Uuid, Vec<u8>>` where `Uuid = Property ID` and `Vec<u8> = Byte value for that property`

-   **Label**: Vertex can have multiple labels. For example, one vertex can be a Person, Programmer and Employee at the same time. This can be misunderstood with `Property`. However, they are not the same. Labels are used for marking node instead of defining attributes.

---

## Commits on Nov 6, 2022

### Description

### Detail explanation
