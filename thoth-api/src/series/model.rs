use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::ThothError;
use crate::graphql::utils::Direction;
#[cfg(feature = "backend")]
use crate::schema::series;
#[cfg(feature = "backend")]
use crate::schema::series_history;

#[cfg_attr(feature = "backend", derive(DbEnum, juniper::GraphQLEnum))]
#[cfg_attr(feature = "backend", DieselType = "Series_type")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeriesType {
    Journal,
    #[cfg_attr(feature = "backend", db_rename = "book-series")]
    BookSeries,
}

#[cfg_attr(
    feature = "backend",
    derive(juniper::GraphQLEnum),
    graphql(description = "Field to use when sorting series list")
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeriesField {
    #[serde(rename = "SERIES_ID")]
    SeriesID,
    SeriesType,
    SeriesName,
    #[serde(rename = "ISSNPRINT")]
    ISSNPrint,
    #[serde(rename = "ISSNDIGITAL")]
    ISSNDigital,
    #[serde(rename = "SERIES_URL")]
    SeriesURL,
    CreatedAt,
    UpdatedAt,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
#[derive(Serialize, Deserialize)]
pub struct Series {
    pub series_id: Uuid,
    pub series_type: SeriesType,
    pub series_name: String,
    pub issn_print: String,
    pub issn_digital: String,
    pub series_url: Option<String>,
    pub imprint_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg_attr(
    feature = "backend",
    derive(juniper::GraphQLInputObject, Insertable),
    table_name = "series"
)]
pub struct NewSeries {
    pub series_type: SeriesType,
    pub series_name: String,
    pub issn_print: String,
    pub issn_digital: String,
    pub series_url: Option<String>,
    pub imprint_id: Uuid,
}

#[cfg_attr(
    feature = "backend",
    derive(juniper::GraphQLInputObject, AsChangeset),
    changeset_options(treat_none_as_null = "true"),
    table_name = "series"
)]
pub struct PatchSeries {
    pub series_id: Uuid,
    pub series_type: SeriesType,
    pub series_name: String,
    pub issn_print: String,
    pub issn_digital: String,
    pub series_url: Option<String>,
    pub imprint_id: Uuid,
}

#[cfg_attr(feature = "backend", derive(Queryable))]
pub struct SeriesHistory {
    pub series_history_id: Uuid,
    pub series_id: Uuid,
    pub account_id: Uuid,
    pub data: serde_json::Value,
    pub timestamp: NaiveDateTime,
}

#[cfg_attr(feature = "backend", derive(Insertable), table_name = "series_history")]
pub struct NewSeriesHistory {
    pub series_id: Uuid,
    pub account_id: Uuid,
    pub data: serde_json::Value,
}

#[cfg_attr(
    feature = "backend",
    derive(juniper::GraphQLInputObject),
    graphql(description = "Field and order to use when sorting seriess list")
)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesOrderBy {
    pub field: SeriesField,
    pub direction: Direction,
}

impl Default for SeriesType {
    fn default() -> SeriesType {
        SeriesType::BookSeries
    }
}

impl Default for SeriesField {
    fn default() -> Self {
        SeriesField::SeriesName
    }
}

impl fmt::Display for SeriesType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SeriesType::Journal => write!(f, "Journal"),
            SeriesType::BookSeries => write!(f, "Book Series"),
        }
    }
}

impl FromStr for SeriesType {
    type Err = ThothError;

    fn from_str(input: &str) -> Result<SeriesType, ThothError> {
        match input {
            "Journal" => Ok(SeriesType::Journal),
            "Book Series" => Ok(SeriesType::BookSeries),
            _ => Err(ThothError::InvalidSeriesType(input.to_string())),
        }
    }
}

impl FromStr for SeriesField {
    type Err = ThothError;

    fn from_str(input: &str) -> Result<SeriesField, ThothError> {
        match input {
            // Only match the headers which are currently defined/sortable in the UI
            "ID" => Ok(SeriesField::SeriesID),
            "Series" => Ok(SeriesField::SeriesName),
            "SeriesType" => Ok(SeriesField::SeriesType),
            "ISSNPrint" => Ok(SeriesField::ISSNPrint),
            "ISSNDigital" => Ok(SeriesField::ISSNDigital),
            "Updated" => Ok(SeriesField::UpdatedAt),
            _ => Err(ThothError::SortFieldError(
                input.to_string(),
                "Series".to_string(),
            )),
        }
    }
}

#[test]
fn test_seriestype_default() {
    let seriestype: SeriesType = Default::default();
    assert_eq!(seriestype, SeriesType::BookSeries);
}

#[test]
fn test_seriesfield_default() {
    let seriesfield: SeriesField = Default::default();
    assert_eq!(seriesfield, SeriesField::SeriesName);
}

#[test]
fn test_seriestype_display() {
    assert_eq!(format!("{}", SeriesType::Journal), "Journal");
    assert_eq!(format!("{}", SeriesType::BookSeries), "Book Series");
}

#[test]
fn test_seriestype_fromstr() {
    assert_eq!(
        SeriesType::from_str("Journal").unwrap(),
        SeriesType::Journal
    );
    assert_eq!(
        SeriesType::from_str("Book Series").unwrap(),
        SeriesType::BookSeries
    );

    assert!(SeriesType::from_str("bookseries").is_err());
    assert!(SeriesType::from_str("Collection").is_err());
}

#[test]
fn test_seriesfield_fromstr() {
    assert_eq!(SeriesField::from_str("ID").unwrap(), SeriesField::SeriesID);
    assert_eq!(
        SeriesField::from_str("Series").unwrap(),
        SeriesField::SeriesName
    );
    assert_eq!(
        SeriesField::from_str("SeriesType").unwrap(),
        SeriesField::SeriesType
    );
    assert_eq!(
        SeriesField::from_str("ISSNPrint").unwrap(),
        SeriesField::ISSNPrint
    );
    assert_eq!(
        SeriesField::from_str("ISSNDigital").unwrap(),
        SeriesField::ISSNDigital
    );
    assert_eq!(
        SeriesField::from_str("Updated").unwrap(),
        SeriesField::UpdatedAt
    );
    assert!(SeriesField::from_str("URL").is_err());
    assert!(SeriesField::from_str("Created").is_err());
}
