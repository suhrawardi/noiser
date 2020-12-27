#[macro_use]
extern crate vst;

use vst::plugin::{Category, Info, Plugin};
use vst::buffer::AudioBuffer;

#[derive(Default)]
struct Noiser {
    mvavg: u8
}

impl Plugin for Noiser {
    fn get_info(&self) -> Info {
        Info {
            name: "Noiser".to_string(),

            unique_id: 1337,

            inputs: 2,

            outputs: 2,

            category: Category::Effect,

            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_input_buffer, mut output_buffer) = buffer.split();

        self.mvavg = 0;

        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                *output_sample = (rand::random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}

plugin_main!(Noiser);
