use super::{DesktopItem, DesktopItems, WindowData};
use leptos::prelude::*;
use std::ops::{Deref, DerefMut};

impl IntoIterator for DesktopItems {
    type Item = DesktopItem;
    type IntoIter = <Vec<DesktopItem> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for DesktopItems {
    type Target = Vec<DesktopItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DesktopItems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<WindowData> for DesktopItem {
    fn from(window: WindowData) -> Self {
        let WindowData {
            icon,
            title,
            is_open,
            ..
        } = window;
        Self {
            icon,
            title,
            func: is_open.into(),
        }
    }
}

/// Sum type for desktop item functions \
/// Created from a `RwSignal<bool>` or `&'static str`
#[derive(Clone, Copy)]
pub enum DesktopItemFunction {
    Window(RwSignal<bool>),
    ExternalLink(&'static str),
}

impl From<RwSignal<bool>> for DesktopItemFunction {
    fn from(window: RwSignal<bool>) -> Self {
        Self::Window(window)
    }
}
impl From<&'static str> for DesktopItemFunction {
    fn from(link: &'static str) -> Self {
        Self::ExternalLink(link)
    }
}
