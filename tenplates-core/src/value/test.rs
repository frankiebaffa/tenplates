use {
    crate::{
        context::{ Alias, Context },
        value::Value,
    },
    rusqlite::Connection,
};

fn db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn
        .prepare(r#"
            create table person (
                id integer not null,
                name text not null,
                nickname text null,
                primary key (id)
            );
        "#)
        .unwrap();
    stmt.execute(()).unwrap();

    drop(stmt);

    let mut stmt = conn
        .prepare(r#"
            insert into person (
                id,
                name,
                nickname
            )
            values
                (
                    1,
                    'Frankie',
                    'Shrek'
                ),
                (
                    2,
                    'Matthew',
                    null
                );
        "#)
        .unwrap();

    stmt.execute(()).unwrap();

    drop(stmt);

    conn
}

#[test]
fn query_1() {
    let conn = db();
    let mut stmt = conn.prepare("select nickname from person where id = 1").unwrap();
    let ctx = Context::default();
    let result = ctx.query(&mut stmt).unwrap();
    let value_opt = result.get_column("NICKNAME");
    assert!(value_opt.is_some());

    let value = value_opt.unwrap();
    assert_eq!(&Value::from("Shrek".to_owned()), value);
}

#[test]
fn query_w_param() {
    let conn = db();
    let mut ctx = Context::default();
    ctx.insert(Alias::from_str("id").unwrap(), 1_i64).unwrap();
    let mut stmt = conn.prepare("select nickname from person where id = :id").unwrap();
    let result = ctx.query(&mut stmt).unwrap();
    let value_opt = result.get_column("NICKNAME");
    assert!(value_opt.is_some());

    let value = value_opt.unwrap();
    assert_eq!(&Value::from("Shrek".to_owned()), value);
}

#[test]
fn integer_equality_1() {
    assert!(Value::from(1_i64) == Value::from(1_i64));
}

#[test]
fn integer_equality_2() {
    assert!(Value::from(2_i64) != Value::from(1_i64));
}

#[test]
fn integer_equality_3() {
    assert!(!(Value::from(2_i64) == Value::from(1_i64)));
}

#[test]
fn integer_equality_4() {
    assert!(!(Value::from(1_i64) != Value::from(1_i64)));
}

#[test]
fn real_equality_1() {
    assert!(Value::from(1.0_f64) == Value::from(1.00_f64));
}

#[test]
fn real_equality_2() {
    assert!(Value::from(1.01_f64) != Value::from(1.0_f64));
}

#[test]
fn real_equality_3() {
    assert!(!(Value::from(1.242_f64) == Value::from(1.24_f64)));
}

#[test]
fn real_equality_4() {
    assert!(!(Value::from(6_f64) != Value::from(6.00_f64)));
}

#[test]
fn text_equality_1() {
    assert!(Value::from("Hello") == Value::from("Hello"));
}

#[test]
fn text_equality_2() {
    assert!(Value::from("Hello") != Value::from("World"));
}

#[test]
fn text_equality_3() {
    assert!(!(Value::from("World") == Value::from("Hello")));
}

#[test]
fn text_equality_4() {
    assert!(!(Value::from("Hello") != Value::from("Hello")));
}

#[test]
fn blob_equality_1() {
    assert!(Value::from(vec![ 0x00, 0x01, 0x02 ]) == Value::from(vec![ 0x00, 0x01, 0x02 ]));
}

#[test]
fn blob_equality_2() {
    assert!(Value::from(vec![ 0x00, 0x01, 0x02 ]) != Value::from(vec![ 0x02, 0x01, 0x00 ]));
}

#[test]
fn blob_equality_3() {
    assert!(!(Value::from(vec![ 0x00, 0x01, 0x02 ]) == Value::from(vec![ 0x02, 0x01, 0x00 ])));
}

#[test]
fn blob_equality_4() {
    assert!(!(Value::from(vec![ 0x00, 0x01, 0x02 ]) != Value::from(vec![ 0x00, 0x01, 0x02 ])));
}
