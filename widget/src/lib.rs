use schemars::{schema_for, JsonSchema};
use serde::Deserialize;
use serde_json::{json, Value};

use wg_display_widget::{export, Guest, WidgetContext, WidgetResult};

const WIDGET_NAME: &str = "Tap Test";

// test for the touch support for the wg display

#[derive(JsonSchema, Deserialize)]
struct WidgetConfig {
    /// Title shown in the widget
    #[serde(default = "default_title")]
    title: String,
}

fn default_title() -> String {
    "Tap Test".into()
}

struct MyWidget;

impl Guest for MyWidget {
    fn get_name() -> String {
        WIDGET_NAME.into()
    }

    fn run(context: WidgetContext) -> WidgetResult {
        let Ok(config) = serde_json::from_str::<WidgetConfig>(&context.config) else {
            return WidgetResult {
                data: "Invalid config".into(),
            };
        };

        // get `_state`: last output
        // and `_input`: id of tap element that triggered this run, otherwise empty
        let extra: Value = serde_json::from_str(&context.config).unwrap_or_default();
        let mut taps: u32 = extra["_state"]
            .as_str()
            .and_then(|state| state.parse().ok())
            .unwrap_or(0);
        if extra["_input"]["tap"] == "btn" {
            taps += 1;
        }

        WidgetResult {
            data: render(&config.title, taps),
        }
    }

    fn get_config_schema() -> String {
        let schema = schema_for!(WidgetConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn get_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn get_run_update_cycle_seconds() -> u32 {
        10
    }
}

/// Tab button to increase counter
fn render(title: &str, taps: u32) -> String {
    json!({
        "state": taps.to_string(),
        "elements": [
            { "type": "text", "x": 0.08, "y": 0.05, "w": 0.84, "h": 0.2, "text": title,
              "font_size": 32, "font_weight": 700, "color": 0xFFFFFFFFu32,
              "horizontal_alignment": "center", "vertical_alignment": "center" },
            { "type": "rect", "x": 0.1, "y": 0.35, "w": 0.8, "h": 0.5,
              "background": 0xFF0089FAu32, "border_color": 0xFFFFFFFFu32, "border_width": 2,
              "border_radius": 20 },
            { "type": "text", "x": 0.1, "y": 0.35, "w": 0.8, "h": 0.5,
              "text": format!("Taps: {taps}"),
              "font_size": 48, "font_weight": 700, "color": 0xFFFFFFFFu32,
              "horizontal_alignment": "center", "vertical_alignment": "center" },
            { "type": "tap", "x": 0.1, "y": 0.35, "w": 0.8, "h": 0.5, "id": "btn" },
        ]
    })
    .to_string()
}

export!(MyWidget);
