use rusqlite::{Connection, LoadExtensionGuard, Result, params};
use std::env;

pub struct MoneyDatabase {
    pub conn: Connection,
    pub tbl_name: String,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoneyError {
    #[error("Balance Insufficient: need {needmore} more")]
    BalanceInsufficient { needmore: u32 },

    #[error("Database Error: {0}")]
    Database(#[from] rusqlite::Error),
}

#[doc = "This macro formats a query string to have the GID since SQLite cant have table names be parameters"]
macro_rules! tblfmt {
    ($self:expr, $query:literal) => {
        format!($query, $self.tbl_name).as_str()
    };
}
pub(crate) use tblfmt;

impl MoneyDatabase {
    pub fn open(gid: u64) -> Result<Self, MoneyError> {
        let home = env::var("HOME").expect("HOME Environemnt Variable must be set");
        let conn = Connection::open(format!("{home}/.moneybot/data.db"))?;
        unsafe {
            let _guard = LoadExtensionGuard::new(&conn);
            conn.load_extension_enable()?;
            conn.load_extension("external/sqlite-vec/dist/vec0", None::<&str>)?;
            conn.load_extension_disable()?;
        }
        let table_name = format!("guild_{gid}");
        let db = MoneyDatabase {
            conn,
            tbl_name: table_name,
        };
        db.conn.execute(
            tblfmt!(
                db,
                "CREATE TABLE IF NOT EXISTS {} (
                uid INTEGER PRIMARY KEY,
                balance INTEGER NOT NULL,
                last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
                items VECTOR
            )"
            ),
            [],
        )?;

        db.conn.execute(
            &format!(
                "CREATE TRIGGER IF NOT EXISTS update_{}_timestamp
                            AFTER UPDATE ON {}
                            FOR EACH ROW
                            BEGIN
                                UPDATE {} SET last_modified = CURRENT_TIMESTAMP WHERE uid = NEW.uid;
                            END",
                db.tbl_name, db.tbl_name, db.tbl_name
            ),
            [],
        )?;
        Ok(db)
    }

    pub fn add_money(&self, uid: u64, amount: u32) -> Result<(), MoneyError> {
        self.conn.execute(
            tblfmt!(
                self,
                "INSERT OR IGNORE INTO {} (uid, balance) VALUES (?1, ?2)"
            ),
            params![uid, 0],
        )?;

        let balance = self.get_balance(uid)?;

        self.conn.execute(
            tblfmt!(self, "UPDATE {} SET balance = ?1 WHERE uid = ?2"),
            params![amount + balance, uid],
        )?;

        Ok(())
    }

    pub fn get_balance(&self, uid: u64) -> Result<u32, MoneyError> {
        let mut stmt = self
            .conn
            .prepare(tblfmt!(self, "SELECT uid, balance FROM {} WHERE uid = ?1"))?;
        let mut rows = stmt.query(params![uid])?;
        if let Some(row) = rows.next()? {
            let balance = row.get(1)?;
            Ok(balance)
        } else {
            Ok(0)
        }
    }

    pub fn delete_money(&self, uid: u64, amount: u32) -> Result<(), MoneyError> {
        let balance = self.get_balance(uid)?;
        if balance < amount {
            return Err(MoneyError::BalanceInsufficient {
                needmore: (amount - balance),
            });
        } else {
            self.conn.execute(
                tblfmt!(self, "UPDATE {} SET balance = ?1 WHERE uid = ?2"),
                params![balance - amount, uid],
            )?;

            Ok(())
        }
    }
}
