use std::str::FromStr;
use thoth_api::account::model::AccountDetails;
use thoth_api::model::imprint::ImprintWithPublisher;
use thoth_api::model::series::SeriesType;
use thoth_api::model::series::SeriesWithImprint;
use uuid::Uuid;
use yew::html;
use yew::prelude::*;
use yew::ComponentLink;
use yew_router::agent::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use yew_router::route::Route;
use yewtil::fetch::Fetch;
use yewtil::fetch::FetchAction;
use yewtil::fetch::FetchState;
use yewtil::future::LinkFuture;
use yewtil::NeqAssign;

use crate::agent::notification_bus::NotificationBus;
use crate::agent::notification_bus::NotificationDispatcher;
use crate::agent::notification_bus::NotificationStatus;
use crate::agent::notification_bus::Request;
use crate::component::delete_dialogue::ConfirmDeleteComponent;
use crate::component::utils::FormImprintSelect;
use crate::component::utils::FormSeriesTypeSelect;
use crate::component::utils::FormTextInput;
use crate::component::utils::FormUrlInput;
use crate::component::utils::Loader;
use crate::models::imprint::imprints_query::FetchActionImprints;
use crate::models::imprint::imprints_query::FetchImprints;
use crate::models::imprint::imprints_query::ImprintsRequest;
use crate::models::imprint::imprints_query::ImprintsRequestBody;
use crate::models::imprint::imprints_query::Variables as ImprintsVariables;
use crate::models::series::delete_series_mutation::DeleteSeriesRequest;
use crate::models::series::delete_series_mutation::DeleteSeriesRequestBody;
use crate::models::series::delete_series_mutation::PushActionDeleteSeries;
use crate::models::series::delete_series_mutation::PushDeleteSeries;
use crate::models::series::delete_series_mutation::Variables as DeleteVariables;
use crate::models::series::series_query::FetchActionSeries;
use crate::models::series::series_query::FetchSeries;
use crate::models::series::series_query::SeriesRequest;
use crate::models::series::series_query::SeriesRequestBody;
use crate::models::series::series_query::Variables;
use crate::models::series::series_types_query::FetchActionSeriesTypes;
use crate::models::series::series_types_query::FetchSeriesTypes;
use crate::models::series::update_series_mutation::PushActionUpdateSeries;
use crate::models::series::update_series_mutation::PushUpdateSeries;
use crate::models::series::update_series_mutation::UpdateSeriesRequest;
use crate::models::series::update_series_mutation::UpdateSeriesRequestBody;
use crate::models::series::update_series_mutation::Variables as UpdateVariables;
use crate::models::series::SeriesTypeValues;
use crate::route::AdminRoute;
use crate::route::AppRoute;
use crate::string::SAVE_BUTTON;

use super::ToOption;

pub struct SeriesComponent {
    series: SeriesWithImprint,
    fetch_series: FetchSeries,
    push_series: PushUpdateSeries,
    data: SeriesFormData,
    fetch_imprints: FetchImprints,
    fetch_series_types: FetchSeriesTypes,
    delete_series: PushDeleteSeries,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
    notification_bus: NotificationDispatcher,
    props: Props,
}

#[derive(Default)]
struct SeriesFormData {
    imprints: Vec<ImprintWithPublisher>,
    series_types: Vec<SeriesTypeValues>,
}

#[allow(clippy::large_enum_variant)]
pub enum Msg {
    SetImprintsFetchState(FetchActionImprints),
    GetImprints,
    SetSeriesTypesFetchState(FetchActionSeriesTypes),
    GetSeriesTypes,
    SetSeriesFetchState(FetchActionSeries),
    GetSeries,
    SetSeriesPushState(PushActionUpdateSeries),
    UpdateSeries,
    SetSeriesDeleteState(PushActionDeleteSeries),
    DeleteSeries,
    ChangeSeriesType(SeriesType),
    ChangeImprint(Uuid),
    ChangeSeriesName(String),
    ChangeIssnPrint(String),
    ChangeIssnDigital(String),
    ChangeSeriesUrl(String),
    ChangeRoute(AppRoute),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub series_id: Uuid,
    pub current_user: AccountDetails,
}

impl Component for SeriesComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_series: FetchSeries = Default::default();
        let push_series = Default::default();
        let delete_series = Default::default();
        let notification_bus = NotificationBus::dispatcher();
        let series: SeriesWithImprint = Default::default();
        let data: SeriesFormData = Default::default();
        let fetch_imprints: FetchImprints = Default::default();
        let fetch_series_types: FetchSeriesTypes = Default::default();
        let router = RouteAgentDispatcher::new();

        link.send_message(Msg::GetSeries);
        link.send_message(Msg::GetImprints);
        link.send_message(Msg::GetSeriesTypes);

        SeriesComponent {
            series,
            fetch_series,
            push_series,
            data,
            fetch_imprints,
            fetch_series_types,
            delete_series,
            link,
            router,
            notification_bus,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetImprintsFetchState(fetch_state) => {
                self.fetch_imprints.apply(fetch_state);
                self.data.imprints = match self.fetch_imprints.as_ref().state() {
                    FetchState::NotFetching(_) => vec![],
                    FetchState::Fetching(_) => vec![],
                    FetchState::Fetched(body) => body.data.imprints.clone(),
                    FetchState::Failed(_, _err) => vec![],
                };
                true
            }
            Msg::GetImprints => {
                let body = ImprintsRequestBody {
                    variables: ImprintsVariables {
                        publishers: self.props.current_user.resource_access.restricted_to(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let request = ImprintsRequest { body };
                self.fetch_imprints = Fetch::new(request);

                self.link
                    .send_future(self.fetch_imprints.fetch(Msg::SetImprintsFetchState));
                self.link
                    .send_message(Msg::SetImprintsFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetSeriesTypesFetchState(fetch_state) => {
                self.fetch_series_types.apply(fetch_state);
                self.data.series_types = match self.fetch_series_types.as_ref().state() {
                    FetchState::NotFetching(_) => vec![],
                    FetchState::Fetching(_) => vec![],
                    FetchState::Fetched(body) => body.data.series_types.enum_values.clone(),
                    FetchState::Failed(_, _err) => vec![],
                };
                true
            }
            Msg::GetSeriesTypes => {
                self.link
                    .send_future(self.fetch_series_types.fetch(Msg::SetSeriesTypesFetchState));
                self.link
                    .send_message(Msg::SetSeriesTypesFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetSeriesFetchState(fetch_state) => {
                self.fetch_series.apply(fetch_state);
                match self.fetch_series.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => {
                        self.series = match &body.data.series {
                            Some(c) => c.to_owned(),
                            None => Default::default(),
                        };
                        // If user doesn't have permission to edit this object, redirect to dashboard
                        if let Some(publishers) =
                            self.props.current_user.resource_access.restricted_to()
                        {
                            if !publishers
                                .contains(&self.series.imprint.publisher.publisher_id.to_string())
                            {
                                self.router.send(RouteRequest::ChangeRoute(Route::from(
                                    AppRoute::Admin(AdminRoute::Dashboard),
                                )));
                            }
                        }
                        true
                    }
                    FetchState::Failed(_, _err) => false,
                }
            }
            Msg::GetSeries => {
                let body = SeriesRequestBody {
                    variables: Variables {
                        series_id: Some(self.props.series_id),
                    },
                    ..Default::default()
                };
                let request = SeriesRequest { body };
                self.fetch_series = Fetch::new(request);

                self.link
                    .send_future(self.fetch_series.fetch(Msg::SetSeriesFetchState));
                self.link
                    .send_message(Msg::SetSeriesFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetSeriesPushState(fetch_state) => {
                self.push_series.apply(fetch_state);
                match self.push_series.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.update_series {
                        Some(s) => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Saved {}", s.series_name),
                                NotificationStatus::Success,
                            )));
                            true
                        }
                        None => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                "Failed to save".to_string(),
                                NotificationStatus::Danger,
                            )));
                            false
                        }
                    },
                    FetchState::Failed(_, err) => {
                        self.notification_bus.send(Request::NotificationBusMsg((
                            err.to_string(),
                            NotificationStatus::Danger,
                        )));
                        false
                    }
                }
            }
            Msg::UpdateSeries => {
                let body = UpdateSeriesRequestBody {
                    variables: UpdateVariables {
                        series_id: self.series.series_id,
                        series_type: self.series.series_type.clone(),
                        series_name: self.series.series_name.clone(),
                        issn_print: self.series.issn_print.clone(),
                        issn_digital: self.series.issn_digital.clone(),
                        series_url: self.series.series_url.clone(),
                        imprint_id: self.series.imprint.imprint_id,
                    },
                    ..Default::default()
                };
                let request = UpdateSeriesRequest { body };
                self.push_series = Fetch::new(request);
                self.link
                    .send_future(self.push_series.fetch(Msg::SetSeriesPushState));
                self.link
                    .send_message(Msg::SetSeriesPushState(FetchAction::Fetching));
                false
            }
            Msg::SetSeriesDeleteState(fetch_state) => {
                self.delete_series.apply(fetch_state);
                match self.delete_series.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.delete_series {
                        Some(s) => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Deleted {}", s.series_name),
                                NotificationStatus::Success,
                            )));
                            self.link.send_message(Msg::ChangeRoute(AppRoute::Admin(
                                AdminRoute::Serieses,
                            )));
                            true
                        }
                        None => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                "Failed to save".to_string(),
                                NotificationStatus::Danger,
                            )));
                            false
                        }
                    },
                    FetchState::Failed(_, err) => {
                        self.notification_bus.send(Request::NotificationBusMsg((
                            err.to_string(),
                            NotificationStatus::Danger,
                        )));
                        false
                    }
                }
            }
            Msg::DeleteSeries => {
                let body = DeleteSeriesRequestBody {
                    variables: DeleteVariables {
                        series_id: self.series.series_id,
                    },
                    ..Default::default()
                };
                let request = DeleteSeriesRequest { body };
                self.delete_series = Fetch::new(request);
                self.link
                    .send_future(self.delete_series.fetch(Msg::SetSeriesDeleteState));
                self.link
                    .send_message(Msg::SetSeriesDeleteState(FetchAction::Fetching));
                false
            }
            Msg::ChangeSeriesType(series_type) => self.series.series_type.neq_assign(series_type),
            Msg::ChangeImprint(imprint_id) => self.series.imprint.imprint_id.neq_assign(imprint_id),
            Msg::ChangeSeriesName(series_name) => self
                .series
                .series_name
                .neq_assign(series_name.trim().to_owned()),
            Msg::ChangeIssnPrint(issn_print) => self
                .series
                .issn_print
                .neq_assign(issn_print.trim().to_owned()),
            Msg::ChangeIssnDigital(issn_digital) => self
                .series
                .issn_digital
                .neq_assign(issn_digital.trim().to_owned()),
            Msg::ChangeSeriesUrl(value) => self.series.series_url.neq_assign(value.to_opt_string()),
            Msg::ChangeRoute(r) => {
                let route = Route::from(r);
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let updated_permissions =
            self.props.current_user.resource_access != props.current_user.resource_access;
        self.props = props;
        if updated_permissions {
            self.link.send_message(Msg::GetImprints);
        }
        false
    }

    fn view(&self) -> Html {
        match self.fetch_series.as_ref().state() {
            FetchState::NotFetching(_) => html! {<Loader/>},
            FetchState::Fetching(_) => html! {<Loader/>},
            FetchState::Fetched(_body) => {
                let callback = self.link.callback(|event: FocusEvent| {
                    event.prevent_default();
                    Msg::UpdateSeries
                });
                html! {
                    <>
                        <nav class="level">
                            <div class="level-left">
                                <p class="subtitle is-5">
                                    { "Edit series" }
                                </p>
                            </div>
                            <div class="level-right">
                                <p class="level-item">
                                    <ConfirmDeleteComponent
                                        onclick=self.link.callback(|_| Msg::DeleteSeries)
                                        object_name=self.series.series_name.clone()
                                    />
                                </p>
                            </div>
                        </nav>
                        <form onsubmit=callback>
                            <FormSeriesTypeSelect
                                label = "Series Type"
                                value=self.series.series_type.clone()
                                onchange=self.link.callback(|event| match event {
                                    ChangeData::Select(elem) => {
                                        let value = elem.value();
                                        Msg::ChangeSeriesType(SeriesType::from_str(&value).unwrap())
                                    }
                                    _ => unreachable!(),
                                })
                                data=self.data.series_types.clone()
                                required = true
                            />
                            <FormImprintSelect
                                label = "Imprint"
                                value=self.series.imprint.imprint_id
                                data=self.data.imprints.clone()
                                onchange=self.link.callback(|event| match event {
                                    ChangeData::Select(elem) => {
                                        let value = elem.value();
                                        Msg::ChangeImprint(Uuid::parse_str(&value).unwrap_or_default())
                                    }
                                    _ => unreachable!(),
                                })
                                required = true
                            />
                            <FormTextInput
                                label = "Series Name"
                                value=self.series.series_name.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeSeriesName(e.value))
                                required=true
                            />
                            <FormTextInput
                                label = "ISSN Print"
                                value=self.series.issn_print.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeIssnPrint(e.value))
                                required=true
                            />
                            <FormTextInput
                                label = "ISSN Digital"
                                value=self.series.issn_digital.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeIssnDigital(e.value))
                                required=true
                            />
                            <FormUrlInput
                                label = "Series URL"
                                value=self.series.series_url.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeSeriesUrl(e.value))
                            />

                            <div class="field">
                                <div class="control">
                                    <button class="button is-success" type="submit">
                                        { SAVE_BUTTON }
                                    </button>
                                </div>
                            </div>
                        </form>
                    </>
                }
            }
            FetchState::Failed(_, err) => html! {&err},
        }
    }
}
