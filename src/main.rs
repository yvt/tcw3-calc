use std::cell::Cell;
use log::debug;
use tcw3::{
    pal,
    pal::prelude::*,
    uicore::{HWnd, WndListener},
    ui::{AlignFlags, layouts::TableLayout},
};

tcw3_calc_meta::designer_impl! { crate::MainView }
tcw3_calc_meta::designer_impl! { crate::CalcButton }

impl MainView {
    fn handle_op(&self, op: char) {
        dbg!(op);
    }
}

struct MyWndListener;

impl WndListener for MyWndListener {
    fn close(&self, wm: pal::Wm, _: &HWnd) {
        wm.terminate();
    }
}

fn main() {
    simple_logger::init().unwrap();

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
