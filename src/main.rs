use log::debug;
use tcw3::{
    pal,
    pal::prelude::*,
    ui::{layouts::TableLayout, theming::ClassSet, AlignFlags},
    uicore::{HWnd, WndListener, WndStyleFlags},
};

mod stylesheet;
use self::stylesheet::elem_id;

tcw3_calc_meta::designer_impl! { crate::MainView }
tcw3_calc_meta::designer_impl! { crate::CalcButton }

impl MainView {
    fn handle_op(&self, op: char) {
        let mut calc_state = self.calc_state();
        calc_state.op(op);
        self.set_calc_state(calc_state);
    }
}

/// The calculator logic around `CalcState` was taken from [Druid]'s [`calc.rs`]
/// example. The original code is owned by The xi-editor Authors and licensed
/// under the Apache License, Version 2.0. The license text can be found in
/// `LICENSE-apache`.
///
/// [Druid]: https://github.com/xi-editor/druid
/// [`calc.rs`]: https://github.com/xi-editor/druid/blob/37d49a364d3baf37c06542a9f9db4b582f21e21f/druid/examples/calc.rs
#[derive(Clone, PartialEq)]
struct CalcState {
    value: String,
    operand: f64,
    operator: char,
    in_num: bool,
}

impl CalcState {
    fn new() -> Self {
        Self {
            value: "0".to_string(),
            operand: 0.0,
            operator: 'C',
            in_num: false,
        }
    }

    fn display(&mut self) {
        // TODO: change hyphen-minus to actual minus
        self.value = self.operand.to_string();
    }

    fn compute(&mut self) {
        if self.in_num {
            let operand2 = self.value.parse().unwrap_or(0.0);
            let result = match self.operator {
                '+' => Some(self.operand + operand2),
                '−' => Some(self.operand - operand2),
                '×' => Some(self.operand * operand2),
                '÷' => Some(self.operand / operand2),
                _ => None,
            };
            if let Some(result) = result {
                self.operand = result;
                self.display();
                self.in_num = false;
            }
        }
    }

    fn op(&mut self, op: char) {
        match op {
            '0'..='9' => {
                if !self.in_num {
                    self.value.clear();
                    self.in_num = true;
                }
                self.value.push(op);
            }
            '+' | '−' | '×' | '÷' | '=' => {
                self.compute();
                self.operand = self.value.parse().unwrap_or(0.0);
                self.operator = op;
                self.in_num = false;
            }
            '±' => {
                if self.in_num {
                    if self.value.starts_with('−') {
                        self.value = self.value[3..].to_string();
                    } else {
                        self.value = ["−", &self.value].concat();
                    }
                } else {
                    self.operand = -self.operand;
                    self.display();
                }
            }
            '.' => {
                if !self.in_num {
                    self.value = "0".to_string();
                    self.in_num = true;
                }
                if self.value.find('.').is_none() {
                    self.value.push('.');
                }
            }
            'c' => {
                self.value = "0".to_string();
                self.in_num = false;
            }
            'C' => {
                self.value = "0".to_string();
                self.operator = 'C';
                self.in_num = false;
            }
            '⌫' => {
                if self.in_num {
                    self.value.pop();
                    if self.value.is_empty() || self.value == "−" {
                        self.value = "0".to_string();
                        self.in_num = false;
                    }
                }
            }
            _ => unreachable!(),
        }
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
    stylesheet::register_stylesheet(style_manager);

    let wnd = HWnd::new(wm);
    wnd.set_visibility(true);
    wnd.set_listener(MyWndListener);
    wnd.set_style_flags(WndStyleFlags::default() | WndStyleFlags::TRANSPARENT_BACKDROP_BLUR);

    if cfg!(target_os = "macos") {
        wnd.set_caption("");
    } else {
        wnd.set_caption("Calculator");
    }

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
