use crate::DBTransaction;

pub type DBType = String; // Unimplemented
pub type TxType = String; // Unimplemented
pub type CassandraDBTransaction = DBTransaction<DBType, TxType>;
