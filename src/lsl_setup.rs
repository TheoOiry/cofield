use chrono::{DateTime, Local};
use lsl::{ExPushable, StreamInfo, StreamOutlet};

use crate::output::OutputRow;

const NUMBER_OF_CHANNELS: u32 = 10;
const MAX_BUFFERED_SECONDS: i32 = 60 * 6;
const NOMINAL_SRATE: f64 = 50.0;
const CHUNK_SIZE: i32 = 5;

pub fn setup_stream_outlet() -> anyhow::Result<StreamOutlet> {
    let info = setup_stream_infos()?;

    Ok(lsl::StreamOutlet::new(
        &info,
        CHUNK_SIZE,
        MAX_BUFFERED_SECONDS,
    )?)
}

pub fn setup_stream_infos() -> anyhow::Result<StreamInfo> {
    let mut info = lsl::StreamInfo::new(
        "HandData",
        "MoCap",
        NUMBER_OF_CHANNELS,
        NOMINAL_SRATE,
        lsl::ChannelFormat::Int16,
        "cofield_glove",
    )?;

    let mut channels = info.desc().append_child("channels");

    for i in 1..=5 {
        channels
            .append_child("channel")
            .append_child_value("label", &format!("Finger{}", i))
            .append_child_value("object", "FigersMouvement");
    }

    for i in 1..=5 {
        channels
            .append_child("channel")
            .append_child_value("label", &format!("FingerVibration{}", i))
            .append_child_value("object", "FigersVibration");
    }

    Ok(info)
}

impl ExPushable<OutputRow<'_>> for StreamOutlet {
    fn push_sample_ex(
        &self,
        data: &OutputRow,
        _timestamp: f64,
        pushthrough: bool,
    ) -> Result<(), lsl::Error> {
        let mut payload = data.notification.flex_values.0.map(|v| v as i16).to_vec();
        payload.extend(data.vibration_state.iter().map(|v| *v as i16));

        let timestamp = synchronize_lsl_time(data.notification.dt);

        self.push_sample_ex(&payload, timestamp, pushthrough)
    }
}

fn synchronize_lsl_time(datetime: DateTime<Local>) -> f64 {
    let lsl_time = lsl::local_clock();

    let unix_time =
        datetime.timestamp() as f64 + datetime.timestamp_subsec_micros() as f64 / 1_000_000.0;

    lsl_time - unix_time
}
