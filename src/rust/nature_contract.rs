//! CEFA-Pro 59/41 Nature Contract
//! Status: supplied implementation; execution validation required.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DistributionResult {
    /// 59% Humanitarian Mercy Pool total allocation.
    pub humanitarian_mercy_total: u64,
    pub children_hospitals: u64,
    pub open_science: u64,
    pub planetary_health: u64,

    /// 41% Infrastructure and operations allocation.
    pub infrastructure_reinvestment: u64,

    /// Integer precision-loss reconciliation.
    pub dust_balance: u64,
}

pub fn calculate_distribution(total_yield: u64) -> DistributionResult {
    let total = total_yield as u128;

    let humanitarian_mercy_total = ((total * 59) / 100) as u64;
    let infrastructure_reinvestment = ((total * 41) / 100) as u64;

    let children_hospitals = ((total * 30) / 100) as u64;
    let open_science = ((total * 20) / 100) as u64;
    let planetary_health = ((total * 9) / 100) as u64;

    let mapped_sum = children_hospitals
        + open_science
        + planetary_health
        + infrastructure_reinvestment;

    let dust_balance = total_yield.saturating_sub(mapped_sum);

    DistributionResult {
        humanitarian_mercy_total,
        children_hospitals,
        open_science,
        planetary_health,
        infrastructure_reinvestment,
        dust_balance,
    }
}

pub fn assert_59_41_contract(result: DistributionResult, total_yield: u64) -> bool {
    let sum = result.children_hospitals
        + result.open_science
        + result.planetary_health
        + result.infrastructure_reinvestment
        + result.dust_balance;

    assert_eq!(
        sum, total_yield,
        "CONTRACT VIOLATION: Distribution sum mismatch!"
    );

    let mercy_total = result.children_hospitals
        + result.open_science
        + result.planetary_health;
    let expected_mercy = (total_yield as u128 * 59 / 100) as u64;

    assert!(
        mercy_total <= expected_mercy,
        "CONTRACT VIOLATION: Mercy Pool exceeds 59% constraint!"
    );

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconciliation_is_exact_for_integer_inputs() {
        for total in [0, 1, 10_000, 59_000, 100_000, u64::MAX] {
            let result = calculate_distribution(total);
            assert!(assert_59_41_contract(result, total));
        }
    }
}
