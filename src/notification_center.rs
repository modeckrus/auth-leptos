use leptos::{IntoView, RwSignal, SignalUpdate};
use std::sync::Arc;

use leptos::logging::*;
use leptos::*;

#[component]
pub fn NotificationCenterBuilder() -> impl IntoView {
    view! {
        <div class="bottom-0 top-auto fixed right-0 left-auto gap-2">
            <InternalNotificationCenter/>
        </div>
    }
}

#[component]
fn InternalNotificationCenter() -> impl IntoView {
    let center = NotificationCenter::new();
    provide_context(center.clone());
    view! {
        {move || {
            center
                .notifications
                .get()
                .into_iter()
                .map(|n| view! { <Notification notification=n.clone()/> })
                .collect_view()
        }}
    }
}

#[component]
pub fn Notification(notification: Notification) -> impl IntoView {
    let status = notification.status;
    let data = notification.data;
    view! {
        <div class="alert alert-error shadow-lg mt-2">
            <div>

                {match data {
                    NotificationData::Error(s) => {
                        view! {
                            <div>
                                <span class="font-bold">"Error:"</span>
                                <span>" "</span>
                                <span>{s}</span>
                            </div>
                        }
                    }
                    NotificationData::Info(s) => {
                        view! {
                            <div>
                                <span class="font-bold">"Info:"</span>
                                <span>" "</span>
                                <span>{s}</span>
                            </div>
                        }
                    }
                    NotificationData::View(v) => {
                        view! { <div>{&*v}</div> }
                    }
                }}

            </div>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NotificationCenter {
    pub notifications: RwSignal<Vec<Notification>>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: RwSignal::new(vec![]),
        }
    }

    pub fn error(&self, s: impl Into<String>) {
        let notification = Notification::error(s);
        let id = notification.id;
        self.notifications.update(|n| n.push(notification));
        let s = self.clone();
        leptos::spawn_local(async move {
            crate::c::sleep(std::time::Duration::from_secs(5)).await;
            s.remove(id);
        })
    }

    pub fn info(&self, s: impl Into<String>) {
        self.notifications.update(|n| n.push(Notification::info(s)));
    }

    pub fn view(&self, view: impl IntoView) {
        self.notifications
            .update(|n| n.push(Notification::view(view)));
    }

    pub fn remove(&self, id: i32) {
        self.notifications.update(|n| n.retain(|n| n.id != id));
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notification {
    pub id: i32,
    pub data: NotificationData,
    pub status: RwSignal<NotificationStatus>,
}

impl Notification {
    pub fn new(data: NotificationData) -> Self {
        Self {
            id: rand::random(),
            data,
            status: RwSignal::new(NotificationStatus::Appearing),
        }
    }

    pub fn error(s: impl Into<String>) -> Self {
        Notification::new(NotificationData::Error(s.into()))
    }

    pub fn info(s: impl Into<String>) -> Self {
        Notification::new(NotificationData::Info(s.into()))
    }

    pub fn view(view: impl IntoView) -> Self {
        Notification::new(NotificationData::View(Arc::new(view.into_view())))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationStatus {
    Appearing,
    Idle,
    Disappearing,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotificationData {
    Error(String),
    Info(String),
    View(Arc<leptos::View>),
}
