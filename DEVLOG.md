# SDL - Solomon Development Log

## _What is it about?_

A place to record the progress of building Solomon DB. This can be tracked as a commit changelog with detailed explanation.

**Note**: `Commits` section won't include commit related to updating DEVLOG or CHANGELOG.

## _What is it changed?_

### Commits on Oct 26, 2022

#### Description

A soft beginning. I just spend some time to read graph database book and learning concepts, requirements needed to construct Solomon DB.

#### Commits

-   [Init layout](https://github.com/nomadiz/solomon-db/commit/502aefdca1f54f9650d80d844b7af9f5c1f362af) at Wed Oct 26 17:17:39 2022 +0700 by <tin@snowflake.so>

#### Detail explanation

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

### Commits on Oct 28, 2022

#### Description

Add `StorageDriver`, new type `RocksDBDriver`, macro `impl_driver_core` and misc implementation for `StorageErr` and `Status`.

#### Commits

-   [Add impl_driver_core macro and rocksdb driver](https://github.com/nomadiz/solomon-db/commit/e3b3ed75c813e38bb523c070500eb88d5a774ac1) at Fri Oct 28 19:14:43 2022 +0700 by <cqtin0903@gmail.com>

#### Detail explanation

Things getting more exciting when we touch advanced syntaxes in Rust: [`New Type`](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) design pattern and [`Macro`](https://doc.rust-lang.org/book/ch19-06-macros.html). Before jumping right into the main point of today changes, one of a missing features in Rust compared to other OOP languages must be mentioned first is the ability to do **inheritance**.

With what Rust provides so far, it is quite tricky to construct an inheritable `struct` object. At first, you may think doing OOP in Rust is quite bad as inheritance is a core component of any OOP design. However, if you learn more about OOP, you will know `Inheritance < Composition`. This is quite off topic, but there is a few discussions and articles related to the way Rust do OOP that you should read:

-   [Rust Internals | Discussion - Why not inheritance?](https://internals.rust-lang.org/t/why-not-inheritance/5738/14)
-   [Where Rust enum shines?](http://smallcultfollowing.com/babysteps/blog/2015/05/05/where-rusts-enum-shines/)

Briefly, even though Rust does not have an obvious OOP feature like inheritance, it still provides several approaches: `Trait`, `New type`, `Macro` or `Enum` to do thing in a inheritance way but cleaner as things are separated elegantly.

##### New Type design pattern for RocksDB Driver

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

### Commits on Nov 1, 2022

#### Description

The prior implementation of `RocksDBDriver` was using the database instance initialized using `DB` object. And all methods are called from the object `DB` instance too. However, this approach has some limitations in terms of concurrency operations. Briefly, I migrate it from using `DB` to handling operations using transactions - a better control flow approach. Instead of using `TransactionDB`, I choose `OptimisticTransactionDB` which is more efficient for read contention database, correct me if I am wrong.

You can read more about this in [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki/Transactions#optimistictransactiondb)

#### Commits

-   [Add optimistic db and basic methods](https://github.com/nomadiz/solomon-db/commit/a37a51e99dd54c6244bcf178e0b71e31334ee599) at Tue Nov 1 12:58:55 2022 +0700 by <cqtin0903@gmail.com>

#### Detail explanation

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

### Commits on Nov 2, 2022

#### Description

#### Commits

-   [Add CI/CD pipeline and format codebase](https://github.com/nomadiz/solomon-db/commit/ed9e5784e629de0db7cb3cc1b41e9f3852dbc621) at Wed Nov 2 09:29:45 2022 +0700 by <cqtin0903@gmail.com>
-   [Resolve workspace issue](https://github.com/nomadiz/solomon-db/commit/fb7d6e9062302cf5a69a3a01eeebfc0094aeb21e) at Wed Nov 2 12:13:38 2022 +0700 by <cqtin0903@gmail.com>
-   [Add optimistic rocksdb adapter](https://github.com/nomadiz/solomon-db/commit/5a63826e1f9104e31949a7645f03f63be3a3575b) at Wed Nov 2 18:45:07 2022 +0700 by <cqtin0903@gmail.com>
-   [Update github action workflow](https://github.com/nomadiz/solomon-db/commit/ae3e0e0784f82c30786f73010a5edc45a70bb833) at Wed Nov 2 23:17:09 2022 +0700 by <cqtin0903@gmail.com>
-   [Update README.md](https://github.com/nomadiz/solomon-db/commit/aaf2e9fbd89c9770c82fb9e6ec57e65fe382234c) at Wed Nov 2 23:29:19 2022 +0700 by <56880684+chungquantin@users.noreply.github.com>
-   [Update README.md](https://github.com/nomadiz/solomon-db/commit/088d77a3ad269431782edb763d8c776f893016a2) at Wed Nov 2 23:37:35 2022 +0700 by <56880684+chungquantin@users.noreply.github.com>

#### Detail explanation

---

### Commits on Nov 4, 2022

#### Description

#### Commits

-   [Generate test suite and refractor codebase](https://github.com/nomadiz/solomon-db/commit/5d4d68c2891822c70d3bfb3b5052bcb499cb7e76) at Fri Nov 4 20:53:44 2022 +0700 by <cqtin0903@gmail.com>
-   [Add dependency random to test-suite feature](https://github.com/nomadiz/solomon-db/commit/4f7269db220f4d3b6c8a40e29382c35ea6332b38) at Fri Nov 4 20:54:12 2022 +0700 by <cqtin0903@gmail.com>
-   [Fix compile error & change test command](https://github.com/nomadiz/solomon-db/commit/d0687e8808f90b9c095cb999b1e9afc865af7bbe) at Fri Nov 4 21:11:47 2022 +0700 by <cqtin0903@gmail.com>

#### Detail explanations

---

### Commits on Nov 5, 2022

#### Description

#### Commits

-   [Update generate path to temp directory](https://github.com/nomadiz/solomon-db/commit/f1b9cbb2c364365e2cd64c2584a0c8d739197900) at Sat Nov 5 00:08:45 2022 +0700 by <cqtin0903@gmail.com>
-   [Add column family to rocksdb datastore](https://github.com/nomadiz/solomon-db/commit/e11a8a6dc35f758cc48049ff1ada8e1f8bbfb810) at Sat Nov 5 10:25:00 2022 +0700 by <cqtin0903@gmail.com>
-   [Add basic structure for graph model + macro for datastore manager](https://github.com/nomadiz/solomon-db/commit/3edb7b4260cd13c483edfe1211e298359ab0b948) at Sat Nov 5 19:27:22 2022 +0700 by <cqtin0903@gmail.com>

#### Detail explanation

---

### Commits on Nov 6, 2022

#### Description

#### Commits

#### Detail explanation
