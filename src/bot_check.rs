use crate::botdb;

pub fn bot_check(gid: u64, uid: u64) -> u32 {
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Ok(money) = db.get_balance(uid) {
            return money;
        } else {
            println!("Error getting balance");
            return 0;
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
}