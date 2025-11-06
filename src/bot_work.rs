use rand::Rng;
use crate::botdb;

pub fn bot_work(gid: u64, uid: u64) -> u32 {
    let mut rng = rand::rng();
    let randint: u32 = rng.random_range(1..=1000);
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Err(why) = db.add_money(uid, randint) {
            println!("Error adding money: {why:?}");
            return 0;
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
    randint
}