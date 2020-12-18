#[macro_use]
extern crate vst;

use vst::plugin::{Category, Info, Plugin};

#[derive(Default)]
struct Noiser;

impl Plugin for Noiser {
    fn get_info(&self) -> Info {
        Info {
            name: "Noiser".to_string(),

            unique_id: 1337,

            inputs: 2,

            outputs: 2,

            category: Category::Effect

            ..Default::default()
        }
    }
}

plugin_main!(Noiser);
