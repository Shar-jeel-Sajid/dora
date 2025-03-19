use dora_node_api::{
    arrow::array::{Float32Array, Float64Array, Int32Array, Int64Array},
    arrow::datatypes::Datatype,
    dora_core::config::DataId,
    ArrowData,
};
use eyre::{Context, ContextCompat, Result};
use rerun::RecordingStream;

pub fn update_series(rec: &RecordingStream, id: DataId, data: ArrowData) -> Result<()> {
    match data.data_type() {
        DataType::Float32 | DataType::Int32 | DataType::Int64 | Datatype::Float64 => {
            let buffer = as_primitive_array::<data.data_type>(&data);
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
