pub mod file;
pub mod kafka;
pub mod postgres;
pub mod pipe;

pub use file::FileSink;
pub use kafka::KafkaSink;
pub use postgres::PostgresSink;
pub use pipe::PipeSink;
