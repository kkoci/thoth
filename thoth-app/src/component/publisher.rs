use thoth_api::account::model::AccountDetails;
use thoth_api::model::publisher::Publisher;
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
use crate::component::utils::FormTextInput;
use crate::component::utils::FormUrlInput;
use crate::component::utils::Loader;
use crate::models::publisher::delete_publisher_mutation::DeletePublisherRequest;
use crate::models::publisher::delete_publisher_mutation::DeletePublisherRequestBody;
use crate::models::publisher::delete_publisher_mutation::PushActionDeletePublisher;
use crate::models::publisher::delete_publisher_mutation::PushDeletePublisher;
use crate::models::publisher::delete_publisher_mutation::Variables as DeleteVariables;
use crate::models::publisher::publisher_query::FetchActionPublisher;
use crate::models::publisher::publisher_query::FetchPublisher;
use crate::models::publisher::publisher_query::PublisherRequest;
use crate::models::publisher::publisher_query::PublisherRequestBody;
use crate::models::publisher::publisher_query::Variables;
use crate::models::publisher::update_publisher_mutation::PushActionUpdatePublisher;
use crate::models::publisher::update_publisher_mutation::PushUpdatePublisher;
use crate::models::publisher::update_publisher_mutation::UpdatePublisherRequest;
use crate::models::publisher::update_publisher_mutation::UpdatePublisherRequestBody;
use crate::models::publisher::update_publisher_mutation::Variables as UpdateVariables;
use crate::route::AdminRoute;
use crate::route::AppRoute;
use crate::string::SAVE_BUTTON;

use super::ToOption;

pub struct PublisherComponent {
    publisher: Publisher,
    fetch_publisher: FetchPublisher,
    push_publisher: PushUpdatePublisher,
    delete_publisher: PushDeletePublisher,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
    notification_bus: NotificationDispatcher,
    props: Props,
}

pub enum Msg {
    SetPublisherFetchState(FetchActionPublisher),
    GetPublisher,
    SetPublisherPushState(PushActionUpdatePublisher),
    UpdatePublisher,
    SetPublisherDeleteState(PushActionDeletePublisher),
    DeletePublisher,
    ChangePublisherName(String),
    ChangePublisherShortname(String),
    ChangePublisherUrl(String),
    ChangeRoute(AppRoute),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub publisher_id: Uuid,
    pub current_user: AccountDetails,
}

impl Component for PublisherComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_publisher: FetchPublisher = Default::default();
        let push_publisher = Default::default();
        let delete_publisher = Default::default();
        let notification_bus = NotificationBus::dispatcher();
        let publisher: Publisher = Default::default();
        let router = RouteAgentDispatcher::new();

        link.send_message(Msg::GetPublisher);

        PublisherComponent {
            publisher,
            fetch_publisher,
            push_publisher,
            delete_publisher,
            link,
            router,
            notification_bus,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPublisherFetchState(fetch_state) => {
                self.fetch_publisher.apply(fetch_state);
                match self.fetch_publisher.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => {
                        self.publisher = match &body.data.publisher {
                            Some(c) => c.to_owned(),
                            None => Default::default(),
                        };
                        // If user doesn't have permission to edit this object, redirect to dashboard
                        if let Some(publishers) =
                            self.props.current_user.resource_access.restricted_to()
                        {
                            if !publishers.contains(&self.publisher.publisher_id.to_string()) {
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
            Msg::GetPublisher => {
                let body = PublisherRequestBody {
                    variables: Variables {
                        publisher_id: Some(self.props.publisher_id),
                    },
                    ..Default::default()
                };
                let request = PublisherRequest { body };
                self.fetch_publisher = Fetch::new(request);

                self.link
                    .send_future(self.fetch_publisher.fetch(Msg::SetPublisherFetchState));
                self.link
                    .send_message(Msg::SetPublisherFetchState(FetchAction::Fetching));
                false
            }
            Msg::SetPublisherPushState(fetch_state) => {
                self.push_publisher.apply(fetch_state);
                match self.push_publisher.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.update_publisher {
                        Some(p) => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Saved {}", p.publisher_name),
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
            Msg::UpdatePublisher => {
                let body = UpdatePublisherRequestBody {
                    variables: UpdateVariables {
                        publisher_id: self.publisher.publisher_id,
                        publisher_name: self.publisher.publisher_name.clone(),
                        publisher_shortname: self.publisher.publisher_shortname.clone(),
                        publisher_url: self.publisher.publisher_url.clone(),
                    },
                    ..Default::default()
                };
                let request = UpdatePublisherRequest { body };
                self.push_publisher = Fetch::new(request);
                self.link
                    .send_future(self.push_publisher.fetch(Msg::SetPublisherPushState));
                self.link
                    .send_message(Msg::SetPublisherPushState(FetchAction::Fetching));
                false
            }
            Msg::SetPublisherDeleteState(fetch_state) => {
                self.delete_publisher.apply(fetch_state);
                match self.delete_publisher.as_ref().state() {
                    FetchState::NotFetching(_) => false,
                    FetchState::Fetching(_) => false,
                    FetchState::Fetched(body) => match &body.data.delete_publisher {
                        Some(f) => {
                            self.notification_bus.send(Request::NotificationBusMsg((
                                format!("Deleted {}", f.publisher_name),
                                NotificationStatus::Success,
                            )));
                            self.link.send_message(Msg::ChangeRoute(AppRoute::Admin(
                                AdminRoute::Publishers,
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
            Msg::DeletePublisher => {
                let body = DeletePublisherRequestBody {
                    variables: DeleteVariables {
                        publisher_id: self.publisher.publisher_id,
                    },
                    ..Default::default()
                };
                let request = DeletePublisherRequest { body };
                self.delete_publisher = Fetch::new(request);
                self.link
                    .send_future(self.delete_publisher.fetch(Msg::SetPublisherDeleteState));
                self.link
                    .send_message(Msg::SetPublisherDeleteState(FetchAction::Fetching));
                false
            }
            Msg::ChangePublisherName(publisher_name) => self
                .publisher
                .publisher_name
                .neq_assign(publisher_name.trim().to_owned()),
            Msg::ChangePublisherShortname(value) => self
                .publisher
                .publisher_shortname
                .neq_assign(value.to_opt_string()),
            Msg::ChangePublisherUrl(value) => self
                .publisher
                .publisher_url
                .neq_assign(value.to_opt_string()),
            Msg::ChangeRoute(r) => {
                let route = Route::from(r);
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match self.fetch_publisher.as_ref().state() {
            FetchState::NotFetching(_) => html! {<Loader/>},
            FetchState::Fetching(_) => html! {<Loader/>},
            FetchState::Fetched(_body) => {
                let callback = self.link.callback(|event: FocusEvent| {
                    event.prevent_default();
                    Msg::UpdatePublisher
                });
                html! {
                    <>
                        <nav class="level">
                            <div class="level-left">
                                <p class="subtitle is-5">
                                    { "Edit publisher" }
                                </p>
                            </div>
                            <div class="level-right">
                                <p class="level-item">
                                    <ConfirmDeleteComponent
                                        onclick=self.link.callback(|_| Msg::DeletePublisher)
                                        object_name=self.publisher.publisher_name.clone()
                                    />
                                </p>
                            </div>
                        </nav>

                        <form onsubmit=callback>
                            <FormTextInput
                                label = "Publisher Name"
                                value=self.publisher.publisher_name.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangePublisherName(e.value))
                                required=true
                            />
                            <FormTextInput
                                label = "Publisher Short Name"
                                value=self.publisher.publisher_shortname.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangePublisherShortname(e.value))
                            />
                            <FormUrlInput
                                label = "Publisher URL"
                                value=self.publisher.publisher_url.clone()
                                oninput=self.link.callback(|e: InputData| Msg::ChangePublisherUrl(e.value))
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
