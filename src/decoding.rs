/*
Dekoduoja pranešimą, esantį struktūroje.
Grąžina: dekoduotą pranešimą
*/
pub fn decode(message: &[bool]) -> Vec<bool> {
    let mut decoder_state_regular = [false; 6];
    let mut decoder_state_feedback = [false; 6];
    let mut result = Vec::with_capacity(message.len() / 2);
    let mut ignored_bits_counter: u8 = 0;

    for two_bits in message.chunks_exact(2) {
        if let [bit1, bit2] = *two_bits {
            let ecc_bit = bit2
                ^ bit1
                ^ decoder_state_regular[1]
                ^ decoder_state_regular[4]
                ^ decoder_state_regular[5];

            // Suskaičiuojame Majority Decision Element (MDE) bitą.
            let mde_bit = {
                let true_bit_count = ecc_bit as u8
                    + decoder_state_feedback[0] as u8
                    + decoder_state_feedback[3] as u8
                    + decoder_state_feedback[5] as u8;

                match true_bit_count {
                    0 | 1 | 2 => false,
                    3 | 4 => true,
                    _ => unreachable!(),
                }
            };

            if ignored_bits_counter == 6 {
                result.push(decoder_state_regular[5] ^ mde_bit);
            } else {
                ignored_bits_counter += 1;
            }

            // Perstumiame viršutinius (per kuriuos eina pirmas iš
            // kanalo atėjęs bitas) ir apatinius (kurie naudojami
            // skaičiuojant MDE bitą) registrus.
            decoder_state_feedback[5] = decoder_state_feedback[4];
            decoder_state_feedback[4] = decoder_state_feedback[3] ^ mde_bit;
            decoder_state_feedback[3] = decoder_state_feedback[2];
            decoder_state_feedback[2] = decoder_state_feedback[1];
            decoder_state_feedback[1] = decoder_state_feedback[0] ^ mde_bit;
            decoder_state_feedback[0] = ecc_bit ^ mde_bit;

            decoder_state_regular.rotate_right(1);
            decoder_state_regular[0] = bit1;
        }
    }

    result
}
