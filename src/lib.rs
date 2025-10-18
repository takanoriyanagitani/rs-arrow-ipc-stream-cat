use std::io;

use io::BufReader;
use io::Read;

use arrow_array::RecordBatch;

use arrow_ipc::reader::StreamReader;

use arrow::util::pretty::print_batches;

pub fn show_records<I>(rbats: I) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<RecordBatch, io::Error>>,
{
    print_batches(
        &rbats
            .map(|r| r.map_err(|e| io::Error::other(format!("show-record error: {e}"))))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map_err(|e| io::Error::other(format!("show-record error: {e}")))
}

pub fn stream2records_buf<R>(
    rdr: BufReader<R>,
    projection: Option<Vec<usize>>,
) -> Result<impl Iterator<Item = Result<RecordBatch, io::Error>>, io::Error>
where
    R: Read,
{
    let srdr = StreamReader::try_new_buffered(rdr, projection)
        .map_err(|e| io::Error::other(format!("buffered stream reader open error: {e}")))?;
    Ok(srdr
        .map(|r| r.map_err(|e| io::Error::other(format!("buffered stream reader map error: {e}")))))
}

pub fn stream2records<R>(
    rdr: R,
    projection: Option<Vec<usize>>,
) -> Result<impl Iterator<Item = Result<RecordBatch, io::Error>>, io::Error>
where
    R: Read,
{
    let srdr = StreamReader::try_new(rdr, projection)
        .map_err(|e| io::Error::other(format!("stream reader open error: {e}")))?;
    Ok(srdr.map(|r| r.map_err(|e| io::Error::other(format!("stream reader map error: {e}")))))
}

pub fn stdin2records(
    projection: Option<Vec<usize>>,
) -> Result<impl Iterator<Item = Result<RecordBatch, io::Error>>, io::Error> {
    stream2records(io::stdin().lock(), projection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_array::{Int32Array, RecordBatch};
    use arrow_ipc::writer::StreamWriter;
    use std::sync::Arc;

    #[test]
    fn test_show_records() {
        // Create a RecordBatch
        let schema = arrow_schema::Schema::new(vec![arrow_schema::Field::new(
            "a",
            arrow_schema::DataType::Int32,
            false,
        )]);
        let a = Int32Array::from(vec![1, 2, 3]);
        let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a)]).unwrap();

        // Create an in-memory IPC stream
        let mut stream = Vec::new();
        let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();

        // Read the batches from the stream
        let records = stream2records(stream.as_slice(), None).unwrap();

        // Show the records (this will just print to stdout, but we can check if it panics)
        show_records(records).unwrap();
    }
}
