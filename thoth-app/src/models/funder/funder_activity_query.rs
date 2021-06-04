use serde::Deserialize;
use serde::Serialize;
use thoth_api::funding::model::SlimFunding;
use uuid::Uuid;

use crate::graphql_query_builder;

pub const FUNDER_ACTIVITY_QUERY: &str = "
    query FunderActivityQuery($funderId: Uuid!) {
        funder(funderId: $funderId) {
            fundings {
                work {
                    workId
                    title
                    imprint {
                        publisher {
                            publisherId
                            publisherName
                        }
                    }
                }
            }
        }
    }
";

graphql_query_builder! {
    FunderActivityRequest,
    FunderActivityRequestBody,
    Variables,
    FUNDER_ACTIVITY_QUERY,
    FunderActivityResponseBody,
    FunderActivityResponseData,
    FetchFunderActivity,
    FetchActionFunderActivity
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub funder_id: Option<Uuid>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FunderActivityResponseData {
    pub funder: Option<FunderActivity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunderActivity {
    pub fundings: Option<Vec<SlimFunding>>,
}
