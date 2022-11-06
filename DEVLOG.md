# SDL - Solomon Development Log

## _What is it about?_

A place to record the progress of building Solomon DB. This can be tracked as a commit changelog with detailed explanation.

## _What is it changed?_

### Oct 26, 2022

#### Description

A soft beginning. I just spend some time to read graph database book and learning concepts, requirements needed to construct Solomon DB.

#### Commits

-   [Init layout](https://github.com/nomadiz/solomon-db/commit/502aefdca1f54f9650d80d844b7af9f5c1f362af)

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

### Oct 28, 2022

#### Description

Add `StorageDriver`, new type `RocksDBDriver`, macro `impl_driver_core` and misc implementation for `StorageErr` and `Status`.

#### Commits

-   [Add impl_driver_core macro and rocksdb driver](https://github.com/nomadiz/solomon-db/commit/e3b3ed75c813e38bb523c070500eb88d5a774ac1)

#### Detail explanation

Things getting more exciting when we touch advanced syntaxes in Rust: [`New Type`](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) design pattern and [`Macro`](https://doc.rust-lang.org/book/ch19-06-macros.html). One of a missing features in Rust compared to other OOP language like Java or C++ is the ability to do inheritance.

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
