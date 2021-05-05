use thoth_api::publisher::model::Publisher;
use yew::html;
use yew::prelude::Html;
use yew::Callback;
use yew::MouseEvent;

use crate::route::AdminRoute;
use crate::route::AppRoute;

use super::MetadataObject;

impl MetadataObject for Publisher {
    fn create_route() -> AppRoute {
        AppRoute::Admin(AdminRoute::NewPublisher)
    }

    fn edit_route(&self) -> AppRoute {
        AppRoute::Admin(AdminRoute::Publisher(self.publisher_id))
    }

    fn as_table_row(&self, callback: Callback<MouseEvent>) -> Html {
        let publisher_shortname = self
            .publisher_shortname
            .clone()
            .unwrap_or_else(|| "".to_string());
        let publisher_url = self.publisher_url.clone().unwrap_or_else(|| "".to_string());
        html! {
            <tr
                class="row"
                onclick=callback
            >
                <td>{&self.publisher_id}</td>
                <td>{&self.publisher_name}</td>
                <td>{publisher_shortname}</td>
                <td>{publisher_url}</td>
                <td>{&self.updated_at.format("%F %T")}</td>
            </tr>
        }
    }
}

pub mod create_publisher_mutation;
pub mod delete_publisher_mutation;
pub mod publisher_query;
pub mod publishers_query;
pub mod update_publisher_mutation;
