use dora_node_api::{
    arrow::array::{Float32Array, Float64Array, Int32Array, Int64Array},
    arrow::datatypes::Datatype::{Float32, Float64, Int32, Int64},
    dora_core::config::DataId,
    ArrowData,
};
use eyre::{Context, ContextCompat, Result};
use rerun::RecordingStream;

pub fn update_series(rec: &RecordingStream, id: DataId, data: ArrowData) -> Result<()> {
    match data.data_type() {
        Float32 => {
            let buffer: &Float32Array = data
                .as_any()
                .downcast_ref()
                .context("series is not float32")?;
            let series: Vec<_> = buffer.values().to_vec();
            for (i, value) in series.iter().enumerate() {
                rec.log(
                    format!("{}_{}", id.as_str(), i),
                    &rerun::Scalar::new(*value as f64),
                )
                .wrap_err("could not log series")?;
            }
        }
        Float64 => {
            let buffer: &Float64Array = data
                .as_any()
                .downcast_ref()
                .context("series is not float64")?;
            let series: Vec<_> = buffer.values().to_vec();
            for (i, value) in series.iter().enumerate() {
                rec.log(
                    format!("{}_{}", id.as_str(), i),
                    &rerun::Scalar::new(*value),
                )
                .wrap_err("could not log series")?;
            }
        }
        Int32 => {
            let buffer: &Int32Array = data
                .as_any()
                .downcast_ref()
                .context("series is not Int32")?;
            let series: Vec<_> = buffer.values().to_vec();
            for (i, value) in series.iter().enumerate() {
                rec.log(
                    format!("{}_{}", id.as_str(), i),
                    &rerun::Scalar::new(*value as f64),
                )
                .wrap_err("could not log series")?;
            }
        }
        Int64 => {
            let buffer: &Int64Array = data
                .as_any()
                .downcast_ref()
                .context("series is not Int64")?;
            let series: Vec<_> = buffer.values().to_vec();
            for (i, value) in series.iter().enumerate() {
                rec.log(
                    format!("{}_{}", id.as_str(), i),
                    &rerun::Scalar::new(*value as f64),
                )
                .wrap_err("could not log series")?;
            }
        }

        _ => unimplemented!("This has not yet implemented. Please contribute to dora-rerun :)"),
    }
    Ok(())
}
