use rand::Rng;
use crate::botdb;

pub fn bot_gamble(gid: u64, uid: u64, amount: u32) -> String {
    let mut rng = rand::rng();
    let mut avg: u64 = 0;
    for _ in 1..=10 {
        avg += rng.random_range(1..=10);
    }
    avg /= 10;

    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Err(err) = db.delete_money(uid, amount) {
            return err.to_string();
        }
        if avg > 5 {
            if let Ok(()) = db.add_money(uid, amount * 2) {
                return format!("You won {}", amount * 2).to_string();
            } else {
                return "There was an error".to_string();
            }
        } else {
            return format!("You lost {}", amount);
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
}