use serde::Deserialize;
use serde::Serialize;
use thoth_api::model::series::SeriesWithImprint;
use uuid::Uuid;

pub const SERIES_QUERY: &str = "
    query SeriesQuery($seriesId: Uuid!) {
        series(seriesId: $seriesId) {
            seriesId
            seriesType
            seriesName
            issnPrint
            issnDigital
            seriesUrl
            updatedAt
            imprint {
                imprintId
                imprintName
                updatedAt
                publisher {
                    publisherId
                    publisherName
                    publisherShortname
                    publisherUrl
                    createdAt
                    updatedAt
                }
            }
        }
    }
";

graphql_query_builder! {
    SeriesRequest,
    SeriesRequestBody,
    Variables,
    SERIES_QUERY,
    SeriesResponseBody,
    SeriesResponseData,
    FetchSeries,
    FetchActionSeries
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub series_id: Option<Uuid>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SeriesResponseData {
    pub series: Option<SeriesWithImprint>,
}
