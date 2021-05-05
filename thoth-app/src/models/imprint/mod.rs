use thoth_api::imprint::model::ImprintExtended as Imprint;
use yew::html;
use yew::prelude::Html;
use yew::Callback;
use yew::MouseEvent;

use super::MetadataObject;
use crate::route::AdminRoute;
use crate::route::AppRoute;

impl MetadataObject for Imprint {
    fn create_route() -> AppRoute {
        AppRoute::Admin(AdminRoute::NewImprint)
    }

    fn edit_route(&self) -> AppRoute {
        AppRoute::Admin(AdminRoute::Imprint(self.imprint_id))
    }

    fn as_table_row(&self, callback: Callback<MouseEvent>) -> Html {
        let imprint_url = self.imprint_url.clone().unwrap_or_else(|| "".to_string());
        html! {
            <tr
                class="row"
                onclick=callback
            >
                <td>{&self.imprint_id}</td>
                <td>{&self.imprint_name}</td>
                <td>{&self.publisher.publisher_name}</td>
                <td>{imprint_url}</td>
                <td>{&self.updated_at.format("%F %T")}</td>
            </tr>
        }
    }
}

pub mod create_imprint_mutation;
pub mod delete_imprint_mutation;
pub mod imprint_query;
pub mod imprints_query;
pub mod update_imprint_mutation;
