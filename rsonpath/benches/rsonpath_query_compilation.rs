use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use decimal_byte_measurement::DecimalByteMeasurement;
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;

type CriterionCtx = Criterion<DecimalByteMeasurement>;

fn rsonpath_query_compilation(c: &mut CriterionCtx, query_string: &str) {
    let mut group = c.benchmark_group(format! {"rsonpath_{}", query_string});

    group.bench_with_input(
        BenchmarkId::new("compile_query", query_string),
        query_string,
        |b, q| {
            b.iter(|| {
                let query = JsonPathQuery::parse(q).unwrap();
                black_box(StacklessRunner::compile_query(&query));
            })
        },
    );

    group.finish();
}

fn decimal_byte_measurement() -> CriterionCtx {
    Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

pub fn descendant_only(c: &mut CriterionCtx) {
    rsonpath_query_compilation(c, "$..claims..references..hash");
}

pub fn small(c: &mut CriterionCtx) {
    rsonpath_query_compilation(c, "$..en.value");
}

pub fn child_only(c: &mut CriterionCtx) {
    rsonpath_query_compilation(c, "$.user.entities.description.urls");
}

pub fn paper_query(c: &mut CriterionCtx) {
    rsonpath_query_compilation(c, "$..x..a.b.a.b.c..y.a");
}

pub fn many_components(c: &mut CriterionCtx) {
    rsonpath_query_compilation(
        c,
        "$..a.a.b.b.a.b.a.a.b.b.a.a.b.a.b.b.a..b.a.b.a.a.b.a.b.a.a.b.a.a.b..c.a.b.c.d.e.f.g.h.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z..d.d.d.d.d.d.d.d.d.d.d.d.d.d.d..e.a.a.a.a.b.b.b.b.c.c.c.c.d.d.d.d.e.e.e.e",
    )
}

criterion_group!(
    name = query_benches;
    config = decimal_byte_measurement();
    targets = descendant_only,
    small,
    child_only,
    paper_query,
    many_components
);

criterion_main!(query_benches);
