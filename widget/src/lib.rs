use schemars::{schema_for, JsonSchema};
use serde::Deserialize;


wit_bindgen::generate!({
    path: "../wg_display_widget_wit/wit",
    world: "widget"
});

const WIDGET_NAME: &str = "ESP32 Widget Template";

#[derive(JsonSchema, Deserialize)]
struct WidgetConfig {
    output: String,
}

struct MyWidget;

impl Guest for MyWidget {
    fn get_name() -> String {
        WIDGET_NAME.into()
    }

    fn run(context: WidgetContext) -> WidgetResult {
        let config: WidgetConfig =
            serde_json::from_str(&context.config).expect("Failed to parse config");
        WidgetResult { data: config.output }
    }

    fn get_config_schema() -> String  {
        let schema = schema_for!(WidgetConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn get_version() -> String {
        "0.1.0-interface-test".into()
    }

    fn get_run_update_cycle_seconds() -> u32 {
        10
    }
}

export!(MyWidget);
