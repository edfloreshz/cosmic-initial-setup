use cosmic::{
    Apply, Element, Task,
    cosmic_config::{Config, CosmicConfigEntry},
    cosmic_theme::{self, ThemeMode},
    iced::{Alignment, Length},
    theme, widget,
};

use crate::{fl, page};

static COSMIC_DARK_SVG: &'static [u8] = include_bytes!("../../res/cosmic-dark.svg");
static COSMIC_LIGHT_SVG: &'static [u8] = include_bytes!("../../res/cosmic-light.svg");

struct Theme {
    name: String,
    handle: widget::svg::Handle,
}

#[derive(Clone, Debug)]
pub enum Message {
    Select(usize),
}

impl From<Message> for super::Message {
    fn from(message: Message) -> Self {
        super::Message::Appearance(message)
    }
}

pub struct Page {
    theme_mode_config: Option<Config>,
    theme_mode: ThemeMode,
    themes: Vec<Theme>,
    selected: usize,
}

impl Page {
    pub fn new() -> Self {
        let mut theme_mode = ThemeMode::default();
        let theme_mode_config = match ThemeMode::config() {
            Ok(config) => {
                match ThemeMode::get_entry(&config) {
                    Ok(entry) => {
                        theme_mode = entry;
                    }
                    Err((err, entry)) => {
                        tracing::warn!(?err, "errors while loading theme mode");
                        theme_mode = entry;
                    }
                }
                Some(config)
            }
            Err(err) => {
                tracing::warn!(err = err.to_string(), "failed to get theme mode config");
                None
            }
        };

        Self {
            theme_mode_config,
            theme_mode,
            themes: vec![
                Theme {
                    name: "COSMIC dark".to_string(),
                    handle: widget::svg::Handle::from_memory(COSMIC_DARK_SVG),
                },
                Theme {
                    name: "COSMIC light".to_string(),
                    handle: widget::svg::Handle::from_memory(COSMIC_LIGHT_SVG),
                },
            ],
            selected: if theme_mode.is_dark { 0 } else { 1 },
        }
    }

    pub fn update(&mut self, message: Message) -> Task<page::Message> {
        match message {
            Message::Select(index) => {
                if let Some(config) = &self.theme_mode_config {
                    match self.theme_mode.set_is_dark(config, index == 0) {
                        Ok(_) => {
                            //TODO: read current config from disk, do not track here
                            self.selected = index;
                        }
                        Err(err) => {
                            tracing::warn!(err = err.to_string(), "failed to set theme mode");
                        }
                    }
                }
            }
        }

        Task::none()
    }
}

impl page::Page for Page {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn title(&self) -> String {
        fl!("appearance-page")
    }

    fn skippable(&self) -> bool {
        true
    }

    fn view(&self) -> Element<page::Message> {
        let cosmic_theme::Spacing {
            space_xxs,
            space_m,
            space_xl,
            ..
        } = theme::active().cosmic().spacing;

        let mut grid = widget::grid().column_spacing(space_m).row_spacing(space_m);
        for (i, theme) in self.themes.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                grid = grid.insert_row();
            }

            let thumbnail = widget::svg(theme.handle.clone()).width(144).height(81);

            let button = widget::button::custom_image_button(thumbnail, None)
                .class(theme::Button::Image)
                .selected(i == self.selected)
                .on_press(Message::Select(i).into());

            let selection = widget::column::with_capacity(2)
                .push(button)
                .push(widget::text::body(&theme.name))
                .spacing(space_xxs)
                .align_x(Alignment::Center);

            grid = grid.push(selection);
        }

        let description = widget::text::body(fl!("appearance-page", "description"))
            .align_x(cosmic::iced::Alignment::Center)
            .apply(widget::container)
            .width(Length::Fill);

        widget::column::with_capacity(2)
            .push(grid)
            .push(description)
            .align_x(Alignment::Center)
            .spacing(space_xl)
            .into()
    }
}
