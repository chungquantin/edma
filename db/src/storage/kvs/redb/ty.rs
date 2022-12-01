use crate::DBTransaction;

pub type DBType = redb::Database;
pub type TxType = redb::WriteTransaction<'static>;
pub type ReDBTransaction = DBTransaction<DBType, TxType>;
