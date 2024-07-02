pub struct ProofOfStake {
    stakes: Vec<u64>,
}

impl ProofOfStake {
    pub fn new(stakes: Vec<u64>) -> Self {
        ProofOfStake { stakes }
    }

    pub fn select_validator(&self) -> usize {
        // Simple selection mechanism based on stake proportion
        let total_stake: u64 = self.stakes.iter().sum();
        let mut rng = rand::thread_rng();
        let selected_stake: u64 = rand::Rng::gen_range(&mut rng, 0..total_stake);

        let mut cumulative_stake = 0;
        for (i, &stake) in self.stakes.iter().enumerate() {
            cumulative_stake += stake;
            if cumulative_stake >= selected_stake {
                return i;
            }
        }
        0
    }
}
