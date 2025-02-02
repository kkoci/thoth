use std::str::FromStr;
use thoth_api::model::location::Location;
use thoth_api::model::location::LocationPlatform;
use uuid::Uuid;
use yew::html;
use yew::prelude::*;
use yew::ComponentLink;
use yewtil::fetch::Fetch;
use yewtil::fetch::FetchAction;
use yewtil::fetch::FetchState;
use yewtil::future::LinkFuture;
use yewtil::NeqAssign;

use crate::agent::notification_bus::NotificationBus;
use crate::agent::notification_bus::NotificationDispatcher;
use crate::agent::notification_bus::NotificationStatus;
use crate::agent::notification_bus::Request;
use crate::component::utils::FormBooleanSelect;
use crate::component::utils::FormLocationPlatformSelect;
use crate::component::utils::FormUrlInput;
use crate::models::location::create_location_mutation::CreateLocationRequest;
use crate::models::location::create_location_mutation::CreateLocationRequestBody;
use crate::models::location::create_location_mutation::PushActionCreateLocation;
use crate::models::location::create_location_mutation::PushCreateLocation;
use crate::models::location::create_location_mutation::Variables;
use crate::models::location::delete_location_mutation::DeleteLocationRequest;
use crate::models::location::delete_location_mutation::DeleteLocationRequestBody;
use crate::models::location::delete_location_mutation::PushActionDeleteLocation;
use crate::models::location::delete_location_mutation::PushDeleteLocation;
use crate::models::location::delete_location_mutation::Variables as DeleteVariables;
use crate::models::location::location_platforms_query::FetchActionLocationPlatforms;
use crate::models::location::location_platforms_query::FetchLocationPlatforms;
use crate::models::location::LocationPlatformValues;
use crate::string::CANCEL_BUTTON;
use crate::string::EMPTY_LOCATIONS;
use crate::string::NO;
use crate::string::REMOVE_BUTTON;
use crate::string::YES;

use super::ToOption;

pub struct LocationsFormComponent {
    props: Props,
    data: LocationsFormData,
    new_location: Location,
    show_add_form: bool,
    fetch_location_platforms: FetchLocationPlatforms,
    push_location: PushCreateLocation,
    delete_location: PushDeleteLocation,
    link: ComponentLink<Self>,
    notification_bus: NotificationDispatcher,
}

#[derive(Default)]
struct LocationsFormData {
    location_platforms: Vec<LocationPlatformValues>,
}

pub enum Msg {
    ToggleAddFormDisplay(bool),
    SetLocationPlatformsFetchState(FetchActionLocationPlatforms),
    GetLocationPlatforms,
    SetLocationPushState(PushActionCreateLocation),
    CreateLocation,
    SetLocationDeleteState(PushActionDeleteLocation),
    DeleteLocation(Uuid),
    ChangeLandingPage(String),
    ChangeFullTextUrl(String),
    ChangeLocationPlatform(LocationPlatform),
    ChangeCanonical(bool),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub locations: Option<Vec<Location>>,
    pub publication_id: Uuid,
    pub update_locations: Callback<Option<Vec<Location>>>,
}

impl Component for LocationsFormComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let data: LocationsFormData = Default::default();
        let show_add_form = false;
        // The first location needs to be canonical = true (as it will be
        // the only location); subsequent locations need to be canonical = false
        let new_location = Location {
            canonical: props.locations.as_ref().unwrap_or(&vec![]).is_empty(),
            ..Default::default()
        };
        let fetch_location_platforms = Default::default();
        let push_location = Default::default();
        let delete_location = Default::default();
        let notification_bus = NotificationBus::dispatcher();

        link.send_message(Msg::GetLocationPlatforms);

        LocationsFormComponent {
            props,
            data,
            new_location,
            show_add_form,
            fetch_location_platforms,
            push_location,
            delete_location,
            link,
            notification_bus,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleAddFormDisplay(value) => {
                self.show_add_form = value;
                true
            }
            Msg::SetLocationPlatformsFetchState(fetch_state) => {
                self.fetch_location_platforms.apply(fetch_state);
                self.data.location_platforms = match self.fetch_location_platforms.as_ref().state()
                {
                    FetchState::NotFetching(_) => vec![],
                    FetchState::Fetching(_) => vec![],
                    FetchState::Fetched(body) => body.data.location_platforms.enum_values.clone(),
                    FetchState::Failed(_, _err) => vec![],
                };
                true
            }
            Msg::GetLocationPlatforms => {
                self.link.send_future(
                    self.fetch_location_platforms
                        .fetch(Msg::SetLocationPlatformsFetchState),
                );
                self.link
                    .send_message(Msg::SetLocationPlatformsFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetLocationPushState(fetch_state) => {
                self.push_location.apply(fetch_state);
                match self.push_location.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.create_location {
                        Some(l) => {
                            let location = l.clone();
                            let mut locations: Vec<Location> =
                                self.props.locations.clone().unwrap_or_default();
                            locations.push(location);
                            self.props.update_locations.emit(Some(locations));
                            self.link.send_message(Msg::ToggleAddFormDisplay(false));
                            true
                        }
                        None => {
                            self.link.send_message(Msg::ToggleAddFormDisplay(false));
                            self.notification_bus.send(Request::NotificationBusMsg((
                                "Failed to save".to_string(),
                                NotificationStatus::Danger,
                            )));
                            false
                        }
                    },
                    FetchState::Failed(_, err) => {
                        self.link.send_message(Msg::ToggleAddFormDisplay(false));
                        self.notification_bus.send(Request::NotificationBusMsg((
                            err.to_string(),
                            NotificationStatus::Danger,
                        )));
                        false
                    }
                }
            }
            Msg::CreateLocation => {
                let body = CreateLocationRequestBody {
                    variables: Variables {
                        publication_id: self.props.publication_id,
                        landing_page: self.new_location.landing_page.clone(),
                        full_text_url: self.new_location.full_text_url.clone(),
                        location_platform: self.new_location.location_platform.clone(),
                        canonical: self.new_location.canonical,
                    },
                    ..Default::default()
                };
                let request = CreateLocationRequest { body };
                self.push_location = Fetch::new(request);
                self.link
                    .send_future(self.push_location.fetch(Msg::SetLocationPushState));
                self.link
                    .send_message(Msg::SetLocationPushState(FetchAction::Fetching));
                false
            }
            Msg::SetLocationDeleteState(fetch_state) => {
                self.delete_location.apply(fetch_state);
                match self.delete_location.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.delete_location {
                        Some(location) => {
                            let to_keep: Vec<Location> = self
                                .props
                                .locations
                                .clone()
                                .unwrap_or_default()
                                .into_iter()
                                .filter(|l| l.location_id != location.location_id)
                                .collect();
                            self.props.update_locations.emit(Some(to_keep));
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
            Msg::DeleteLocation(location_id) => {
                let body = DeleteLocationRequestBody {
                    variables: DeleteVariables { location_id },
                    ..Default::default()
                };
                let request = DeleteLocationRequest { body };
                self.delete_location = Fetch::new(request);
                self.link
                    .send_future(self.delete_location.fetch(Msg::SetLocationDeleteState));
                self.link
                    .send_message(Msg::SetLocationDeleteState(FetchAction::Fetching));
                false
            }
            Msg::ChangeLandingPage(val) => self
                .new_location
                .landing_page
                .neq_assign(val.to_opt_string()),
            Msg::ChangeFullTextUrl(val) => self
                .new_location
                .full_text_url
                .neq_assign(val.to_opt_string()),
            Msg::ChangeLocationPlatform(code) => {
                self.new_location.location_platform.neq_assign(code)
            }
            Msg::ChangeCanonical(val) => self.new_location.canonical.neq_assign(val),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let locations = self.props.locations.clone().unwrap_or_default();
        let open_modal = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::ToggleAddFormDisplay(true)
        });
        let close_modal = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::ToggleAddFormDisplay(false)
        });
        html! {
            <nav class="panel">
                <p class="panel-heading">
                    { "Locations" }
                </p>
                <div class="panel-block">
                    <button
                        class="button is-link is-outlined is-success is-fullwidth"
                        onclick=open_modal
                    >
                        { "Add Location" }
                    </button>
                </div>
                <div class=self.add_form_status()>
                    <div class="modal-background" onclick=&close_modal></div>
                    <div class="modal-card">
                        <header class="modal-card-head">
                            <p class="modal-card-title">{ "New Location" }</p>
                            <button
                                class="delete"
                                aria-label="close"
                                onclick=&close_modal
                            ></button>
                        </header>
                        <section class="modal-card-body">
                            <form id="locations-form" onsubmit=self.link.callback(|e: FocusEvent| {
                                e.prevent_default();
                                Msg::CreateLocation
                            })
                            >
                                <FormUrlInput
                                    label="Landing Page"
                                    value=self.new_location.landing_page.clone()
                                    oninput=self.link.callback(|e: InputData| Msg::ChangeLandingPage(e.value))
                                />
                                <FormUrlInput
                                    label="Full Text URL"
                                    value=self.new_location.full_text_url.clone().unwrap_or_else(|| "".to_string())
                                    oninput=self.link.callback(|e: InputData| Msg::ChangeFullTextUrl(e.value))
                                />
                                <FormLocationPlatformSelect
                                    label = "Location Platform"
                                    value=self.new_location.location_platform.clone()
                                    data=self.data.location_platforms.clone()
                                    onchange=self.link.callback(|event| match event {
                                        ChangeData::Select(elem) => {
                                            let value = elem.value();
                                            Msg::ChangeLocationPlatform(
                                                LocationPlatform::from_str(&value).unwrap()
                                            )
                                        }
                                        _ => unreachable!(),
                                    })
                                    required = true
                                />
                                <FormBooleanSelect
                                    label = "Canonical"
                                    value=self.new_location.canonical
                                    onchange=self.link.callback(|event| match event {
                                        ChangeData::Select(elem) => {
                                            let value = elem.value();
                                            let boolean = value == "true";
                                            Msg::ChangeCanonical(boolean)
                                        }
                                        _ => unreachable!(),
                                    })
                                    required = true
                                />
                            </form>
                        </section>
                        <footer class="modal-card-foot">
                            <button
                                class="button is-success"
                                type="submit"
                                form="locations-form"
                            >
                                { "Add Location" }
                            </button>
                            <button
                                class="button"
                                onclick=&close_modal
                            >
                                { CANCEL_BUTTON }
                            </button>
                        </footer>
                    </div>
                </div>
                {
                    if !locations.is_empty() {
                        html!{{for locations.iter().map(|l| self.render_location(l))}}
                    } else {
                        html! {
                            <div class="notification is-warning is-light">
                                { EMPTY_LOCATIONS }
                            </div>
                        }
                    }
                }
            </nav>
        }
    }
}

impl LocationsFormComponent {
    fn add_form_status(&self) -> String {
        match self.show_add_form {
            true => "modal is-active".to_string(),
            false => "modal".to_string(),
        }
    }

    fn render_location(&self, l: &Location) -> Html {
        let location_id = l.location_id;
        let mut delete_callback = Some(
            self.link
                .callback(move |_| Msg::DeleteLocation(location_id)),
        );
        let mut delete_deactivated = false;
        // If the location is canonical and other (non-canonical) locations exist, prevent it from
        // being deleted by deactivating the delete button and unsetting its callback attribute
        if l.canonical && self.props.locations.as_ref().unwrap_or(&vec![]).len() > 1 {
            delete_callback = None;
            delete_deactivated = true;
        }
        html! {
            <div class="panel-block field is-horizontal">
                <span class="panel-icon">
                    <i class="fas fa-file-invoice-dollar" aria-hidden="true"></i>
                </span>
                <div class="field-body">
                    <div class="field" style="width: 8em;">
                        <label class="label">{ "Landing Page" }</label>
                        <div class="control is-expanded">
                            {&l.landing_page.clone().unwrap_or_else(|| "".to_string())}
                        </div>
                    </div>
                    <div class="field" style="width: 8em;">
                        <label class="label">{ "Full Text URL" }</label>
                        <div class="control is-expanded">
                            {&l.full_text_url.clone().unwrap_or_else(|| "".to_string())}
                        </div>
                    </div>
                    <div class="field" style="width: 8em;">
                        <label class="label">{ "Platform" }</label>
                        <div class="control is-expanded">
                            {&l.location_platform}
                        </div>
                    </div>
                    <div class="field" style="width: 8em;">
                        <label class="label">{ "Canonical" }</label>
                        <div class="control is-expanded">
                            {
                                match l.canonical {
                                    true => { YES },
                                    false => { NO }
                                }
                            }
                        </div>
                    </div>

                    <div class="field">
                        <label class="label"></label>
                        <div class="control is-expanded">
                            <a
                                class="button is-danger"
                                onclick=delete_callback
                                disabled=delete_deactivated
                            >
                                { REMOVE_BUTTON }
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
