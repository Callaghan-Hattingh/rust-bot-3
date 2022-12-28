mod db;
mod migrator;
use futures::executor::block_on;

fn main() {
    println!("Start!");

    if let Err(err) = block_on(db::db_migration()) {
        panic!("{}", err);
    }

    println!("End!")
}
