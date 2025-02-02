use std::str::FromStr;
use thoth_api::model::institution::CountryCode;
use thoth_api::model::institution::Institution;
use thoth_api::model::work::WorkWithRelations;
use thoth_api::model::{Doi, Ror, DOI_DOMAIN, ROR_DOMAIN};
use thoth_errors::ThothError;
use uuid::Uuid;
use yew::html;
use yew::prelude::*;
use yew::ComponentLink;
use yew_router::agent::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use yew_router::prelude::RouterAnchor;
use yew_router::route::Route;
use yewtil::fetch::Fetch;
use yewtil::fetch::FetchAction;
use yewtil::fetch::FetchState;
use yewtil::future::LinkFuture;
use yewtil::NeqAssign;

use crate::agent::institution_activity_checker::InstitutionActivityChecker;
use crate::agent::institution_activity_checker::Request as InstitutionActivityRequest;
use crate::agent::notification_bus::NotificationBus;
use crate::agent::notification_bus::NotificationDispatcher;
use crate::agent::notification_bus::NotificationStatus;
use crate::agent::notification_bus::Request;
use crate::component::delete_dialogue::ConfirmDeleteComponent;
use crate::component::utils::FormCountryCodeSelect;
use crate::component::utils::FormTextInput;
use crate::component::utils::FormTextInputExtended;
use crate::component::utils::Loader;
use crate::models::institution::country_codes_query::FetchActionCountryCodes;
use crate::models::institution::country_codes_query::FetchCountryCodes;
use crate::models::institution::delete_institution_mutation::DeleteInstitutionRequest;
use crate::models::institution::delete_institution_mutation::DeleteInstitutionRequestBody;
use crate::models::institution::delete_institution_mutation::PushActionDeleteInstitution;
use crate::models::institution::delete_institution_mutation::PushDeleteInstitution;
use crate::models::institution::delete_institution_mutation::Variables as DeleteVariables;
use crate::models::institution::institution_activity_query::InstitutionActivityResponseData;
use crate::models::institution::institution_query::FetchActionInstitution;
use crate::models::institution::institution_query::FetchInstitution;
use crate::models::institution::institution_query::InstitutionRequest;
use crate::models::institution::institution_query::InstitutionRequestBody;
use crate::models::institution::institution_query::Variables;
use crate::models::institution::update_institution_mutation::PushActionUpdateInstitution;
use crate::models::institution::update_institution_mutation::PushUpdateInstitution;
use crate::models::institution::update_institution_mutation::UpdateInstitutionRequest;
use crate::models::institution::update_institution_mutation::UpdateInstitutionRequestBody;
use crate::models::institution::update_institution_mutation::Variables as UpdateVariables;
use crate::models::institution::CountryCodeValues;
use crate::models::EditRoute;
use crate::route::AdminRoute;
use crate::route::AppRoute;
use crate::string::SAVE_BUTTON;

pub struct InstitutionComponent {
    institution: Institution,
    fetch_country_codes: FetchCountryCodes,
    // Track the user-entered DOI string, which may not be validly formatted
    institution_doi: String,
    institution_doi_warning: String,
    // Track the user-entered ROR string, which may not be validly formatted
    ror: String,
    ror_warning: String,
    fetch_institution: FetchInstitution,
    push_institution: PushUpdateInstitution,
    delete_institution: PushDeleteInstitution,
    data: InstitutionFormData,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
    notification_bus: NotificationDispatcher,
    _institution_activity_checker: Box<dyn Bridge<InstitutionActivityChecker>>,
    funded_works: Vec<WorkWithRelations>,
    affiliated_works: Vec<WorkWithRelations>,
}

#[derive(Default)]
struct InstitutionFormData {
    country_codes: Vec<CountryCodeValues>,
}

pub enum Msg {
    SetCountryCodesFetchState(FetchActionCountryCodes),
    GetCountryCodes,
    GetInstitutionActivity(InstitutionActivityResponseData),
    SetInstitutionFetchState(FetchActionInstitution),
    GetInstitution,
    SetInstitutionPushState(PushActionUpdateInstitution),
    UpdateInstitution,
    SetInstitutionDeleteState(PushActionDeleteInstitution),
    DeleteInstitution,
    ChangeInstitutionName(String),
    ChangeInstitutionDoi(String),
    ChangeRor(String),
    ChangeCountryCode(String),
    ChangeRoute(AppRoute),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub institution_id: Uuid,
}

impl Component for InstitutionComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let body = InstitutionRequestBody {
            variables: Variables {
                institution_id: Some(props.institution_id),
            },
            ..Default::default()
        };
        let request = InstitutionRequest { body };
        let fetch_institution = Fetch::new(request);
        let push_institution = Default::default();
        let delete_institution = Default::default();
        let data: InstitutionFormData = Default::default();
        let notification_bus = NotificationBus::dispatcher();
        let institution: Institution = Default::default();
        let fetch_country_codes = Default::default();
        let institution_doi = Default::default();
        let institution_doi_warning = Default::default();
        let ror = Default::default();
        let ror_warning = Default::default();
        let router = RouteAgentDispatcher::new();
        let mut _institution_activity_checker =
            InstitutionActivityChecker::bridge(link.callback(Msg::GetInstitutionActivity));
        let funded_works = Default::default();
        let affiliated_works = Default::default();

        link.send_message(Msg::GetInstitution);
        link.send_message(Msg::GetCountryCodes);
        _institution_activity_checker.send(
            InstitutionActivityRequest::RetrieveInstitutionActivity(props.institution_id),
        );

        InstitutionComponent {
            institution,
            fetch_country_codes,
            institution_doi,
            institution_doi_warning,
            ror,
            ror_warning,
            fetch_institution,
            push_institution,
            delete_institution,
            data,
            link,
            router,
            notification_bus,
            _institution_activity_checker,
            funded_works,
            affiliated_works,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetCountryCodesFetchState(fetch_state) => {
                self.fetch_country_codes.apply(fetch_state);
                self.data.country_codes = match self.fetch_country_codes.as_ref().state() {
                    FetchState::NotFetching(_) => vec![],
                    FetchState::Fetching(_) => vec![],
                    FetchState::Fetched(body) => body.data.country_codes.enum_values.clone(),
                    FetchState::Failed(_, _err) => vec![],
                };
                true
            }
            Msg::GetCountryCodes => {
                self.link.send_future(
                    self.fetch_country_codes
                        .fetch(Msg::SetCountryCodesFetchState),
                );
                self.link
                    .send_message(Msg::SetCountryCodesFetchState(FetchAction::Fetching));
                false
            }
            Msg::GetInstitutionActivity(response) => {
                let mut should_render = false;
                if let Some(institution) = response.institution {
                    if let Some(fundings) = institution.fundings {
                        if !fundings.is_empty() {
                            self.funded_works = fundings.iter().map(|f| f.work.clone()).collect();
                            self.funded_works.sort_by_key(|f| f.work_id);
                            self.funded_works.dedup_by_key(|f| f.work_id);
                            should_render = true;
                        }
                    }
                    if let Some(affiliations) = institution.affiliations {
                        if !affiliations.is_empty() {
                            self.affiliated_works = affiliations
                                .iter()
                                .map(|a| a.contribution.work.clone())
                                .collect();
                            self.affiliated_works.sort_by_key(|a| a.work_id);
                            self.affiliated_works.dedup_by_key(|a| a.work_id);
                            should_render = true;
                        }
                    }
                }
                should_render
            }
            Msg::SetInstitutionFetchState(fetch_state) => {
                self.fetch_institution.apply(fetch_state);
                match self.fetch_institution.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => {
                        self.institution = match &body.data.institution {
                            Some(c) => c.to_owned(),
                            None => Default::default(),
                        };
                        // Initialise user-entered DOI variable to match DOI in database
                        self.institution_doi = self
                            .institution
                            .institution_doi
                            .clone()
                            .unwrap_or_default()
                            .to_string();
                        // Initialise user-entered ROR variable to match ROR in database
                        self.ror = self.institution.ror.clone().unwrap_or_default().to_string();
                        true
                    }
                    FetchState::Failed(_, _err) => false,
                }
            }
            Msg::GetInstitution => {
                self.link
                    .send_future(self.fetch_institution.fetch(Msg::SetInstitutionFetchState));
                self.link
                    .send_message(Msg::SetInstitutionFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetInstitutionPushState(fetch_state) => {
                self.push_institution.apply(fetch_state);
                match self.push_institution.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.update_institution {
                        Some(i) => {
                            // Save was successful: update user-entered DOI variable to match DOI in database
                            self.institution_doi = self
                                .institution
                                .institution_doi
                                .clone()
                                .unwrap_or_default()
                                .to_string();
                            self.institution_doi_warning.clear();
                            // Save was successful: update user-entered ROR variable to match ROR in database
                            self.ror = self.institution.ror.clone().unwrap_or_default().to_string();
                            self.ror_warning.clear();
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Saved {}", i.institution_name),
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
            Msg::UpdateInstitution => {
                // Only update the DOI value with the current user-entered string
                // if it is validly formatted - otherwise keep the database version.
                // If no DOI was provided, no format check is required.
                if self.institution_doi.is_empty() {
                    self.institution.institution_doi.neq_assign(None);
                } else if let Ok(result) = self.institution_doi.parse::<Doi>() {
                    self.institution.institution_doi.neq_assign(Some(result));
                }
                // Only update the ROR value with the current user-entered string
                // if it is validly formatted - otherwise keep the database version.
                // If no ROR was provided, no format check is required.
                if self.ror.is_empty() {
                    self.institution.ror.neq_assign(None);
                } else if let Ok(result) = self.ror.parse::<Ror>() {
                    self.institution.ror.neq_assign(Some(result));
                }
                let body = UpdateInstitutionRequestBody {
                    variables: UpdateVariables {
                        institution_id: self.institution.institution_id,
                        institution_name: self.institution.institution_name.clone(),
                        institution_doi: self.institution.institution_doi.clone(),
                        ror: self.institution.ror.clone(),
                        country_code: self.institution.country_code.clone(),
                    },
                    ..Default::default()
                };
                let request = UpdateInstitutionRequest { body };
                self.push_institution = Fetch::new(request);
                self.link
                    .send_future(self.push_institution.fetch(Msg::SetInstitutionPushState));
                self.link
                    .send_message(Msg::SetInstitutionPushState(FetchAction::Fetching));
                false
            }
            Msg::SetInstitutionDeleteState(fetch_state) => {
                self.delete_institution.apply(fetch_state);
                match self.delete_institution.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.delete_institution {
                        Some(i) => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Deleted {}", i.institution_name),
                                NotificationStatus::Success,
                            )));
                            self.link.send_message(Msg::ChangeRoute(AppRoute::Admin(
                                AdminRoute::Institutions,
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
            Msg::DeleteInstitution => {
                let body = DeleteInstitutionRequestBody {
                    variables: DeleteVariables {
                        institution_id: self.institution.institution_id,
                    },
                    ..Default::default()
                };
                let request = DeleteInstitutionRequest { body };
                self.delete_institution = Fetch::new(request);
                self.link.send_future(
                    self.delete_institution
                        .fetch(Msg::SetInstitutionDeleteState),
                );
                self.link
                    .send_message(Msg::SetInstitutionDeleteState(FetchAction::Fetching));
                false
            }
            Msg::ChangeInstitutionName(institution_name) => self
                .institution
                .institution_name
                .neq_assign(institution_name.trim().to_owned()),
            Msg::ChangeInstitutionDoi(value) => {
                if self.institution_doi.neq_assign(value.trim().to_owned()) {
                    // If DOI is not correctly formatted, display a warning.
                    // Don't update self.institution.institution_doi yet, as user may later
                    // overwrite a new valid value with an invalid one.
                    self.institution_doi_warning.clear();
                    match self.institution_doi.parse::<Doi>() {
                        Err(e) => {
                            match e {
                                // If no DOI was provided, no warning is required.
                                ThothError::DoiEmptyError => {}
                                _ => self.institution_doi_warning = e.to_string(),
                            }
                        }
                        Ok(value) => self.institution_doi = value.to_string(),
                    }
                    true
                } else {
                    false
                }
            }
            Msg::ChangeRor(value) => {
                if self.ror.neq_assign(value.trim().to_owned()) {
                    // If ROR is not correctly formatted, display a warning.
                    // Don't update self.institution.ror yet, as user may later
                    // overwrite a new valid value with an invalid one.
                    self.ror_warning.clear();
                    match self.ror.parse::<Ror>() {
                        Err(e) => {
                            match e {
                                // If no ROR was provided, no warning is required.
                                ThothError::RorEmptyError => {}
                                _ => self.ror_warning = e.to_string(),
                            }
                        }
                        Ok(value) => self.ror = value.to_string(),
                    }
                    true
                } else {
                    false
                }
            }
            Msg::ChangeCountryCode(value) => self
                .institution
                .country_code
                .neq_assign(CountryCode::from_str(&value).ok()),
            Msg::ChangeRoute(r) => {
                let route = Route::from(r);
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.fetch_institution.as_ref().state() {
            FetchState::NotFetching(_) => html! {<Loader/>},
            FetchState::Fetching(_) => html! {<Loader/>},
            FetchState::Fetched(_body) => {
                let callback = self.link.callback(|event: FocusEvent| {
                    event.prevent_default();
                    Msg::UpdateInstitution
                });
                html! {
                    <>
                        <nav class="level">
                            <div class="level-left">
                                <p class="subtitle is-5">
                                    { "Edit institution" }
                                </p>
                            </div>
                            <div class="level-right">
                                <p class="level-item">
                                    <ConfirmDeleteComponent
                                        onclick=self.link.callback(|_| Msg::DeleteInstitution)
                                        object_name=self.institution.institution_name.clone()
                                    />
                                </p>
                            </div>
                        </nav>

                        { self.render_associated_works(&self.funded_works, "Funded: ") }

                        { self.render_associated_works(&self.affiliated_works, "Member(s) contributed to: ") }

                        <form onsubmit=callback>
                            <FormTextInput
                                label = "Institution Name"
                                value=self.institution.institution_name.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeInstitutionName(e.value))
                                required=true
                            />
                            <FormTextInputExtended
                                label = "Institution DOI"
                                statictext = DOI_DOMAIN
                                value=self.institution_doi.clone()
                                tooltip=self.institution_doi_warning.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeInstitutionDoi(e.value))
                            />
                            <FormTextInputExtended
                                label = "ROR ID"
                                statictext = ROR_DOMAIN
                                value=self.ror.clone()
                                tooltip=self.ror_warning.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangeRor(e.value))
                            />
                            <FormCountryCodeSelect
                                label = "Country"
                                value=self.institution.country_code.clone()
                                data=self.data.country_codes.clone()
                                onchange=self.link.callback(|event| match event {
                                    ChangeData::Select(elem) => {
                                        Msg::ChangeCountryCode(elem.value())
                                    }
                                    _ => unreachable!(),
                                })
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

impl InstitutionComponent {
    fn render_associated_works(&self, w: &[WorkWithRelations], explanatory_text: &str) -> Html {
        {
            if !w.is_empty() {
                html! {
                <div class="notification is-link">
                    {
                        for w.iter().map(|work| {
                            html! {
                                <p>
                                    { explanatory_text }
                                    <RouterAnchor<AppRoute>
                                        route=work.edit_route()
                                    >
                                        { &work.title }
                                    </  RouterAnchor<AppRoute>>
                                    { format!(", from: {}", work.imprint.publisher.publisher_name) }
                                </p>
                            }
                        })
                    }
                </div>
                }
            } else {
                html! {}
            }
        }
    }
}
