
extern crate dsp;
extern crate rms;
extern crate time_calc as time;


#[derive(Clone, Debug)]
pub struct Cuntpressor {
    channels: Vec<Channel>,
    interpolation_ms: time::calc::Ms,
    rms: rms::Rms,
    target_amp: f32,
}



fn init_channels(num_channels: usize) -> Vec<Channel> {
    (0..num_channels)
        .map(|_| Channel { maybe_prev_normaliser: None })
        .collect()
}


#[derive(Copy, Clone, Debug)]
pub struct Channel {
    /// The multiplier used to force the RMS to ~+/-1.0.
    maybe_prev_normaliser: Option<f32>,
}


impl Cuntpressor {

    /// Constructor for a new Cuntpressor.
    pub fn new() -> Cuntpressor {
        Cuntpressor {
            channels: Vec::new(),
            interpolation_ms: 10.0,
            rms: rms::Rms::new(0),
            target_amp: 1.0,
        }
    }

    /// The target amplitude that the RMS will be cuntpressed too.
    pub fn target_amp(mut self, amp: f32) -> Cuntpressor {
        self.target_amp = amp;
        self
    }

    /// Build the Cuntpressor with a given number of channels.
    pub fn channels(mut self, num_channels: usize) -> Cuntpressor {
        self.channels = init_channels(num_channels);
        self.rms = rms::Rms::new(num_channels);
        self
    }

    /// Build the Cuntpressor with a given interpolation duration in milliseconds.
    pub fn interpolation_ms(mut self, ms: time::calc::Ms) -> Cuntpressor {
        self.interpolation_ms = ms;
        self
    }

}



impl<S> dsp::Node<S> for Cuntpressor where S: dsp::Sample {

    fn audio_requested(&mut self, buffer: &mut [S], settings: dsp::Settings) {
        let Cuntpressor { ref mut channels, interpolation_ms, ref mut rms, target_amp } = *self;

        if channels.len() != (settings.channels as usize) {
            *channels = init_channels(settings.channels as usize);
            *rms = rms::Rms::new(settings.channels as usize);
        }

        rms.update(buffer, settings);

        let rms_per_channel = rms.per_channel().iter().map(|&rms| rms);
        let channels_with_indices = channels.iter_mut().enumerate();
        for ((channel_idx, channel), channel_rms) in channels_with_indices.zip(rms_per_channel) {

            // Determine the normaliser for the current channel_rms.
            let current_normaliser = if channel_rms > 0.0 {
                target_amp / channel_rms
            } else { 0.0 };

            match channel.maybe_prev_normaliser {

                // If the normaliser used for the previous buffer is different to the volume used
                // for the current buffer, we should interpolate from it to the current volume to
                // avoid clipping.
                Some(prev_normaliser) if prev_normaliser != current_normaliser
                    && interpolation_ms > 0.0 => {

                    // Calculate the interpolation duration in frames along with the volume increment
                    // to use for interpolation.
                    let interpolation_frames = ::std::cmp::min(
                        settings.frames as usize,
                        time::Ms(self.interpolation_ms).samples(settings.sample_hz as f64) as usize
                    );
                    let normaliser_diff = current_normaliser - prev_normaliser;
                    let normaliser_increment = normaliser_diff * (1.0 / interpolation_frames as f32);
                    let mut normaliser = prev_normaliser;

                    // Interpolated frames.
                    for frame in 0..interpolation_frames {
                        normaliser += normaliser_increment;
                        let idx = frame * (settings.channels as usize) + channel_idx;
                        let sample = &mut buffer[idx];
                        *sample = sample.mul_amp(normaliser);
                    }

                    // Remaining frames.
                    for frame in interpolation_frames..(settings.frames as usize) {
                        let idx = frame * (settings.channels as usize) + channel_idx;
                        let sample = &mut buffer[idx];
                        *sample = sample.mul_amp(normaliser);
                    }
                },

                // Otherwise, simply multiply every sample by the current volume.
                _ => for frame in 0..(settings.frames as usize) {
                    let idx = frame * (settings.channels as usize) + channel_idx;
                    let sample = &mut buffer[idx];
                    *sample = sample.mul_amp(current_normaliser);
                },

            }

            // Always set the current volume as the new `maybe_prev`.
            channel.maybe_prev_normaliser = Some(current_normaliser);
        }
    }

}

