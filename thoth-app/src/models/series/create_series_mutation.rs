use serde::Deserialize;
use serde::Serialize;
use thoth_api::series::model::SeriesType;
use uuid::Uuid;

const CREATE_SERIES_MUTATION: &str = "
    mutation CreateSeries(
            $seriesType: SeriesType!,
            $seriesName: String!,
            $issnPrint: String!,
            $issnDigital: String!,
            $seriesUrl: String,
            $imprintId: Uuid!
    ) {
        createSeries(data: {
            seriesType: $seriesType
            seriesName: $seriesName
            issnPrint: $issnPrint
            issnDigital: $issnDigital
            seriesUrl: $seriesUrl
            imprintId: $imprintId
        }){
            seriesId
            seriesName
        }
    }
";

graphql_query_builder! {
    CreateSeriesRequest,
    CreateSeriesRequestBody,
    Variables,
    CREATE_SERIES_MUTATION,
    CreateSeriesResponseBody,
    CreateSeriesResponseData,
    PushCreateSeries,
    PushActionCreateSeries
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub series_type: SeriesType,
    pub series_name: String,
    pub issn_print: String,
    pub issn_digital: String,
    pub series_url: Option<String>,
    pub imprint_id: Uuid,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SlimSeries {
    pub series_id: Uuid,
    pub series_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesResponseData {
    pub create_series: Option<SlimSeries>,
}
