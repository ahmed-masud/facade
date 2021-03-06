//! This module defines aliases to widget types.

/*
mod bind;
pub use bind::BindWidget as Bind;

mod blank;
pub use blank::BlankWidget as Blank;

mod button;
pub use button::ButtonWidget as Button;

mod control;
pub use control::ControlWidget as Control;

mod dashboard;
pub use dashboard::DashboardWidget as Dashboard;

mod dynamic;
pub use dynamic::DynamicWidget as Dynamic;

mod fixed;
pub use fixed::FixedWidget as Fixed;

mod page;
pub use page::PageWidget as Page;

mod panel;
pub use panel::PanelWidget as Panel;

mod welcome;
pub use welcome::WelcomeWidget as Welcome;
*/

mod app;
pub use app::AppWidget as App;

mod app_bar;
pub use app_bar::AppBarWidget as AppBar;

mod container;
pub use container::ContainerWidget as Container;

mod component;
pub use component::ComponentWidget as Component;

mod footer;
pub use footer::FooterWidget as Footer;

mod icon;
pub use icon::Icon;

mod row;
pub use row::RowWidget as Row;

mod list;
pub use list::ListWidget as List;

mod navigation_drawer;
pub use navigation_drawer::NavigationDrawerWidget as NavigationDrawer;

mod scene;
pub use scene::SceneWidget as Scene;

mod spinner;
pub use spinner::SpinnerWidget as Spinner;

mod widget;
pub use widget::{Reqs, View, Widget, WidgetModel};
