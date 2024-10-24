use std::time::Duration;
use tokio::time::sleep;

pub async fn retry<F, T, E>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> futures::future::BoxFuture<'static, Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut retries = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }
                println!("Operation failed, retrying in {:?}. Error: {:?}", delay, e);
                sleep(delay).await;
                retries += 1;
                delay *= 2; // Exponential backoff
            }
        }
    }
}
