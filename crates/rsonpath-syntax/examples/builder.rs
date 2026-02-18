use rsonpath_syntax::{
    builder::JsonPathQueryBuilder,
    num::{JsonFloat, JsonInt},
    str::JsonString,
    Slice,
};
use std::{error::Error, process::ExitCode};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    // === Build each example from the JSONPath spec. ===

    // $.store.book[*].author
    let query = JsonPathQueryBuilder::new()
        .child_name("store")
        .child_name("book")
        .child_wildcard()
        .child_name("author")
        .to_query();
    println!("{query}");

    // $..author
    let query = JsonPathQueryBuilder::new().descendant_name("author").to_query();
    println!("{query}");

    // $.store.*
    let query = JsonPathQueryBuilder::new()
        .child_name("store")
        .child_wildcard()
        .to_query();
    println!("{query}");

    // $.store..price
    let query = JsonPathQueryBuilder::new()
        .child_name("store")
        .descendant_name("price")
        .to_query();
    println!("{query}");

    // $..book[2]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child_index(2)
        .to_query();
    println!("{query}");

    // $..book[2].author
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child_index(2)
        .child_name("author")
        .to_query();
    println!("{query}");

    // $..book[2].publisher
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child_index(2)
        .child_name("publisher")
        .to_query();
    println!("{query}");

    // $..book[-1]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child_index(-1)
        .to_query();
    println!("{query}");

    // $..book[0, 1]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child(|x| x.index(0).index(1))
        .to_query();
    println!("{query}");

    // $..book[:2]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child_slice(Slice::build().with_end(2))
        .to_query();
    println!("{query}");

    // $..book[?@.isbn]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child(|x| x.filter(|x| x.test_relative(|x| x.child_name("isbn"))))
        .to_query();
    println!("{query}");

    // $..book[?@.price<10]
    let query = JsonPathQueryBuilder::new()
        .descendant_name("book")
        .child(|x| {
            x.filter(|x| {
                x.comparison(|x| {
                    x.query_relative(|x| x.name("price"))
                        .less_than()
                        .literal(JsonInt::from(10))
                })
            })
        })
        .to_query();
    println!("{query}");

    // $..*
    let query = JsonPathQueryBuilder::new().descendant_wildcard().to_query();
    println!("{query}");
    println!();

    // === Examples from the filter selector section: https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#name-examples-6

    // $.a[?@.b == 'kilo']
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| {
            x.comparison(|x| {
                x.query_relative(|q| q.name("b"))
                    .equal_to()
                    .literal(JsonString::from("kilo"))
            })
        })
        .to_query();
    println!("{query}");

    // $.a[?@>3.5]
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| {
            x.comparison(|x| {
                x.query_relative(|q| q)
                    .greater_than()
                    .literal(JsonFloat::try_from(3.5).unwrap())
            })
        })
        .to_query();
    println!("{query}");

    // $.a[?@.b]
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| x.test_relative(|x| x.child_name("b")))
        .to_query();
    println!("{query}");

    // $[?@.*]
    let query = JsonPathQueryBuilder::new()
        .child_filter(|x| x.test_relative(|x| x.child_wildcard()))
        .to_query();
    println!("{query}");

    // $[?@[?@.b]]
    let query = JsonPathQueryBuilder::new()
        .child_filter(|x| x.test_relative(|x| x.child_filter(|x| x.test_relative(|x| x.child_name("b")))))
        .to_query();
    println!("{query}");

    // $.o[?@<3, ?@<3]
    let query = JsonPathQueryBuilder::new()
        .child_name("o")
        .child(|x| {
            x.filter(|x| x.comparison(|x| x.query_relative(|x| x).less_than().literal(JsonInt::from(3))))
                .filter(|x| x.comparison(|x| x.query_relative(|x| x).less_than().literal(JsonInt::from(3))))
        })
        .to_query();
    println!("{query}");

    // $.a[?@<2 || @.b == "k"]
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| {
            x.comparison(|x| x.query_relative(|x| x).less_than().literal(JsonInt::from(2)))
                .or(|x| {
                    x.comparison(|x| {
                        x.query_relative(|x| x.name("b"))
                            .equal_to()
                            .literal(JsonString::from("k"))
                    })
                })
        })
        .to_query();
    println!("{query}");

    // $.o[?@>1 && @<4]
    let query = JsonPathQueryBuilder::new()
        .child_name("o")
        .child_filter(|x| {
            x.comparison(|x| x.query_relative(|x| x).greater_than().literal(JsonInt::from(1)))
                .and(|x| x.comparison(|x| x.query_relative(|x| x).less_than().literal(JsonInt::from(4))))
        })
        .to_query();
    println!("{query}");

    // $.o[?@.u || @.x]
    let query = JsonPathQueryBuilder::new()
        .child_name("o")
        .child_filter(|x| {
            x.test_relative(|x| x.child_name("u"))
                .or(|x| x.test_relative(|x| x.child_name("x")))
        })
        .to_query();
    println!("{query}");

    // $.a[?@.b == $.x]
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| {
            x.comparison(|x| {
                x.query_relative(|x| x.name("b"))
                    .equal_to()
                    .query_absolute(|x| x.name("x"))
            })
        })
        .to_query();
    println!("{query}");

    // $.a[?@ == @]
    let query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|x| x.comparison(|x| x.query_relative(|x| x).equal_to().query_absolute(|x| x)))
        .to_query();
    println!("{query}");

    println!();

    // === Build a query showcasing all syntactic elements. ===
    // $.['store\t1']
    //  ..[3, -5, ::3, ::-5, :7:2, 3::2, 3:7, 3:7:2, -3:-7:-2]
    //  [*]
    //  [? @['name'] == "Best" && @['book'][?@.price > 30.3] ||
    //     $['global'] != false && (@['year'] < 2023 || $[0].year >= 2000) ||
    //     !($..['invalid'])]
    let query = JsonPathQueryBuilder::new()
        .child_name("store\t1")
        .descendant(|x| {
            x.index(3)
                .index(-5)
                .slice(Slice::build().with_step(3))
                .slice(Slice::build().with_step(-5))
                .slice(Slice::build().with_end(7).with_step(2))
                .slice(Slice::build().with_start(3).with_step(2))
                .slice(Slice::build().with_start(3).with_end(7))
                .slice(Slice::build().with_start(3).with_end(7).with_step(2))
                .slice(Slice::build().with_start(-3).with_end(-7).with_step(-2))
        })
        .child_wildcard()
        .child_filter(|f| {
            f.comparison(|c| c.query_relative(|q| q.name("name")).equal_to().literal("Best"))
                .and(|f| {
                    f.test_relative(|t| {
                        t.child_name("book").child_filter(|f2| {
                            f2.comparison(|c| {
                                c.query_relative(|q| q.name("price"))
                                    .greater_than()
                                    .literal(JsonFloat::try_from(30.3).unwrap())
                            })
                        })
                    })
                })
                .or(|f| {
                    f.comparison(|c| c.query_absolute(|q| q.name("global")).not_equal_to().literal(false))
                        .and(|f| {
                            f.comparison(|c| {
                                c.query_relative(|q| q.name("year"))
                                    .less_than()
                                    .literal(JsonInt::from(2023))
                            })
                            .or(|f| {
                                f.comparison(|c| {
                                    c.query_absolute(|q| q.index(0).name("year"))
                                        .greater_or_equal_to()
                                        .literal(JsonInt::from(2000))
                                })
                            })
                        })
                })
                .or(|f| f.not(|f| f.test_absolute(|q| q.descendant_name("invalid"))))
        })
        .to_query();

    println!("{query}");

    Ok(ExitCode::SUCCESS)
}
