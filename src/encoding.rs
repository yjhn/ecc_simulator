/*
Užkoduoja pranešimą, esantį struktūroje.
Grąžina: užkoduotą pranešimą
*/
pub fn encode(message: &[bool]) -> Vec<bool> {
    let mut encoder_state = [false; 6];
    let mut result = Vec::with_capacity(2 * message.len() + 20);

    for &bit in message.iter().chain([false; 6].iter()) {
        result.push(bit);
        // Ekvivalentu (bit + self.state[1] + self.state[4] + self.state[5]) mod 2.
        let ecc_bit = bit ^ encoder_state[1] ^ encoder_state[4] ^ encoder_state[5];
        result.push(ecc_bit);

        // Perstumiame atminties registrus.
        encoder_state.rotate_right(1);
        encoder_state[0] = bit;
    }

    result
}
