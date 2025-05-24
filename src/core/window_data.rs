use super::{Dimensions, WindowData};
use leptos::prelude::*;
use leptos_use::core::Position;

impl WindowData {
    /// [`WindowData`] builder function \
    /// Sets the `icon`'s path for the window
    /// ```rust
    /// // Create a new default window with a custom icon
    /// let data = WindowData::default().icon("public/icon.svg");
    /// view! {
    ///     <Window data>
    ///         <h1>"This is a window with default settings"</h1>
    ///         <p>"Though its icon has been updated"</p>
    ///     </Window
    /// }
    /// ```
    pub fn icon(mut self, path: &'static str) -> Self {
        self.icon = path;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `title` for the window
    /// ```rust
    /// // Create a new default window with a custom title
    /// let data = WindowData::default().title("I'm Different");
    /// view! {
    ///     <Window data>
    ///         <h1>"Yeah, I'm Different"</h1>
    ///         <p>"Pull up to the scene with my ceiling missing"</p>
    ///     </Window
    /// }
    /// ```
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the [`Dimensions`] for the window `Default: (400,250)`
    /// ```rust
    ///
    /// // Default
    /// use grant_duchars_dev::data::WindowData;
    /// let data = WindowData::default();
    /// assert_equals!(Dimensions{ w: 400.0, h: 250.0 }, data.dimensions);
    ///
    /// // Setting the dimensions
    /// let data = WindowData::default().position(Dimensions{ w: 420.0, h: 69.0 });
    /// assert_equals!(Dimensions{ w: 420.0, h: 69.0 }, data.dimensions);
    /// ```
    pub fn dimensions(mut self, dim: Dimensions) -> Self {
        self.dimensions = dim;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `initial_position` for the window `Default: (0,0)`
    /// ```rust
    ///
    /// // Default
    /// use grant_duchars_dev::data::WindowData;
    /// let data = WindowData::default();
    /// assert_equals!(Position{ x: 0.0, y: 0.0 }, data.initial_position);
    ///
    /// // Setting the initial_position
    /// let data = WindowData::default().position(Position{ x: 420.0, y: 69.0 });
    /// assert_equals!(Position{ x: 420.0, y: 69.0 }, data.initial_position);
    /// ```
    pub fn position(mut self, pos: Position) -> Self {
        self.initial_position = pos;
        *self.set_position.write() = pos;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `initial_position` for the window to be centered in the browser
    /// ```rust
    /// // Create a new default window that is centered in the browser
    /// let data = WindowData::default().centered()
    /// view! {
    ///     <Window data>
    ///         <h1>"This is a window with default settings"</h1>
    ///         <p>"But hey, at least it starts centered"</p>
    ///     </Window
    /// }
    /// ```
    pub fn centered(mut self) -> Self {
        let window_center = expect_context::<ReadSignal<(f64, f64)>>().read_untracked();
        let Dimensions { w, h } = self.dimensions;
        let centered = Position {
            x: window_center.0 - w / 2.0,
            y: window_center.1 - h / 2.0,
        };
        self.initial_position = centered;
        *self.set_position.write() = centered;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets whether the window starts open or closed `Default: true`
    /// ```rust
    ///
    /// // Default
    /// use grant_duchars_dev::data::WindowData;
    /// let data = WindowData::default();
    /// assert_equals!(true, data.is_open.get());
    ///
    /// // Setting is_open
    /// let data = WindowData::default().open(false);
    /// assert_equals!(false, data.is_open.get());
    /// ```
    pub fn open(self, is_open: bool) -> Self {
        self.is_open.set(is_open);
        self
    }

    /// [`WindowData`] builder function \
    /// Sets whether the window starts minimized or not `Default: false`
    /// ```rust
    ///
    /// // Default
    /// use grant_duchars_dev::data::WindowData;
    /// let data = WindowData::default();
    /// assert_equals!(false, data.is_minimized.get());
    ///
    /// // Setting is_minimized
    /// let data = WindowData::default().minimized(true);
    /// assert_equals!(true, data.is_minimized.get());
    /// ```
    pub fn minimized(self, is_minimized: bool) -> Self {
        self.is_minimized.set(is_minimized);
        self
    }
}
