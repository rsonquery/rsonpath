use arbitrary::{Arbitrary, Unstructured};
use rsonpath_syntax::builder::{
    EmptyComparisonExprBuilder, EmptyLogicalExprBuilder, JsonPathQueryBuilder, JsonPathSelectorsBuilder,
    LogicalExprBuilder, SingularJsonPathQueryBuilder, SliceBuilder,
};
use rsonpath_syntax::num::{JsonFloat, JsonInt, JsonNumber, JsonUInt};
use rsonpath_syntax::prelude::JsonString;
use rsonpath_syntax::{ComparisonExpr, JsonPathQuery, Literal};

struct SafeUnstructured<'a, 'b> {
    u: &'b mut Unstructured<'a>,
    error: Option<arbitrary::Error>,
}

impl<'a, 'b> SafeUnstructured<'a, 'b> {
    fn new(u: &'b mut Unstructured<'a>) -> Self {
        Self { u, error: None }
    }

    fn err_or<T>(self, t: T) -> arbitrary::Result<T> {
        if let Some(err) = self.error {
            Err(err)
        } else {
            Ok(t)
        }
    }

    fn access<F, T: Default>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Unstructured<'a>) -> arbitrary::Result<T>,
    {
        match f(self.u) {
            Ok(x) => x,
            Err(err) => {
                self.error = Some(err);
                T::default()
            }
        }
    }
}

#[derive(Debug)]
pub struct ArbitrarySupportedQuery(pub JsonPathQuery);

#[derive(Debug, Arbitrary)]
enum SupportedSegment {
    Child(SupportedSelector),
    Descendant(SupportedSelector),
}

#[derive(Debug, Arbitrary)]
enum SupportedSelector {
    Name(ArbitraryJsonString),
    Wildcard,
    Index(ArbitraryJsonUInt),
}

impl<'a> Arbitrary<'a> for ArbitrarySupportedQuery {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let segment_count = u.arbitrary_len::<SupportedSegment>()?;
        let mut query = JsonPathQueryBuilder::new();

        for _ in 0..segment_count {
            let segment = u.arbitrary::<SupportedSegment>()?;
            match segment {
                SupportedSegment::Child(SupportedSelector::Name(name)) => query.child_name(name.0),
                SupportedSegment::Child(SupportedSelector::Wildcard) => query.child_wildcard(),
                SupportedSegment::Child(SupportedSelector::Index(idx)) => query.child_index(idx.0),
                SupportedSegment::Descendant(SupportedSelector::Name(name)) => query.descendant_name(name.0),
                SupportedSegment::Descendant(SupportedSelector::Wildcard) => query.descendant_wildcard(),
                SupportedSegment::Descendant(SupportedSelector::Index(idx)) => query.descendant_index(idx.0),
            };
        }

        Ok(ArbitrarySupportedQuery(query.into()))
    }
}

#[derive(Debug)]
pub struct ArbitraryJsonPathQuery(pub JsonPathQuery);
#[derive(Debug)]
pub struct ArbitraryJsonString(pub JsonString);
#[derive(Debug)]
pub struct ArbitraryJsonUInt(pub JsonUInt);

impl<'a> arbitrary::Arbitrary<'a> for ArbitraryJsonPathQuery {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut builder = JsonPathQueryBuilder::new();
        let mut u = SafeUnstructured::new(u);
        ArbitraryQueryGenerator::generate_query(&mut builder, 1, &mut u);
        Ok(Self(builder.into_query()))
    }
}

impl<'a> arbitrary::Arbitrary<'a> for ArbitraryJsonString {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut u = SafeUnstructured::new(u);
        let inner = generate_string(&mut u);
        u.err_or(Self(inner))
    }
}

impl<'a> arbitrary::Arbitrary<'a> for ArbitraryJsonUInt {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut u = SafeUnstructured::new(u);
        let inner = generate_json_uint(&mut u);
        u.err_or(Self(inner))
    }
}

struct ArbitraryQueryGenerator {
    depth: usize,
}

impl ArbitraryQueryGenerator {
    const MAX_QUERY_DEPTH: usize = 3;
    const MAX_LOGICAL_EXPR_SIZE: usize = 5;
    const MAX_SEGMENTS: usize = 5;
    const MAX_SELECTORS: usize = 3;

    fn generate_query<'a>(
        builder: &'a mut JsonPathQueryBuilder,
        depth: usize,
        u: &mut SafeUnstructured,
    ) -> &'a mut JsonPathQueryBuilder {
        let mut this = ArbitraryQueryGenerator { depth };

        let len = u.access(|u| u.int_in_range(0..=ArbitraryQueryGenerator::MAX_SEGMENTS));
        for _ in 0..len {
            this.generate_segment(builder, u);
        }
        builder
    }

    fn generate_segment(&mut self, builder: &mut JsonPathQueryBuilder, u: &mut SafeUnstructured) {
        let discriminant = u.access(|u| u.int_in_range(0..=1));
        if discriminant == 0 {
            builder.child(|b| self.generate_selectors(b, u));
        } else {
            builder.descendant(|b| self.generate_selectors(b, u));
        }
    }

    fn generate_selectors<'a>(
        &self,
        builder: &'a mut JsonPathSelectorsBuilder,
        u: &mut SafeUnstructured,
    ) -> &'a mut JsonPathSelectorsBuilder {
        let len = u.access(|u| u.int_in_range(1..=Self::MAX_SELECTORS));
        for _ in 0..len {
            self.generate_selector(builder, u);
        }
        builder
    }

    fn generate_selector(&self, builder: &mut JsonPathSelectorsBuilder, u: &mut SafeUnstructured) {
        let discriminant = u.access(|u| u.int_in_range(0..=4));
        match discriminant {
            0 => Self::generate_name_selector(builder, u),
            1 => Self::generate_index_selector(builder, u),
            2 => Self::generate_slice_selector(builder, u),
            3 => Self::generate_wildcard_selector(builder, u),
            4 => self.generate_filter_selector(builder, u),
            _ => unreachable!(),
        }
    }

    fn generate_name_selector(builder: &mut JsonPathSelectorsBuilder, u: &mut SafeUnstructured) {
        let string = generate_string(u);
        builder.name(string);
    }

    fn generate_index_selector(builder: &mut JsonPathSelectorsBuilder, u: &mut SafeUnstructured) {
        let index = generate_json_int(u);
        builder.index(index);
    }

    fn generate_slice_selector(builder: &mut JsonPathSelectorsBuilder, u: &mut SafeUnstructured) {
        builder.slice(|b| generate_slice(b, u));
    }

    fn generate_wildcard_selector(builder: &mut JsonPathSelectorsBuilder, _u: &mut SafeUnstructured) {
        builder.wildcard();
    }

    fn generate_filter_selector(&self, builder: &mut JsonPathSelectorsBuilder, u: &mut SafeUnstructured) {
        builder.filter(|b| self.generate_logical_expr(b, u));
    }

    fn generate_logical_expr(&self, b: EmptyLogicalExprBuilder, u: &mut SafeUnstructured) -> LogicalExprBuilder {
        let target_depth = u.access(|u| u.int_in_range(1..=Self::MAX_LOGICAL_EXPR_SIZE));
        return generate_with_depth(self, b, u, target_depth);

        fn generate_with_depth(
            this: &ArbitraryQueryGenerator,
            builder: EmptyLogicalExprBuilder,
            u: &mut SafeUnstructured,
            target_depth: usize,
        ) -> LogicalExprBuilder {
            let discriminant = if this.depth < ArbitraryQueryGenerator::MAX_QUERY_DEPTH {
                u.access(|u| u.int_in_range(0..=2))
            } else {
                0
            };
            let builder = match discriminant {
                0 => builder.comparison(|b| this.generate_comparison(b, u)),
                1 => builder.test_absolute(|b| this.generate_nested_query(b, u)),
                2 => builder.test_relative(|b| this.generate_nested_query(b, u)),
                _ => unreachable!(),
            };
            if target_depth == 1 {
                builder
            } else {
                let discriminant = u.access(|u| u.int_in_range(0..=2));
                match discriminant {
                    0 => EmptyLogicalExprBuilder.not(|_| builder),
                    1 => builder.and(|b| generate_with_depth(this, b, u, target_depth - 1)),
                    2 => builder.or(|b| generate_with_depth(this, b, u, target_depth - 1)),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn generate_comparison(&self, builder: EmptyComparisonExprBuilder, u: &mut SafeUnstructured) -> ComparisonExpr {
        let discriminant = u.access(|u| u.int_in_range(0..=2));
        let lhs = match discriminant {
            0 => {
                let literal = generate_literal(u);
                builder.literal(literal)
            }
            1 => builder.query_absolute(|b| self.generate_singular_query(b, u)),
            2 => builder.query_relative(|b| self.generate_singular_query(b, u)),
            _ => unreachable!(),
        };
        let discriminant = u.access(|u| u.int_in_range(0..=5));
        let op = match discriminant {
            0 => lhs.equal_to(),
            1 => lhs.greater_or_equal_to(),
            2 => lhs.greater_than(),
            3 => lhs.less_than(),
            4 => lhs.lesser_or_equal_to(),
            5 => lhs.not_equal_to(),
            _ => unreachable!(),
        };
        let discriminant = u.access(|u| u.int_in_range(0..=2));
        let result = match discriminant {
            0 => {
                let literal = generate_literal(u);
                op.literal(literal)
            }
            1 => op.query_absolute(|b| self.generate_singular_query(b, u)),
            2 => op.query_relative(|b| self.generate_singular_query(b, u)),
            _ => unreachable!(),
        };
        result
    }

    fn generate_singular_query<'a>(
        &self,
        builder: &'a mut SingularJsonPathQueryBuilder,
        u: &mut SafeUnstructured,
    ) -> &'a mut SingularJsonPathQueryBuilder {
        let len = u.access(|u| u.int_in_range(0..=Self::MAX_SEGMENTS));
        for _ in 0..len {
            let discriminant = u.access(|u| u.int_in_range(0..=1));
            match discriminant {
                0 => builder.index(generate_json_int(u)),
                1 => builder.name(generate_string(u)),
                _ => unreachable!(),
            };
        }
        builder
    }

    fn generate_nested_query<'a>(
        &self,
        builder: &'a mut JsonPathQueryBuilder,
        u: &mut SafeUnstructured,
    ) -> &'a mut JsonPathQueryBuilder {
        Self::generate_query(builder, self.depth + 1, u)
    }
}

fn generate_literal(u: &mut SafeUnstructured) -> Literal {
    let discriminant = u.access(|u| u.int_in_range(0..=3));
    match discriminant {
        0 => Literal::Null,
        1 => Literal::Bool(u.access(|u| u.arbitrary::<bool>())),
        2 => Literal::Number(generate_number(u)),
        3 => Literal::String(generate_string(u)),
        _ => unreachable!(),
    }
}

fn generate_number(u: &mut SafeUnstructured) -> JsonNumber {
    let discriminant = u.access(|u| u.int_in_range(0..=1));
    match discriminant {
        0 => JsonNumber::from(generate_json_int(u)),
        1 => JsonNumber::from(generate_json_float(u)),
        _ => unreachable!(),
    }
}

fn generate_json_int(u: &mut SafeUnstructured) -> JsonInt {
    u.access(|u| {
        let val = u.int_in_range(JsonInt::MIN.as_i64()..=JsonInt::MAX.as_i64())?;
        let int = JsonInt::try_from(val).expect("int is in range above and should succeed");
        Ok(int)
    })
}

fn generate_json_uint(u: &mut SafeUnstructured) -> JsonUInt {
    u.access(|u| {
        let val = u.int_in_range(0..=JsonUInt::MAX.as_u64())?;
        let int = JsonUInt::try_from(val).expect("uint is in range above and should succeed");
        Ok(int)
    })
}

fn generate_json_float(u: &mut SafeUnstructured) -> JsonFloat {
    struct SafeFloat(JsonFloat);
    impl Default for SafeFloat {
        fn default() -> Self {
            SafeFloat(JsonFloat::ZERO)
        }
    }

    u.access(|u| {
        let val = u.arbitrary::<f64>()?;
        // Wrap NaN, +Inf, -Inf into zero.
        let val = if val.is_nan() {
            0.0
        } else if val.is_infinite() {
            0.0_f64.copysign(val)
        } else {
            val
        };

        Ok(SafeFloat(
            JsonFloat::try_from(val).expect("the above construction should always give correct values"),
        ))
    })
    .0
}

fn generate_string(u: &mut SafeUnstructured) -> JsonString {
    struct SafeString(JsonString);
    impl Default for SafeString {
        fn default() -> Self {
            SafeString(JsonString::new(""))
        }
    }

    u.access(|u| {
        let iter = u.arbitrary_iter::<char>()?.map(|x| x.unwrap_or_default());
        Ok(SafeString(JsonString::from_iter(iter)))
    })
    .0
}

fn generate_slice<'a>(builder: &'a mut SliceBuilder, u: &mut SafeUnstructured) -> &'a mut SliceBuilder {
    let (has_start, has_end, has_step) = u.access(|u| Ok((u.arbitrary()?, u.arbitrary()?, u.arbitrary()?)));
    if has_start {
        let int = generate_json_int(u);
        builder.with_start(int);
    }
    if has_end {
        let int = generate_json_int(u);
        builder.with_end(int);
    }
    if has_step {
        let int = generate_json_int(u);
        builder.with_step(int);
    }
    builder
}
