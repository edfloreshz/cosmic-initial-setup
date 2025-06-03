use std::any::{Any, TypeId};

use cosmic::{Element, widget};
use indexmap::IndexMap;

pub mod appearance;
pub mod keyboard;
pub mod language;
pub mod launcher;
pub mod layout;
pub mod location;
pub mod new_apps;
pub mod new_shortcuts;
pub mod user;
pub mod welcome;
pub mod wifi;
pub mod workflow;

pub enum AppMode {
    NewInstall {
        create_user: bool,
    },
    /// Transitioned from GNOME.
    GnomeTransition,
}

#[inline]
pub fn pages(mode: AppMode) -> IndexMap<TypeId, Box<dyn Page>> {
    let mut pages: IndexMap<TypeId, Box<dyn Page>> = IndexMap::new();
    pages.insert(
        TypeId::of::<welcome::Page>(),
        Box::new(welcome::Page::new()),
    );

    if matches!(mode, AppMode::NewInstall { .. }) {
        pages.insert(
            TypeId::of::<language::Page>(),
            Box::new(language::Page::new()),
        );

        pages.insert(
            TypeId::of::<location::Page>(),
            Box::new(location::Page::new()),
        );

        pages.insert(
            TypeId::of::<keyboard::Page>(),
            Box::new(keyboard::Page::new()),
        );
    }

    if matches!(mode, AppMode::NewInstall { create_user: true }) {
        pages.insert(TypeId::of::<user::Page>(), Box::new(user::Page::default()));
    }

    pages.insert(
        TypeId::of::<appearance::Page>(),
        Box::new(appearance::Page::new()),
    );

    pages.insert(
        TypeId::of::<layout::Page>(),
        Box::new(layout::Page::default()),
    );

    if matches!(mode, AppMode::GnomeTransition) {
        pages.insert(
            TypeId::of::<new_apps::Page>(),
            Box::new(new_apps::Page::default()),
        );
    }

    pages.insert(
        TypeId::of::<workflow::Page>(),
        Box::new(workflow::Page::default()),
    );

    if matches!(mode, AppMode::GnomeTransition) {
        pages.insert(
            TypeId::of::<new_shortcuts::Page>(),
            Box::new(new_shortcuts::Page::default()),
        );
    } else {
        pages.insert(
            TypeId::of::<launcher::Page>(),
            Box::new(launcher::Page::new()),
        );
    }

    if matches!(mode, AppMode::NewInstall { .. }) {
        pages.insert(TypeId::of::<wifi::Page>(), Box::new(wifi::Page::default()));
    }

    pages
}

#[derive(Clone, Debug)]
pub enum Message {
    Appearance(appearance::Message),
    Keyboard(keyboard::Message),
    Language(language::Message),
    Layout(layout::Message),
    Location(location::Message),
    User(user::Message),
    Welcome(welcome::Message),
    WiFi(wifi::Message),
    Todo,
}

impl From<Message> for super::Message {
    fn from(message: Message) -> Self {
        super::Message::PageMessage(message)
    }
}

pub trait Page {
    fn as_any(&mut self) -> &mut dyn Any;

    fn title(&self) -> String;

    fn init(&mut self) -> cosmic::Task<Message> {
        cosmic::Task::none()
    }

    fn apply_settings(&mut self) -> cosmic::Task<Message> {
        cosmic::Task::none()
    }

    fn open(&mut self) -> cosmic::Task<Message> {
        cosmic::Task::none()
    }

    fn width(&self) -> f32 {
        480.0
    }

    fn completed(&self) -> bool {
        true
    }

    fn skippable(&self) -> bool {
        false
    }

    fn dialog(&self) -> Option<Element<Message>> {
        None
    }

    fn view(&self) -> Element<Message> {
        widget::text::body("TODO").into()
    }
}
