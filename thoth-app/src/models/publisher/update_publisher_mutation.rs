use serde::Deserialize;
use serde::Serialize;
use thoth_api::model::publisher::Publisher;
use uuid::Uuid;

const UPDATE_PUBLISHER_MUTATION: &str = "
    mutation UpdatePublisher(
        $publisherId: Uuid!,
        $publisherName: String!,
        $publisherShortname: String
        $publisherUrl: String
    ) {
        updatePublisher(data: {
            publisherId: $publisherId
            publisherName: $publisherName
            publisherShortname: $publisherShortname
            publisherUrl: $publisherUrl
        }){
            publisherId
            publisherName
            createdAt
            updatedAt
        }
    }
";

graphql_query_builder! {
    UpdatePublisherRequest,
    UpdatePublisherRequestBody,
    Variables,
    UPDATE_PUBLISHER_MUTATION,
    UpdatePublisherResponseBody,
    UpdatePublisherResponseData,
    PushUpdatePublisher,
    PushActionUpdatePublisher
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub publisher_id: Uuid,
    pub publisher_name: String,
    pub publisher_shortname: Option<String>,
    pub publisher_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePublisherResponseData {
    pub update_publisher: Option<Publisher>,
}
