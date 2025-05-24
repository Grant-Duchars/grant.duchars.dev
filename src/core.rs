pub use desktop_items::DesktopItemFunction;
use leptos::{html, prelude::*};
use leptos_use::core::Position;

mod defaults;
mod desktop_items;
mod window_data;

/// Semantic struct for dimension data
#[derive(Clone, Copy)]
pub struct Dimensions {
    /// `width`
    pub w: f64,
    /// `height`
    pub h: f64,
}

/// Stores data about the window and reactive signals to control the window
#[derive(Clone, Copy)]
pub struct WindowData {
    /// static str with icon path
    pub icon: &'static str,
    /// static str with title
    pub title: &'static str,
    /// NodeRef to window bar div
    pub node_ref: NodeRef<html::Div>,
    /// dimensions of the window
    pub dimensions: Dimensions,
    /// reactive position of the window
    pub position: Signal<Position>,
    /// setter for the window position
    pub set_position: WriteSignal<Position>,
    /// initial starting position of the window
    pub initial_position: Position,
    /// reactive read & write signal for whether the window is open
    pub is_open: RwSignal<bool>,
    /// reactive read & write signal for whether the window is minimized
    pub is_minimized: RwSignal<bool>,
    /// whether the window should create a desktop item
    pub desktop_item: bool,
}

impl WindowData {
    /// Creates a new [`WindowData`] \
    /// You can use the various builder functions to customize the window
    /// ```rust
    /// // Creating a new window
    /// let data = WindowData::new("public/icon.svg", "My Custom Window")
    ///     .dimensions(Dimensions{ w: 1280, h: 720 })
    ///     .centered();
    /// view! {
    ///     <Window data>
    ///         <h1>"This is the new window"</h1>
    ///         <p>"You can put your content here"<p>
    ///     </Window>
    /// }
    /// ```
    pub fn new(icon: &'static str, title: &'static str) -> Self {
        Self {
            icon,
            title,
            ..Default::default()
        }
    }

    /// Resets the window's position to its `initial_position`
    pub fn reset_position(&self) {
        *self.set_position.write() = self.initial_position;
    }
}

/// Vec wrapper type for [`DesktopItem`]s \
/// Used for simplifying type signatures and for implementing a default set of items
#[derive(Clone)]
pub struct DesktopItems(Vec<DesktopItem>);
impl From<Vec<DesktopItem>> for DesktopItems {
    fn from(items: Vec<DesktopItem>) -> Self {
        Self(items)
    }
}

/// Stores data about desktop items and has function depending on whether its a window item or an external link
#[derive(Clone, Copy)]
pub struct DesktopItem {
    pub icon: &'static str,
    pub title: &'static str,
    pub func: DesktopItemFunction,
}

impl DesktopItem {
    pub fn new(
        icon: &'static str,
        title: &'static str,
        func: impl Into<DesktopItemFunction>,
    ) -> Self {
        let func = func.into();
        Self { icon, title, func }
    }
}
