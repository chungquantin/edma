# Solomon DB Wiki

Solomon DB is a graph database built for community.

## Chapter 1: Learning concepts

The book that I read to build this database is Graph Databases from O'Reilly. In this chapter, concepts of the graph databases will be covered and used as materials for later implementation. Need to cultivate knowledge before jumping into coding first.

### Components of the graph database

#### Underlying storage

Storage layer is the key component of any database as we need it to store and query data. In terms of graph database, there are two types of storage: native and non-native. Native storage is built specifically for graph-related operations and it's fit with the schema design of graph database.

On the other hand, non-native storage can be any other relational or non-relational database storage. This approach reduces a lot of time for implementation and non-native storage is a proof of experiments. All minor incremental improvements are covered in the development of the storage, like MySQL or Cassandra which has been used by top companies in the field.

#### Processing engine

Some definitions require that a graph database use **index-free adjacency**, meaning
that connected nodes physically “point” to each other in the database.

Here we take a slightly broader view: any database that from the user’s perspective behaves like a graph database (i.e., exposes a graph data model through CRUD operations) qualifies as a graph database.

=> In conclusion, native storage graph database has a well-built engineered and performant storage while native storage depends on the mature of non-graph backend.

### Graph Compute Engine

A graph compute engine is a technology that enables global graph computational algorithms to be run against large datasets. Graph compute engines are designed to do
things like identify clusters in your data, or answer questions such as, “how many relationships, on average, does everyone in a social network have?”

The graph computing process can be handled offline, there will be an ETL (Extract - Transform - Load) pipeline that handle the stage of moving data from a system records to the offline graph compute engine.

### Graph Query Language

Each database has its own language interface provided to user to simplify the database operations like querying and mutating data. Some databases are successful just by diversifying variants of libraries and interfaces: From CLI to SDK to GUI. For SQL database, a very common approach is query language SQL. Relatively, Neo4j graph database also has its own query language: **Cypher**.

To construct a successful query language is not easy. It requires a lot of deep knowledge of fundamental concepts of programming languages like lexical parser, abstract syntax tree, tokenizer...

The parsed tokens will be handled by a query logical planner which transform those tokens to relational algebra and execute based on the instructions.

### Brain storming database design & architecture

After a few days of researching and reading **O'Reilly Graph Database** book (it was a very good book for graph database introduction actually), I decided to stumble into experimenting.

It was quite hard to identify the uniqueness of Solomon DB. The main reason is there are too many databases already. From production-ready databases like **Cassandra, PostgreSQL** or a common graph database like **Neo4j** and **JanusGraph**. Even though my desire for being able to design an outstanding architecture is quite high, I position my current skills not capable of that. Hence, instead of trying to over complicate everything, I think starting small steps and climb up stair by stair might be a better strategy.

I got inspired by the design of SurrealDB which does not build its own underlying layer but trying to maximize the power of multiple databases. And I think it is a good design to try. So the main architecture of SolomonDB will be:

#### Underlying storage

There are two NoSQL databases that fascinate me due to its design and scalability.

-   **RocksDB:** The popular key-value database designed and maintained by team at Facebook. It is used widely by several big databases as an underlying storage. The use of RocksDB will be elaborated more in a later chapter.
-   **Cassandra:** For anyone who works with big data, is quite familiar with this wide-column database. Cassandra is a big guy in the field with its distributed architecture and several other features for streaming and configuring node instances.

#### Property graph data structure & algorithms

The concepts of property graph data structure is used commonly in production system which is **read contention** and graph driven. A quite well-known examples of property graph are social network.

Solomon DB will use property graph data structure to present the graph data in the system. The main problems of this data structure is applying algorithms designed for single relational graph will be a bit complicated.

#### Modelling graph data

Facebook's TAO graph database has a very well-explained white paper that demonstrates the approach to design Property Graph. Core components of Solomon graph model will be **Node, Relationship, Property and Label**.
