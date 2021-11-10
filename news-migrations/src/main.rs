extern crate postgres;

use postgres::tls::NoTls;
use postgres::Client;

mod migrations;
use migrations::*;

fn main() {
    let mut client =
        Client::connect("postgres://postgres:docker@127.0.0.1:5432/postgres", NoTls).unwrap();

    let mut migrations = vec![CreateTableNewsMigration::new().run(&mut client)];
    migrations.push(AddNewsRecordsMigration::new().run(&mut client));

    for result in migrations.iter() {
        match result {
            Ok(changes) => println!("Migration Success: {}", changes),
            Err(error) => println!("Migration Failure: {}", error),
        };
    }
}
