#[cfg(test)]
mod tests {
    use super::*;

    include!("tests_helpers.rs");
    include!("tests_happy_and_precommit.rs");
    include!("tests_replay_and_generation.rs");
    include!("tests_recovery.rs");
    include!("tests_receipts.rs");
}
