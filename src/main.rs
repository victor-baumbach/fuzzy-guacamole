use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex, Padding, Align};

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("Top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("Bottom left")), 1.0),
            1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("Top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new("Bottom right")), 1.0),
            1.0))
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}
