mod bucket;
mod metric;
mod pipeline;

pub use self::bucket::*;
pub use self::metric::*;
pub use self::pipeline::*;
use crate::{impl_from_agg_for_aggregation, impl_from_agg_for_bucket_aggregation};
use derive_builder::Builder;
use derive_more::From;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct Aggregations {
    #[serde(flatten)]
    #[builder(setter(custom), default)]
    aggs: HashMap<String, Aggregation>,
}

impl Aggregations {
    pub fn builder() -> AggregationsBuilder {
        AggregationsBuilder::default()
    }
}

impl AggregationsBuilder {
    pub fn agg(&mut self, name: impl Into<String>, agg: impl Into<Aggregation>) -> &mut Self {
        self.aggs
            .get_or_insert_default()
            .insert(name.into(), agg.into());
        self
    }

    pub fn aggs<A: Into<Aggregation>>(&mut self, aggs: HashMap<String, A>) -> &mut Self {
        self.aggs = Some(aggs.into_iter().map(|(k, v)| (k, v.into())).collect());
        self
    }
}

impl<A, S, I> From<I> for Aggregations
where
    A: Into<Aggregation>,
    S: Into<String>,
    I: IntoIterator<Item = (S, A)>,
{
    fn from(iter: I) -> Aggregations {
        Aggregations {
            aggs: iter
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum Aggregation {
    Default(DefaultAggregation),
    Bucket(BucketAggregation),
    Nested(NestedAggregation),
    ReverseNested(ReverseNestedAggregation),
    Filter(FilterAggregation),
    Filters(FiltersAggregation),
    Sample(SampleAggregation),
}

impl Aggregation {
    pub fn nested() -> NestedAggregationBuilder {
        NestedAggregationBuilder::default()
    }

    pub fn sample() -> SampleAggregationBuilder {
        SampleAggregationBuilder::default()
    }

    pub fn reverse_nested() -> ReverseNestedAggregationBuilder {
        ReverseNestedAggregationBuilder::default()
    }

    // Metric aggregation builders
    pub fn avg() -> AvgAggregationBuilder {
        AvgAggregationBuilder::default()
    }

    pub fn max() -> MaxAggregationBuilder {
        MaxAggregationBuilder::default()
    }

    pub fn min() -> MinAggregationBuilder {
        MinAggregationBuilder::default()
    }

    pub fn sum() -> SumAggregationBuilder {
        SumAggregationBuilder::default()
    }

    pub fn value_count() -> ValueCountAggregationBuilder {
        ValueCountAggregationBuilder::default()
    }

    pub fn cardinality() -> CardinalityAggregationBuilder {
        CardinalityAggregationBuilder::default()
    }

    pub fn stats() -> StatsAggregationBuilder {
        StatsAggregationBuilder::default()
    }

    pub fn extended_stats() -> ExtendedStatsAggregationBuilder {
        ExtendedStatsAggregationBuilder::default()
    }

    pub fn percentiles() -> PercentilesAggregationBuilder {
        PercentilesAggregationBuilder::default()
    }

    pub fn percentile_ranks() -> PercentileRanksAggregationBuilder {
        PercentileRanksAggregationBuilder::default()
    }

    pub fn geo_bounds() -> GeoBoundsAggregationBuilder {
        GeoBoundsAggregationBuilder::default()
    }

    pub fn geo_centroid() -> GeoCentroidAggregationBuilder {
        GeoCentroidAggregationBuilder::default()
    }

    pub fn top_hits() -> TopHitsAggregationBuilder {
        TopHitsAggregationBuilder::default()
    }

    pub fn scripted_metric() -> ScriptedMetricAggregationBuilder {
        ScriptedMetricAggregationBuilder::default()
    }

    pub fn weighted_avg() -> WeightedAvgAggregationBuilder {
        WeightedAvgAggregationBuilder::default()
    }

    pub fn string_stats() -> StringStatsAggregationBuilder {
        StringStatsAggregationBuilder::default()
    }

    pub fn boxplot() -> BoxplotAggregationBuilder {
        BoxplotAggregationBuilder::default()
    }

    pub fn rate() -> RateAggregationBuilder {
        RateAggregationBuilder::default()
    }

    pub fn median_absolute_deviation() -> MedianAbsoluteDeviationAggregationBuilder {
        MedianAbsoluteDeviationAggregationBuilder::default()
    }

    pub fn matrix_stats() -> MatrixStatsAggregationBuilder {
        MatrixStatsAggregationBuilder::default()
    }

    // Bucket aggregation builders
    pub fn terms() -> TermsAggregationBuilder {
        TermsAggregationBuilder::default()
    }

    pub fn filter() -> FilterAggregationBuilder {
        FilterAggregationBuilder::default()
    }

    pub fn filters() -> FiltersAggregationBuilder {
        FiltersAggregationBuilder::default()
    }

    pub fn range() -> RangeAggregationBuilder {
        RangeAggregationBuilder::default()
    }

    pub fn date_range() -> DateRangeAggregationBuilder {
        DateRangeAggregationBuilder::default()
    }

    pub fn histogram() -> HistogramAggregationBuilder {
        HistogramAggregationBuilder::default()
    }

    pub fn date_histogram() -> DateHistogramAggregationBuilder {
        DateHistogramAggregationBuilder::default()
    }

    pub fn adjacency_matrix() -> AdjacencyMatrixAggregationBuilder {
        AdjacencyMatrixAggregationBuilder::default()
    }

    // Pipeline aggregation builders
    pub fn avg_bucket() -> AvgBucketAggregationBuilder {
        AvgBucketAggregationBuilder::default()
    }

    pub fn sum_bucket() -> SumBucketAggregationBuilder {
        SumBucketAggregationBuilder::default()
    }

    pub fn min_bucket() -> MinBucketAggregationBuilder {
        MinBucketAggregationBuilder::default()
    }

    pub fn max_bucket() -> MaxBucketAggregationBuilder {
        MaxBucketAggregationBuilder::default()
    }

    pub fn stats_bucket() -> StatsBucketAggregationBuilder {
        StatsBucketAggregationBuilder::default()
    }

    pub fn extended_stats_bucket() -> ExtendedStatsBucketAggregationBuilder {
        ExtendedStatsBucketAggregationBuilder::default()
    }

    pub fn percentiles_bucket() -> PercentilesBucketAggregationBuilder {
        PercentilesBucketAggregationBuilder::default()
    }

    pub fn derivative() -> DerivativeAggregationBuilder {
        DerivativeAggregationBuilder::default()
    }

    pub fn cumulative_sum() -> CumulativeSumAggregationBuilder {
        CumulativeSumAggregationBuilder::default()
    }

    pub fn moving_average() -> MovingAverageAggregationBuilder {
        MovingAverageAggregationBuilder::default()
    }

    pub fn serial_differencing() -> SerialDifferencingAggregationBuilder {
        SerialDifferencingAggregationBuilder::default()
    }

    pub fn bucket_script() -> BucketScriptAggregationBuilder {
        BucketScriptAggregationBuilder::default()
    }

    pub fn bucket_selector() -> BucketSelectorAggregationBuilder {
        BucketSelectorAggregationBuilder::default()
    }

    pub fn bucket_sort() -> BucketSortAggregationBuilder {
        BucketSortAggregationBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(rename_all = "snake_case")]
pub enum DefaultAggregation {
    // Metric Aggregations
    Avg(AvgAggregation),
    Max(MaxAggregation),
    Min(MinAggregation),
    Sum(SumAggregation),
    ValueCount(ValueCountAggregation),
    Cardinality(CardinalityAggregation),
    Stats(StatsAggregation),
    ExtendedStats(ExtendedStatsAggregation),
    Percentiles(PercentilesAggregation),
    PercentileRanks(PercentileRanksAggregation),
    GeoBounds(GeoBoundsAggregation),
    GeoCentroid(GeoCentroidAggregation),
    TopHits(TopHitsAggregation),
    ScriptedMetric(ScriptedMetricAggregation),
    WeightedAvg(WeightedAvgAggregation),
    StringStats(StringStatsAggregation),
    Boxplot(BoxplotAggregation),
    Rate(RateAggregation),
    MedianAbsoluteDeviation(MedianAbsoluteDeviationAggregation),
    MatrixStats(MatrixStatsAggregation),

    // Pipeline Aggregations
    AvgBucket(AvgBucketAggregation),
    SumBucket(SumBucketAggregation),
    MinBucket(MinBucketAggregation),
    MaxBucket(MaxBucketAggregation),
    StatsBucket(StatsBucketAggregation),
    ExtendedStatsBucket(ExtendedStatsBucketAggregation),
    PercentilesBucket(PercentilesBucketAggregation),
    Derivative(DerivativeAggregation),
    CumulativeSum(CumulativeSumAggregation),
    MovingAverage(MovingAverageAggregation),
    SerialDifferencing(SerialDifferencingAggregation),
    BucketScript(BucketScriptAggregation),
    BucketSelector(BucketSelectorAggregation),
    BucketSort(BucketSortAggregation),
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(rename_all = "snake_case")]
pub struct BucketAggregation {
    #[serde(flatten)]
    agg: BucketAggregationInner,
    aggs: Option<Aggregations>,
}

impl From<BucketAggregationInner> for BucketAggregation {
    fn from(mut agg: BucketAggregationInner) -> Self {
        Self {
            aggs: agg.take_aggs(),
            agg,
        }
    }
}

impl BucketAggregationInner {
    pub fn take_aggs(&mut self) -> Option<Aggregations> {
        match self {
            Self::Terms(t) => t.aggs.take(),
            Self::Range(r) => r.aggs.take(),
            Self::DateRange(dr) => dr.aggs.take(),
            Self::Histogram(h) => h.aggs.take(),
            Self::DateHistogram(dh) => dh.aggs.take(),
            Self::AdjacencyMatrix(am) => am.aggs.take(),
            Self::Sampler(s) => s.aggs.take(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(rename_all = "snake_case")]
pub enum BucketAggregationInner {
    Terms(TermsAggregation),
    Range(RangeAggregation),
    DateRange(DateRangeAggregation),
    Histogram(HistogramAggregation),
    DateHistogram(DateHistogramAggregation),
    AdjacencyMatrix(AdjacencyMatrixAggregation),
    Sampler(SampleAggregation),
}

impl BucketAggregation {
    pub fn take_aggs(&mut self) -> Option<Aggregations> {
        self.aggs.take()
    }
}

// Use macro to implement From trait for all aggregation types
impl_from_agg_for_aggregation!(
    // Metric Aggregations
    AvgAggregation,
    MaxAggregation,
    MinAggregation,
    SumAggregation,
    ValueCountAggregation,
    CardinalityAggregation,
    StatsAggregation,
    ExtendedStatsAggregation,
    PercentilesAggregation,
    PercentileRanksAggregation,
    GeoBoundsAggregation,
    GeoCentroidAggregation,
    TopHitsAggregation,
    ScriptedMetricAggregation,
    WeightedAvgAggregation,
    StringStatsAggregation,
    BoxplotAggregation,
    RateAggregation,
    MedianAbsoluteDeviationAggregation,
    MatrixStatsAggregation,
    // Pipeline Aggregations
    AvgBucketAggregation,
    SumBucketAggregation,
    MinBucketAggregation,
    MaxBucketAggregation,
    StatsBucketAggregation,
    ExtendedStatsBucketAggregation,
    PercentilesBucketAggregation,
    DerivativeAggregation,
    CumulativeSumAggregation,
    MovingAverageAggregation,
    SerialDifferencingAggregation,
    BucketScriptAggregation,
    BucketSelectorAggregation,
    BucketSortAggregation
);

impl_from_agg_for_bucket_aggregation!(
    TermsAggregation,
    RangeAggregation,
    DateRangeAggregation,
    HistogramAggregation,
    DateHistogramAggregation,
    AdjacencyMatrixAggregation
);
