use std::time::Duration;

pub fn blocking_client(timeout: Duration) -> Result<reqwest::blocking::Client, reqwest::Error> {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
}

#[cfg(test)]
mod tests {
    use super::blocking_client;
    use std::time::Duration;

    #[test]
    fn builds_bounded_blocking_client() {
        blocking_client(Duration::from_secs(1)).expect("bounded HTTP client should build");
    }
}
