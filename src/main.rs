use log::debug;
use tcw3::{
    pal,
    pal::prelude::*,
    uicore::{HWnd, WndListener},
};

tcw3_calc_meta::designer_impl! {
    crate::MainView
}

struct MyWndListener;

impl WndListener for MyWndListener {
    fn close(&self, wm: pal::Wm, _: &HWnd) {
        wm.terminate();
    }
}

fn main() {
    // Enable logging only in debug builds
    #[cfg(debug_assertions)]
    {
        env_logger::init();
    }

    debug!("Initializing WM");
    let wm = pal::Wm::global();

    // Register the application's custom stylesheet
    let style_manager = tcw3::ui::theming::Manager::global(wm);
    // TODO

    let wnd = HWnd::new(wm);
    wnd.set_visibility(true);
    wnd.set_listener(MyWndListener);

    // Create and attach the main view
    let main_view = MainViewBuilder::new()
        .with_wm(wm)
        .with_style_manager(style_manager)
        .build();

    use tcw3::ui::layouts::FillLayout;
    wnd.content_view()
        .set_layout(FillLayout::new(main_view.view().clone()));

    debug!("Entering the main loop");
    wm.enter_main_loop();
}
