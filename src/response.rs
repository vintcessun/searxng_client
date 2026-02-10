use std::collections::HashMap;

use chrono::NaiveDateTime;
use iso8601::Duration;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// The top-level response returned by the SearXNG API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The original query string.
    pub query: String,
    /// Estimated total number of results across all engines.
    pub number_of_results: i64,
    /// A list of search results.
    pub results: Vec<SearchResult>,
    /// Instant answers provided by specialized engines.
    pub answers: Vec<AnswerSet>,
    /// Possible query corrections.
    pub corrections: Vec<Correction>,
    /// Structured information boxes (Infoboxes).
    pub infoboxes: Vec<Infobox>,
    /// Search suggestions for related queries.
    pub suggestions: Vec<Suggestion>,
    /// A list of engines that failed to respond or returned errors.
    pub unresponsive_engines: Vec<EngineError>,
}

/// A search result entry.
///
/// SearXNG results are untagged enums that can represent either a modern `MainResult`
/// or an older `LegacyResult` format, ensuring compatibility across different SearXNG versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchResult {
    /// A result with legacy structure.
    LegacyResult(LegacySearchResult),
    /// A result with the modern main structure.
    MainResult(MainSearchResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MainSearchResult {
    // The Result Class
    // from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L228
    pub url: Option<String>,
    pub engine: Option<String>,
    pub parsed_url: Option<Vec<String>>,

    // The MainResult Class
    // from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L338
    pub template: String,
    pub title: String,
    pub content: String,
    pub img_src: String,
    pub iframe_src: String,
    pub audio_src: String,
    pub thumbnail: String,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<NaiveDateTime>,
    #[deprecated(
        since = "0.1.0",
        note = r#"According to the SearXNG codebase: "it is still partially used in the templates, but will one day be completely eliminated."(from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L372). So please use "published_date" instead"#
    )]
    // Considered to be deprecated since SearXNG's codebase has already marked it as deprecated, so although it is set `str` in the SearXNG codebase, it is set to `Option<String>` here for better compatibility with the future SearXNG versions.
    pub pubdate: Option<String>,
    pub length: Option<Duration>,
    pub views: String,
    pub author: String,
    pub metadata: String,
    pub priority: PriorityType,
    pub engines: SmallVec<[String; 4]>,
    pub open_group: bool,
    pub close_group: bool,
    pub positions: SmallVec<[i32; 4]>,
    pub score: f64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorityType {
    // from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L388
    #[serde(rename = "")]
    None,
    High,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySearchResult {
    // from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L427
    pub url: Option<String>,
    pub template: String,
    pub engine: String,
    pub parsed_url: Option<Vec<String>>,

    pub title: String,
    pub content: String,
    pub img_src: String,
    pub thumbnail: String,
    pub priority: PriorityType,
    pub engines: SmallVec<[String; 4]>,
    pub positions: SmallVec<[i32; 4]>,
    pub score: f64,
    pub category: String,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<NaiveDateTime>,
    pub pubdate: Option<String>,
}

/// A structured information box typically displayed on the side of search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infobox {
    pub infobox: String,
    pub id: String,
    /// The text content of the infobox.
    pub content: String,
    /// Associated URLs (e.g., Wikipedia links). Uses `serde_json::Value` for schema flexibility.
    pub urls: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Structured attributes. Uses `serde_json::Value` to handle dynamic key-value pairs.
    pub attributes: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// The name of the primary engine providing this infobox.
    pub engine: String,
    /// Primary URL for the entity.
    pub url: Option<String>,
    pub img_src: String,
    pub template: String,
    pub parsed_url: Option<Vec<String>>,
    pub title: String,
    pub thumbnail: String,
    pub priority: PriorityType,
    pub engines: SmallVec<[String; 4]>,
    pub positions: String,
    pub score: f64,
    pub category: String,
    /// The date this entry was published, if available.
    #[serde(rename = "publishedDate")]
    pub published_date: Option<NaiveDateTime>,
    /// Legacy publication date string.
    pub pubdate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    // The Result Class
    // from https://github.com/searxng/searxng/blob/master/searx/result_types/_base.py#L228
    pub url: Option<String>,
    pub engine: Option<String>,
    pub parsed_url: Option<Vec<String>>,
}

type BaseAnswer = Result;
type AnswerSet = Vec<BaseAnswer>;
type SetStr = String;
type Correction = SetStr;
type Suggestion = SetStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "(String, String)")]
pub struct EngineError {
    pub engine: String,
    pub error_msg: String,
}

impl From<(String, String)> for EngineError {
    fn from(tuple: (String, String)) -> Self {
        Self {
            engine: tuple.0,
            error_msg: tuple.1,
        }
    }
}
