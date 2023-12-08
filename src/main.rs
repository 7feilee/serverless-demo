use lambda_runtime::{handler_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use serverless_demo::lambda_functions::process_data;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    lambda_runtime::run(handler_fn(process_data)).await?;
    Ok(())
}
