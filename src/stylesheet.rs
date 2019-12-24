use cggeom::box2;
use tcw3::{
    stylesheet,
    images::himg_from_rounded_rect,
    ui::theming::{Manager, Metrics, Stylesheet, Role},
};

/// Define styling ID values.
pub mod elem_id {
    use tcw3::ui::theming::ClassSet;

    pub const WRAPPER: ClassSet = ClassSet::id(1);
    pub const LABEL_WRAP: ClassSet = ClassSet::id(2);
    pub const LABEL: ClassSet = ClassSet::id(3);
    pub const RED: ClassSet = ClassSet::id(4);
}

fn new_custom_stylesheet() -> impl Stylesheet {
    // Import IDs (e.g., `#WRAPPER`) into the scope
    use self::elem_id::*;

    use std::f32::NAN;
    const BUTTON_SIZE: [f32; 2] = [37.0, 37.0];
    const BUTTON_MARGIN: f32 = 3.0;

    stylesheet! {
        ([.BUTTON]) (priority = 100) {
            min_size: BUTTON_SIZE.into(),
            subview_metrics[Role::Generic]: Metrics {
                margin: [NAN; 4],
                .. Metrics::default()
            },

            num_layers: 2,
            layer_img[0]: None,
            layer_img[1]: Some(himg_from_rounded_rect(
                [0.65, 0.65, 0.65, 1.0].into(),
                [[(BUTTON_SIZE[0] - BUTTON_MARGIN) * 0.5; 2]; 4]
            )),
            layer_center[1]: box2! { point: [0.5, 0.5] },
            layer_opacity[1]: 0.8,
            layer_metrics[1]: Metrics {
                margin: [BUTTON_MARGIN; 4],
                .. Metrics::default()
            },
        },
        ([#RED.BUTTON]) (priority = 200) {
            layer_img[1]: Some(himg_from_rounded_rect(
                [0.96, 0.64, 0.15, 1.0].into(),
                [[(BUTTON_SIZE[0] - BUTTON_MARGIN) * 0.5; 2]; 4]
            )),
        },
        ([.BUTTON.HOVER]) (priority = 200) {
            layer_opacity[1]: 1.0,
        },
        ([.BUTTON.ACTIVE]) (priority = 200) {
            layer_img[0]: None,
            layer_opacity[1]: 0.6,
        },

        // Button label
        ([] < [.BUTTON.ACTIVE]) (priority = 200) {
            fg_color: [0.0, 0.0, 0.0, 1.0].into(),
        },

        ([#WRAPPER]) (priority = 100) {
            num_layers: 2,

            layer_bg_color[0]: [0.1, 0.1, 0.1, 0.4].into(),
            layer_metrics[0]: Metrics {
                // Extend to fill the titlebar
                margin: [-100.0, 0.0, 0.0, 0.0],
                .. Metrics::default()
            },

            layer_bg_color[1]: [0.1, 0.1, 0.1, 0.4].into(),
            layer_metrics[1]: Metrics {
                margin: [-100.0, 0.0, NAN, 0.0],
                size: [NAN, 100.0 + 42.0].into(),
            },

            subview_metrics[Role::Generic]: Metrics {
                margin: [6.0, 0.0, 4.0, 0.0],
                .. Metrics::default()
            },
        },

        ([#LABEL_WRAP]) (priority = 100) {
            num_layers: 1,
            subview_metrics[Role::Generic]: Metrics {
                margin: [8.0, NAN, 12.0, 10.0],
                .. Metrics::default()
            },
        },

        ([#LABEL]) (priority = 100) {
            fg_color: [1.0; 4].into(),
        },
    }
}

pub fn register_stylesheet(manager: &'static Manager) {
    manager.subscribe_new_sheet_set(Box::new(move |_, _, ctx| {
        ctx.insert_stylesheet(new_custom_stylesheet());
    }));
    manager.update_sheet_set();
}
