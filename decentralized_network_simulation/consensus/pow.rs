pub struct ProofOfWork;

impl ProofOfWork {
    pub fn new() -> Self {
        ProofOfWork
    }

    pub fn mine(&self, data: &[u8]) -> u64 {
        let mut nonce = 0;
        loop {
            if self.is_valid_nonce(data, nonce) {
                return nonce;
            }
            nonce += 1;
        }
    }

    fn is_valid_nonce(&self, data: &[u8], nonce: u64) -> bool {
        // Simple hash function for demonstration (use a real hash function in production)
        let hash = data.iter().fold(0, |acc, &b| acc.wrapping_add(b as u64).wrapping_add(nonce));
        hash % 1000 == 0 // Simple condition for valid nonce
    }
}
