/*
Paverčia tekstinį bitų vektorių bool tipo reikšmių vektoriumi
(pvz.: "1011" -> [true, false, true, true]).
Parametrai:
    buffer - tekstinis bitų vektorius
Grąžina:
    bool tipo reikšmių vektorių arba klaidą; klaida grąžinama,
    jei tekstinis bitų vektorius sudarytas ne vien iš 0 ir 1
*/
pub fn bit_string_to_bools(buffer: &str) -> Result<Vec<bool>, ()> {
    let mut result = Vec::with_capacity(buffer.len() * 8);

    for &char in buffer.as_bytes() {
        match char {
            b'1' => result.push(true),
            b'0' => result.push(false),
            _ => return Err(()),
        }
    }

    Ok(result)
}

/*
Paverčia baitų masyvą į bool tipo reikšmių vektorių
(pvz.: [23, 95] -> [false, false, false, true, false, true,
true, true, false, true, false, true, true, true, true, true]).
Parametrai:
    buffer - baitų masyvas
Grąžina: bool tipo kintamųjų vektorių
*/
pub fn bytes_to_bools(buffer: &[u8]) -> Vec<bool> {
    let mut result = Vec::with_capacity(buffer.len() * 8);

    for &byte in buffer {
        let bit8 = (byte & 1) > 0;
        let bit7 = (byte & 2) > 0;
        let bit6 = (byte & 4) > 0;
        let bit5 = (byte & 8) > 0;
        let bit4 = (byte & 16) > 0;
        let bit3 = (byte & 32) > 0;
        let bit2 = (byte & 64) > 0;
        let bit1 = (byte & 128) > 0;

        result.push(bit1);
        result.push(bit2);
        result.push(bit3);
        result.push(bit4);
        result.push(bit5);
        result.push(bit6);
        result.push(bit7);
        result.push(bit8);
    }

    result
}

/*
Paverčia bool tipo reišmių masyvą į tekstinį bitų vektorių
(pvz.: [false, true, true, false, false, false, false, true] -> "a").
Parametrai:
    buffer - bool tipo reiškmių masyvas
Grąžina: tekstinį bitų vektorių
*/
pub fn bools_to_bit_string(buffer: &[bool]) -> String {
    let mut result = String::with_capacity(buffer.len() / 8);
    for &bit in buffer {
        result.push(if bit { '1' } else { '0' });
    }

    result
}

/*
Paverčia bool tipo reikšmių masyvą į baitų vektorių
(pvz.: [true, true, true, true, true, true, true, true] -> [255]).
Parametrai:
    buffer - bool tipo reikšmių masyvas
Grąžina: baitų vektorių
*/
pub fn bools_to_bytes(buffer: &[bool]) -> Vec<u8> {
    if buffer.len() % 8 != 0 {
        panic!("Bitai nepasidalina į baitus");
    }

    // Sudėjus bitus į baitus, reikalingos vietos kiekis sumažėja 8 kartus.
    let mut result = Vec::with_capacity(buffer.len() / 8);

    for eight_bits in buffer.chunks_exact(8) {
        if let [b1, b2, b3, b4, b5, b6, b7, b8] = *eight_bits {
            // Sudedame baitą iš bitų.
            let byte = (b1 as u8) * 128
                + (b2 as u8) * 64
                + (b3 as u8) * 32
                + (b4 as u8) * 16
                + (b5 as u8) * 8
                + (b6 as u8) * 4
                + (b7 as u8) * 2
                + b8 as u8;

            result.push(byte);
        }
    }

    result
}

/*
Paverčia tekstą į bool tipo reikšmių vektorių.
Parametrai:
    buffer - tekstas
Grąžina: bool tipo reikšmių vektorių
*/
pub fn text_string_to_bools(buffer: &str) -> Vec<bool> {
    bytes_to_bools(buffer.as_bytes())
}

/*
Paverčia bool tipo reikšmių masyvą į tekstą.
Parametrai:
    message: bool tipo reikšmių masyvas
Grąžina:
    tekstą, pakeičia su UTF-8 neužkoduojamus simbolius
    į '�' (U+FFFD REPLACEMENT CHARACTER)
*/
pub fn bools_to_text_string(message: &[bool]) -> String {
    String::from_utf8_lossy(&bools_to_bytes(message)).into_owned()
}
