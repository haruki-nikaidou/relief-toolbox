use smallvec::SmallVec;

pub struct PrimeFactors(pub SmallVec<[u32; 6]>);

/// 0-th element is always empty.
/// i-th element is the prime factors of i.
pub struct PrimeFactorsTable(pub Vec<PrimeFactors>);

impl PrimeFactorsTable {
    pub fn new(n: usize) -> Self {
        let mut table = (0..n + 1)
            .map(|_| PrimeFactors(SmallVec::new()))
            .collect::<Vec<_>>();
        let mut idx = 2usize;
        while idx <= n {
            if table[idx].0.is_empty() {
                // idx is prime; mark all multiples
                let mut multiple = idx;
                while multiple <= n {
                    let mut m = multiple;
                    while m % idx == 0 {
                        table[multiple].0.push(idx as u32);
                        m /= idx;
                    }
                    multiple += idx;
                }
            }
            idx += 1;
        }
        PrimeFactorsTable(table)
    }
    pub fn max_number(&self) -> usize {
        self.0.len() - 1
    }
}
