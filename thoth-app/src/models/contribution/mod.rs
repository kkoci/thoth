use serde::Deserialize;
use serde::Serialize;
use thoth_api::contribution::model::ContributionType;
use yew::prelude::html;
use yew::Html;

use super::contributor::Contributor;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    pub work_id: String,
    pub contributor_id: String,
    pub contribution_type: ContributionType,
    pub main_contribution: bool,
    pub biography: Option<String>,
    pub institution: Option<String>,
    pub contributor: Contributor,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContributionTypeDefinition {
    pub enum_values: Vec<ContributionTypeValues>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContributionTypeValues {
    pub name: ContributionType,
}

impl Contribution {
    pub fn main_contribution_item(&self) -> Html {
        if self.main_contribution {
            html! {
                <small class="contributor">
                    {&self.contributor.full_name}
                    <span>{ ", " }</span>
                </small>
            }
        } else {
            html! {}
        }
    }
}

pub mod create_contribution_mutation;
pub mod delete_contribution_mutation;
pub mod contribution_types_query;
pub mod update_contribution_mutation;
