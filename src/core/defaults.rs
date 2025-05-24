use super::{DesktopItem, DesktopItems, Dimensions, WindowData};
use leptos::{html, prelude::*};
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

impl Default for Dimensions {
    fn default() -> Self {
        Self { w: 400.0, h: 250.0 }
    }
}

impl Default for WindowData {
    fn default() -> Self {
        let node_ref = NodeRef::<html::Div>::new();
        let UseDraggableReturn {
            position,
            set_position,
            ..
        } = use_draggable_with_options(
            node_ref,
            UseDraggableOptions::default().prevent_default(true),
        );
        Self {
            icon: "",
            title: "Window",
            node_ref,
            dimensions: Default::default(),
            position,
            set_position,
            initial_position: Default::default(),
            is_open: RwSignal::new(true),
            is_minimized: RwSignal::new(false),
            desktop_item: true,
        }
    }
}

impl Default for DesktopItems {
    fn default() -> Self {
        Self::from(vec![
            DesktopItem::new(
                "public/github-mark-white.svg",
                "Github",
                "https://github.com/Grant-Duchars",
            ),
            DesktopItem::new(
                "public/LI-In-Bug.png",
                "LinkedIn",
                "https://www.linkedin.com/in/grant-duchars/",
            ),
        ])
    }
}
