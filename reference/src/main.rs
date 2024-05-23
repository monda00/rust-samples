use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn show_with_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

struct S<'a> {
    r: &'a i32
}

struct T {
    r: &'static i32
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    show_with_ref(&table);
    show_with_ref(&table);

    show(table);
    //show(table);

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // x += 10; // error: cannot assign to `x` because it is borrowed
    // let m = &mut x; // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("{} {}", r1, r2);

    let mut y = 20;
    let m1 = &mut y;
    // let m2 = &mut y;
    // let z = y;
    println!("{}", m1);

    let s = S { r: &x };
    println!("{}", s.r);

    static u: i32 = 42;
    let t = T { r: &u };
    println!("{}", t.r);
}
