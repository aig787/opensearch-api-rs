//! Aggregation response types for OpenSearch

mod bucket;
mod metric;
mod pipeline;

pub use self::bucket::*;
pub use self::metric::*;
pub use self::pipeline::*;
use enum_as_inner::EnumAsInner;

/// Represents different types of aggregation responses from OpenSearch
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, EnumAsInner)]
#[serde(untagged)]
pub enum AggregationResponse {
    // /// Unknown aggregation response type - most general, must be last
    // Unknown(serde_json::Value),
    /// Top Hits aggregation response - has unique fields like _index, _id, _score
    TopHits(TopHitsAggregationResponse),

    /// Matrix Stats aggregation response - has fields/correlations/covariances structure
    MatrixStats(MatrixStatsAggregationResponse),

    /// String Stats aggregation response - has count/min_length/max_length/avg_length/distribution
    StringStats(StringStatsAggregationResponse),

    /// Stats aggregation response - has count/min/max/avg/sum fields
    /// May also include extended stats fields (sum_of_squares, variance, std_deviation, std_deviation_bounds)
    Stats(StatsAggregationResponse),

    /// Boxplot aggregation response - has min/max/q1/q2/q3 non-optional fields
    Boxplot(BoxplotAggregationResponse),

    /// Composite aggregation response - has 'buckets' and 'after_key'
    Composite(CompositeAggregationResponse),

    /// Significant terms aggregation response - has 'buckets' with bg_count/score
    SignificantTerms(SignificantTermsAggregationResponse),

    /// Filters aggregation response - has 'buckets' with named keys
    Filters(FiltersAggregationResponse),

    /// Terms aggregation response - has 'buckets' array with doc_count/key combinations
    Terms(TermsAggregationResponse),

    /// Range aggregation response - has 'buckets' with from/to
    Range(RangeAggregationResponse),

    /// IP range aggregation response - has 'buckets' with IP format strings
    IPRange(IPRangeAggregationResponse),

    /// Date range aggregation response - has 'buckets' with from/to/from_as_string/to_as_string
    DateRange(DateRangeAggregationResponse),

    /// Histogram aggregation response - has 'buckets' with numeric keys
    Histogram(HistogramAggregationResponse),

    /// Geo distance aggregation response - has 'buckets' with range info
    GeoDistance(GeoDistanceAggregationResponse),

    /// Geo grid aggregation response - has 'buckets' with geohash/geocentroid
    GeoGrid(GeoGridAggregationResponse),

    /// Adjacency matrix aggregation response - has 'buckets' with specific key structure
    AdjacencyMatrix(AdjacencyMatrixAggregationResponse),

    /// Geo Bounds aggregation response - has top_left/bottom_right coordinates
    GeoBounds(GeoBoundsAggregationResponse),

    /// Geo Centroid aggregation response - has location data structure
    GeoCentroid(GeoCentroidAggregationResponse),

    Bucket(BucketAggregationResponse),

    Percentile(PercentileAggregationResponse),

    NumericInt {
        value: i64,
    },

    NumericFloat {
        value: f64,
    },

    NumericString {
        value_string: String,
    },
    // Unknown aggregation response type - most general, must be last
    Unknown(serde_json::Value),
}
