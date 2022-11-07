use crate::DBTransaction;

pub type DBType = String;
pub type TxType = String;
pub type CassandraDBTransaction = DBTransaction<DBType, TxType>;
