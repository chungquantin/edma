# CHANGELOG | November 2022

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

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

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

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

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

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 5, 2022

### Description

Introducing a new struct `DatastoreManager` which is generated using macro `impl_datastore_manager`. The idea behinds this implementation is to manage all datastore adapter in the cleanest way.

-   Adding method to generate a random datastore path

```diff
/// Generate a path to store data for RocksDB
fn generate_path(id: Option<i32>) -> String {
	 let random_id: i32 = generate_random_i32();
	 let id = &id.unwrap_or(random_id).to_string();
- String::from("./.temp/rocks-") + id + ".db"
+ let path = if cfg!(target_os = "linux") {
+	 "/dev/shm/".into()
+ } else {
+	 temp_dir()
+ }
+ .join(format!("solomon-rocksdb-{}", id));
+
+	path_to_string(&path).unwrap()
}
```

-   Adding column family to RocksDB

Adding models for graph related struct: `Node - Label - Property - Relationship`

### Detail explanation

#### [RocksDB Column Family](https://github.com/EighteenZi/rocksdb_wiki/blob/master/Column-Families.md)

As being stated in the RocksDB Wiki:

> Each key-value pair in RocksDB is associated with exactly one Column Family.

In SolomonDB design, there will be multiple column families specified for different set of data. For example: **vertex, vertex-property, edge, edge-property and** **label**. All methods of RocksDB transaction will include a column family attribute. For example, the `get` method

```diff
/// Fetch a key from the database
- async fn get<K: Into<Key> + Send>(&mut self, key: K) -> Result<Option<Val>, Error>;
+ async fn get<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<Option<Val>, Error>;
```

#### [Property Graph Design](https://github.com/opencypher/openCypher/blob/master/docs/property-graph-model.adoc)

Not sure but I remember this has been mentioned in a very first commit log, our graph database will follow the structure of `Property Graph`.

-   **Vertex**: Common object in every graph model. In Property Graph, vertex is more versatile. It has multiple properties, it can be considered as a document in NoSQL database. Vertex can also have multiple labels to identify itself.
-   **Relationship**: Or edge in single relational database. Indicates the link between two vertex. However, relationship can have properties too. It also has a type, for example, if two `LOVER` nodes connect, the relationship type should be `LOVE`. Defined as `source_vertex -> relationship -> target_vertex`.

-   **Property**: Define attribute type and name of object field. For example, vertex (or node) can have name, age, birthday and relationship can also have name. Structure of `property` is `uuid | name | type`. Property of each core objects (node and relationship) will be stored in a `HashMap<Uuid, Vec<u8>>` where `Uuid = Property ID` and `Vec<u8> = Byte value for that property`

-   **Label**: Vertex can have multiple labels. For example, one vertex can be a Person, Programmer and Employee at the same time. This can be misunderstood with `Property`. However, they are not the same. Labels are used for marking node instead of defining attributes.

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 6, 2022

### Description

-   Write DEVLOG for 2022 October CHANGLOG and November 1st commits

### Detail explanation

In facts, I did try to add a Github Actions workflows to auto generate Github CHANGELOG. However, it did not work as expected so I just decide to write CHANGELOG on my own.

```yaml
name: Changelog
on:
    release:
        types:
            - created
jobs:
    changelog:
        runs-on: ubuntu-20.04
        steps:
            - name: "✏️ Generate release changelog"
              uses: heinrichreimer/github-changelog-generator-action@v2.3
              with:
                  token: ${{ secrets.GITHUB_TOKEN }}
```

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 7, 2022

### Description

-   Updating Rust compiler version to support new feature in Rust `1.65.0`: `Generic Associated Type`.
-   Adding `CassandraDB`
-   Write two new methods for `Vertex` controller

### Detail explanation

Generic Associated Type is not new and it is common in Rust `nightly` channel. However, it was only officially released to `stable` channel in the latest version. Based on the given definition:

> Generic associated types are an extension that permit associated types to have generic parameters. This allows associated types to capture types that may include generic parameters that came from a trait method. These can be lifetime or type parameters.

Here is how GAT used in SolomonDB code. We no longer have to care about `Generic Type` passed between structs. On the other hand, `GAT Transaction` force all implemented `Transaction` type to be SimpleTransaction.

```diff
#[async_trait]
- pub trait DatastoreAdapter<T: SimpleTransaction> {
+ pub trait DatastoreAdapter {
+   type Transaction: SimpleTransaction;
	// # Create new database transaction
	// Set `rw` default to false means readable but not readable
- fn transaction(&self, rw: bool) -> Result<T, Error>;
+ fn transaction(&self, rw: bool) -> Result<Self::Transaction, Error>;

	fn default() -> Self;
```

With GAT, we can make a fully reusable struct `DatastoreAdapter` and get rid of declaring `Transaction` generic type like `RocksDBTransaction`. The implementation of `DatastoreAdapter` instance for `RocksDB` will be

```diff
#[async_trait]
- impl DatastoreAdapter<RocksDBTransaction> for RocksDBAdapter {
+ impl DatastoreAdapter for RocksDBAdapter {
+ type Transaction = RocksDBTransaction;

	fn default() -> Self {
		let path = &RocksDBAdapter::generate_path(None);
		RocksDBAdapter::new(path, None).unwrap()
```

Super clean right? Worth a try if you have not updated yet.

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 8, 2022

### Description

-   Adding macro to avoid duplicate of controller implementation
-   Update `static` column family names
-   Implement basic methods for `LabelController` and `VertexController`. These methods allow us to mutate and query data from `vertex` database.

### Detail explanation

Add more column family name to the global variable

```rs
pub const CF_NAMES: [&str; 4] = ["test_suite:v1", "vertices:v1", "labels:v1", "edges:v1"];
```

How `LabelController` and `VertexController` implemented using the macro

```rs
impl_controller!(get RocksDBAdapter; from rocks_db for LabelController);
impl_controller!(get RocksDBAdapter; from rocks_db for VertexController);
```

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 10, 2022

### Description

-   Adding more methods to controllers
-   Refractor `impl_controller` macro to make it cleaner and more versatile
-   Add a property controller `PropertyController` to manage properties
-   Add feature for `CassandraDB`: `kv-cassandradb`
-   New method `multi_get` for database transaction
-   Implement a `GlobalConfig`

### Detail explanation

A new approach to implement a controller. Now we can identify the controller identifier and its column family.

```rs
impl_controller!(LabelController("labels:v1"));
impl_controller!(VertexController("vertices:v1"));
impl_controller!(PropertyController("properties:v1"));
```

New method added for database transaction to retrieve multiple data at once

```rs
async fn multi_get<K>(&self, cf: CF, keys: Vec<K>) -> Result<Vec<Option<Val>>, Error>
	where
		K: Into<Key> + Send + AsRef<[u8]>,
```

The point of `GlobalConfig` is that we don't want controller use different databases for each of them. To make it in sync, there will be a struct `GlobalConfig` storing a datastore where other controllers can reference from.

```rs
macro_rules! impl_global_config {
	($datastore: ident) => {
		pub struct GlobalConfig {
			pub ds: $datastore,
		}

		impl GlobalConfig {
			pub fn new() -> Result<Self, Error> {
				Ok(GlobalConfig {
					ds: $datastore::default(),
				})
			}
		}
	};
}
```

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 11, 2022

### Description

Apply some techniques to serialize Rust struct data into bytes data. Derived from my experience with Solana development, I learnt the approach Solana codebase uses to for byte serialization and deserialization. This can be briefly described with a work **Discriminator**.

### Detail explanation

The concept of **Account Discriminator** is common in Solana development using Anchor framework ([Link to reference](https://github.com/coral-xyz/anchor/issues/10)).

We will take these lines of code below as an example. This code is to serialize `Label` struct into `Vec<u8>` and store it in our data storage.

```rs
// Handle byte concatenate for label components
let _label_discriminator = serialize_discriminator(AccountDiscriminator::Label).unwrap();
let _labels = build_bytes(&label_components).unwrap();
let _label_offset = &build_offset(ll, uuid_len);
let (l_dis, l_offset, l) =
(_label_discriminator.as_slice(), _label_offset.as_slice(), _labels.as_slice());
let labels_concat = [l_dis, l_offset, l].concat();
```

Understanding the code, there will be three components of the bytes vector `Label: (account discriminator / offset / object byte data)`

-   `Account Discriminator`: Identifier of a struct object. Below code indicates the mapped index for each object. In `Label` case, the mapped index is `2`

```rs
#[derive(PartialEq, Serialize, Deserialize, Eq, Debug, Clone)]
pub enum AccountDiscriminator {
	None = 0,
	Vertex = 1,
	Label = 2,
	Property = 3,
	Relationship = 4,
}
```

-   `Offset`: Or it can be called `meta`, it describes the number of byte indices `object data` will occupy. For example, a string `hello = [68 65 6c 6c 6f]` will have an offset as `5` as there are 5 byte elements in the vector.

> (NOTE: The reason why we must store metadata of the object byte data is because there might be a chance the stored data is a vector of data)

-   `Object byte data`: This is obvious, the byte data of the object data. Taking the above string example, the byte data for String object `hello` is `hello = [68 65 6c 6c 6f]`.

Combining all the byte component together, we get something like

```rs
/// The return data of this example might not be exact
/// but it is the idea
hello = [ 2 5 68 65 6c 6c 6f ]
```

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 12, 2022

### Description

Add a workflow script to automatically build and deploy a doc page to Github Pages.

### Detail explanation

Rust ecosystem has a very interesting crate called [mdbook](https://rust-lang.github.io/mdBook/) that helps developer to construct a documentation page in one click. Command to install a cargo crate

```
cargo install mdbook
```

Documentation book of SolomonDB is stored in `dpc/src`. The page is hosted on [SolomonDB Documentation Page](https://nomadiz.github.io/solomon-db/devlog/2022/november-2022.html)

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 13, 2022

### Description

Add two badges to markdown. Just a visual stuff.

### Detail explanation

Here is the two badges added to the `README.md` file

<p align="center">
    <a href="https://github.com/nomadiz/solomon-db/graphs/contributors" alt="Contributors">
        <img src="https://img.shields.io/github/contributors/nomadiz/solomon-db" /></a>
    <a href="https://github.com/nomadiz/solomon-db/pulse" alt="Activity">
        <img src="https://img.shields.io/github/commit-activity/m/nomadiz/solomon-db" /></a>
</p>

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 15, 2022

### Description

### Detail explanation

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 17, 2022

### Description

### Detail explanation

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))

---

## Commits on Nov 18, 2022

### Description

### Detail explanation

### Contributors

-   Chung Quan Tin ([@chungquantin](https://github.com/chungquantin))
