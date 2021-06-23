use serde::Deserialize;
use serde::Serialize;
use thoth_api::publication::model::PublicationExtended as Publication;
use uuid::Uuid;

pub const PUBLICATION_QUERY: &str = "
    query PublicationQuery($publicationId: Uuid!) {
        publication(publicationId: $publicationId) {
            publicationId
            publicationType
            workId
            isbn
            publicationUrl
            prices {
                priceId
                publicationId
                currencyCode
                unitPrice
                createdAt
                updatedAt
            }
            work {
                workId
                title
                imprint {
                    imprintId
                    imprintName
                    updatedAt
                    publisher {
                        publisherId
                        publisherName
                        createdAt
                        updatedAt
                    }
                }
            }
        }
    }
";

graphql_query_builder! {
    PublicationRequest,
    PublicationRequestBody,
    Variables,
    PUBLICATION_QUERY,
    PublicationResponseBody,
    PublicationResponseData,
    FetchPublication,
    FetchActionPublication
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub publication_id: Option<Uuid>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PublicationResponseData {
    pub publication: Option<Publication>,
}
