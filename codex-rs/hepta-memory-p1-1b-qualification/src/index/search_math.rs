fn ordered_probe_signatures(query_signature: u64) -> Vec<u64> {
    let mut neighbors = Vec::with_capacity(64);
    for bit in 0..64 {
        neighbors.push(query_signature ^ (1_u64 << bit));
    }
    neighbors.sort_unstable();
    neighbors.dedup();

    let mut signatures = Vec::with_capacity(65);
    signatures.push(query_signature);
    signatures.extend(neighbors);
    signatures
}

fn lsh_signature(vector: &[i16], seed: &Digest32) -> Result<u64, ContractError> {
    let mut seed_bytes = [0_u8; 8];
    seed_bytes.copy_from_slice(&seed.as_bytes()[..8]);
    let seed_value = u64::from_be_bytes(seed_bytes);
    let mut signature = 0_u64;

    for bit in 0_u64..64 {
        let mut projection = 0_i64;
        for (dimension, value) in vector.iter().enumerate() {
            let sign = projection_sign(seed_value, bit, usize_to_u64(dimension, "LSH dimension")?);
            projection = projection
                .checked_add(i64::from(*value).saturating_mul(sign))
                .ok_or(ContractError::Overflow)?;
        }
        if projection >= 0 {
            signature |= 1_u64 << bit;
        }
    }
    Ok(signature)
}

fn projection_sign(seed: u64, bit: u64, dimension: u64) -> i64 {
    let mut state = seed
        ^ bit.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ dimension.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 12;
    state ^= state << 25;
    state ^= state >> 27;
    state = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
    if state & 1 == 0 { 1 } else { -1 }
}

fn cosine_similarity_ppm(left: &[i16], right: &[i16]) -> Result<u32, ContractError> {
    if left.len() != right.len() {
        return Err(ContractError::Corrupt(
            "cosine vectors have different dimensions".to_string(),
        ));
    }
    let dot = left
        .iter()
        .zip(right)
        .try_fold(0_i128, |sum, (left, right)| {
            sum.checked_add(i128::from(*left) * i128::from(*right))
                .ok_or(ContractError::Overflow)
        })?;
    if dot <= 0 {
        return Ok(0);
    }

    let left_norm_squared = norm_squared(left)?;
    let right_norm_squared = norm_squared(right)?;
    if left_norm_squared == 0 || right_norm_squared == 0 {
        return Err(ContractError::Corrupt(
            "cosine vectors must have non-zero norm".to_string(),
        ));
    }
    let norm_product = u128::from(left_norm_squared)
        .checked_mul(u128::from(right_norm_squared))
        .ok_or(ContractError::Overflow)?;
    let denominator = integer_sqrt_u128(norm_product);
    if denominator == 0 {
        return Err(ContractError::Overflow);
    }

    let numerator = u128::try_from(dot)
        .map_err(|_| ContractError::Overflow)?
        .checked_mul(SCORE_SCALE_PPM)
        .ok_or(ContractError::Overflow)?;
    let score = numerator / denominator;
    u32::try_from(score.min(SCORE_SCALE_PPM)).map_err(|_| ContractError::Overflow)
}

fn integer_sqrt_u128(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let mut previous = value;
    let mut current = (value >> 1) + 1;
    while current < previous {
        previous = current;
        current = (current + value / current) >> 1;
    }
    previous
}
