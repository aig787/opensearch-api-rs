/// Macro to implement conversion from specific aggregation types to the Aggregation enum
#[macro_export]
macro_rules! impl_from_agg_for_aggregation {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Aggregation {
                fn from(t: $t) -> Aggregation {
                    DefaultAggregation::from(t).into()
                }
            }
        )*
    };
}

/// Macro to implement conversion from specific bucket aggregation types to the Aggregation enum
#[macro_export]
macro_rules! impl_from_agg_for_bucket_aggregation {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Aggregation {
                fn from(t: $t) -> Aggregation {
                    BucketAggregation::from(BucketAggregationInner::from(t)).into()
                }
            }
        )*
    };
}

/// Macro to implement conversion from specific query types to Query and Box<Query> enums
#[macro_export]
macro_rules! impl_from_query_type {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Box<Query> {
                fn from(t: $t) -> Box<Query> {
                    Box::new(Query::from(t))
                }
            }
        )*
    };
}