#[macro_use]
extern crate criterion;

use display_as::{with_template, DisplayAs, HTML, format_as};

pub fn displayas_big_table(b: &mut criterion::Bencher, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    let ctx = BigTable { table };
    b.iter(|| format_as!(HTML, ctx));
}

struct BigTable {
    table: Vec<Vec<usize>>,
}

#[with_template("[%" "%]" "big-table-displayas.html")]
impl DisplayAs<HTML> for BigTable {}

pub fn displayas_teams(b: &mut criterion::Bencher, _: &usize) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };
    b.iter(|| format_as!(HTML, teams));
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

#[with_template("[%" "%]" "teams-displayas.html")]
impl DisplayAs<HTML> for Teams {}

struct Team {
    name: String,
    score: u8,
}

use criterion::{Criterion, Fun};

fn big_table(c: &mut Criterion) {
    c.bench_functions(
        "Big table",
        vec![
            Fun::new("DisplayAs", displayas_big_table),
        ],
        100,
    );
}

fn teams(c: &mut Criterion) {
    c.bench_functions(
        "Teams",
        vec![
            Fun::new("DisplayAs", displayas_teams),
        ],
        0,
    );
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
