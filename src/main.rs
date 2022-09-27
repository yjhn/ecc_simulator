use crate::{converting::*, decoding::decode, encoding::encode};
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, BufReader, ErrorKind, Read, Write},
    path::Path,
    process::exit,
};

mod channel;
mod converting;
mod decoding;
mod encoding;
#[cfg(test)]
mod experiments;

fn main() {
    let stdin = io::stdin();
    let rng = SmallRng::from_entropy();
    println!(
        "Įveskite klaidos perdavimo kanale tikimybę, priklausančią [0, 1]\n\
        (dešimtainės trupmenos skirtukas ',' arba '.'):"
    );
    let mut buffer = String::with_capacity(100);

    let p = loop {
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim_end().replace(",", ".").parse::<f64>() {
            Ok(number) if (0. ..=1.).contains(&number) => break number,
            _ => {
                println!("Neteisinga įvestis. Įveskite iš naujo:");
                buffer.clear();
            }
        }
    };

    println!(
        "Pasirinkite, ką norite siųsti (įveskite pasirinkimo numerį):\n\
             1) Bitų vektorių\n\
             2) Tekstą\n\
             3) Paveiksliuką"
    );

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim_end().parse::<u8>() {
            Ok(number) => match number {
                1 => {
                    bit_vector(&stdin, p, rng);
                    break;
                }
                2 => {
                    string(&stdin, p, rng);
                    break;
                }
                3 => {
                    image(&stdin, p, rng);
                    break;
                }
                _ => println!("Neteisingas pasirinkimo numeris. Įveskite iš naujo."),
            },
            Err(_) => println!("Įveskite pasirinkimo numerį."),
        }
    }
}

/*
Paprašo naudotojo įvesti bitų vektorių ir jį siunčia kanalu.
Leidžia naudotojui pakeisti kanalu persiųtą vektorių.
Parametrai:
    stdin - struktūra, leidžianti nuskaityti naudotojo įvestus
            duomenis iš komandinės eilutės
    p - klaidos tikimybė kanale
*/
fn bit_vector(stdin: &io::Stdin, p: f64, mut rng: impl Rng) {
    let mut buffer = String::with_capacity(100);

    println!("Įveskite bitų vektorių (sudarytą iš nulių ir vienetų):");

    let message = loop {
        stdin.read_line(&mut buffer).unwrap();
        match bit_string_to_bools(buffer.trim_end()) {
            Ok(msg) => {
                break msg;
            }
            Err(_) => {
                println!(
                    "Įvesties vektorius privalo būti sudarytas iš nulių ir vienetų.\n\
                    Įveskite iš naujo:"
                );
                buffer.clear();
            }
        }
    };

    let mut encoded_message = encode(&message);
    println!(
        "Užkoduotas pranešimas:\n{}",
        bools_to_bit_string(&encoded_message)
    );
    let errors = channel::send_with_error_info(&mut encoded_message, p, &mut rng);
    let sent_encoded_message = bools_to_bit_string(&encoded_message);
    // Pažymime vietas, kuriose įvyko klaidos su '^'.
    let error_markers = {
        let mut e = String::with_capacity(sent_encoded_message.len());
        for pos in 0..sent_encoded_message.len() {
            e.push(if errors.contains(&pos) { '^' } else { ' ' });
        }
        e
    };
    println!(
        "Iš kanalo išėjęs pranešimas:\n{}\n\
        Klaidų skaičius: {}\n\
        Klaidų pozicijos (skaičiuoti pradedama nuo 0):\n{:?}\n\
        Iš kanalo išėjęs vektorius su pažymėtomis klaidų pozicijomis:\n{}\n{}",
        sent_encoded_message,
        errors.len(),
        errors,
        sent_encoded_message,
        error_markers
    );
    change_channel_output(&mut encoded_message, stdin);
    let decoded_message = decode(&encoded_message);
    println!(
        "Dekoduotas pranešimas:\n{}",
        bools_to_bit_string(&decoded_message)
    );
}

/*
Paprašo naudotojo įvesti tekstą ir jį siunčia kanalu
(ir užkoduotą, ir neužkoduotą).
Parametrai:
    stdin - struktūra, leidžianti nuskaityti naudotojo įvestus
            duomenis iš komandinės eilutės
    p - klaidos tikimybė kanale
*/
fn string(stdin: &io::Stdin, p: f64, mut rng: impl Rng) {
    let mut buffer = String::with_capacity(100);
    let mut last_len = 0;
    let endline_chars: &[char] = &['\r', '\n'];

    println!("Įveskite tekstą. Kai norite užbaigti, spauskite Enter du kartus:");
    loop {
        stdin.read_line(&mut buffer).unwrap();
        let current_len = buffer.trim_end_matches(endline_chars).len();
        // Jei paskutinė eilutė buvo tuščia, baigiame skaityti.
        if last_len == current_len {
            buffer.truncate(current_len);
            break;
        } else {
            last_len = current_len;
        }
    }
    let mut message = text_string_to_bools(&buffer);

    let mut encoded_message = encode(&message);
    channel::send(&mut encoded_message, p, &mut rng);
    channel::send(&mut message, p, &mut rng);
    let decoded_message = decode(&encoded_message);

    println!(
        "\nOriginalus tekstas:\n{}\n\n\
        Tekstas, persiųstas naudojant kodavimą:\n{}\n\n\
        Tekstas, persiųstas nenaudojant kodavimo:\n{}",
        buffer,
        bools_to_text_string(&decoded_message),
        bools_to_text_string(&message)
    );
}

/*
Paprašo naudotojo įvesti kelią iki paveiksliuko (galimi tik BMP paveiksliukai),
tada tą paveiksliuką siunčia kanalu (ir užkoduotą, ir neužkoduotą).
Parametrai:
    stdin - struktūra, leidžianti nuskaityti naudotojo įvestus
            duomenis iš komandinės eilutės
    p - klaidos tikimybė kanale
*/
fn image(stdin: &io::Stdin, p: f64, mut rng: impl Rng) {
    let mut file_path_buf = String::new();
    let mut file_path: &str;
    let image_contents = loop {
        println!("Įveskite kelią iki paveiksliuko (galimi tik BMP paveiksliukai):");
        stdin.read_line(&mut file_path_buf).unwrap();
        file_path = file_path_buf.trim_end();
        if !file_path.ends_with(".bmp") {
            println!("Tai ne BMP paveiksliukas.");
            file_path_buf.clear();
            continue;
        }

        match File::open(file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut file_contents = Vec::<u8>::new();
                match reader.read_to_end(&mut file_contents) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Klaida skaitant failą. Klaidos informacija:\n{:?}", e);
                        exit(1);
                    }
                }
                // BMP failai prasideda su "BM".
                if !file_contents.starts_with(b"BM") {
                    println!("Tai ne BMP paveiksliukas.");
                    file_path_buf.clear();
                    continue;
                }

                break file_contents;
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("Failas nerastas.");
                file_path_buf.clear();
                continue;
            }
            Err(e) => {
                println!("Klaida atidarant failą. Klaidos informacija:\n{:?}", e);
                exit(1);
            }
        }
    };

    println!("Atidaromas paveiksliukas: {}", file_path);
    match opener::open(OsStr::new(file_path)) {
        Ok(()) => (),
        Err(e) => println!(
            "Nepavyko atidaryti failo\n'{}'\nKlaidos informacija:\n{:?}",
            file_path, e
        ),
    }
    // BMP paveiksliuke pikselių pradžios vieta nurodoma
    // [10, 11, 12, 13] baituose (skaičiuojant nuo 0)
    // little-endian tvarka.

    // Sužinome, kur prasideda pikseliai. Visa informacija,
    // esanti faile prieš pikselius nėra iškraipoma.
    let header_size: usize = {
        let header_size_bytes = &image_contents[10..14];
        let n: u32 = (header_size_bytes[0] as u32)
            + ((header_size_bytes[1] as u32) << 8)
            + ((header_size_bytes[2] as u32) << 16)
            + ((header_size_bytes[3] as u32) << 24);
        n as usize
    };
    let bmp_header = &image_contents[0..header_size];
    let mut message = bytes_to_bools(&image_contents[header_size..]);

    // Siunčiame paveiksliuką su kodavimu.
    let mut encoded_message = encode(&message);
    channel::send(&mut encoded_message, p, &mut rng);
    let decoded_message = decode(&encoded_message);
    let decoded_image_pixels = bools_to_bytes(&decoded_message);
    let mut final_decoded_image = bmp_header.to_vec();
    final_decoded_image.extend_from_slice(&decoded_image_pixels);

    // Siunčiame paveiksliuką be kodavimo.
    channel::send(&mut message, p, &mut rng);
    let image_pixels = bools_to_bytes(&message);
    let mut final_image = bmp_header.to_vec();
    final_image.extend_from_slice(&image_pixels);

    // Išsaugome abu persiųstus paveiksliukus programos aplanke
    // (nesvarbu, kur buvo pradinis paveiksliukas).
    // Paveiksliukas, siųstas su kodavimu, vadinsis "<originalus pavadinimas> (su kodavimu).bmp"
    // Paveiksliukas, siųstas be kodavimo, vadinsis "<originalus pavadinimas> (be kodavimo).bmp"
    let file_name = Path::new(&file_path[0..file_path.len() - 4])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let name_with_enc = file_name.clone() + " (su kodavimu).bmp";
    let name_without_enc = file_name + " (be kodavimo).bmp";

    let mut file_with_enc = match File::create(&name_with_enc) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "Nepavyko sukurti failo\n{}\nKlaidos informacija:\n{:?}",
                name_with_enc, e
            );
            exit(1);
        }
    };
    let mut file_without_enc = match File::create(&name_without_enc) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "Nepavyko sukurti failo\n{}\nKlaidos informacija:\n{:?}",
                name_without_enc, e
            );
            exit(1);
        }
    };

    file_with_enc.write_all(&final_decoded_image).unwrap();
    file_without_enc.write_all(&final_image).unwrap();

    // Atidarome abu persiųstus paveiksliukus.
    println!(
        "Atidaromi persiųsti paveiksliukai:\n\
        Siųstas su kodavimu: '{}'\n\
        Siųstas be kodavimo: '{}'",
        name_with_enc, name_without_enc
    );
    match opener::open(OsStr::new(&name_with_enc)) {
        Ok(()) => (),
        Err(e) => println!(
            "Nepavyko atidaryti failo\n'{}'\nKlaidos informacija:\n{:?}",
            name_with_enc, e
        ),
    }
    match opener::open(OsStr::new(&name_without_enc)) {
        Ok(()) => (),
        Err(e) => println!(
            "Nepavyko atidaryti failo\n'{}'\nKlaidos informacija:\n{:?}",
            name_without_enc, e
        ),
    }
}

/*
Leidžia naudotojui pakeisti kanalu persiųstą pranešimą.
Pranešimas lieka tokio paties ilgio.
Parametrai:
    message - kanalu persiųstas pranešimas
    stdin - struktūra, leidžianti nuskaityti naudotojo įvestus
            duomenis iš komandinės eilutės
*/
fn change_channel_output(message: &mut Vec<bool>, stdin: &io::Stdin) {
    let mut buffer = String::new();
    println!(
        "Ar norite pakeisti iš kanalo išėjusį pranešimą?\n\
        1) taip\n\
        2) ne"
    );
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim_end().parse::<u8>() {
            Ok(number) => {
                match number {
                    1 => {
                        println!("Įveskite iš kanalo išėjusį pranešimą (bitų vektorių). Įvestas vektorius\n\
                                 turi būti tokio pat ilgio kaip ir iš kanalo išėjęs vektorius.");

                        loop {
                            buffer.clear();
                            stdin.read_line(&mut buffer).unwrap();
                            let user_sent_message = match bit_string_to_bools(buffer.trim_end()) {
                                Ok(msg) => msg,
                                Err(_) => {
                                    println!(
                                        "Įvesties vektorius privalo būti sudarytas iš nulių ir vienetų.\n\
                                        Įveskite iš naujo:"
                                    );
                                    continue;
                                }
                            };

                            if user_sent_message.len() == message.len() {
                                *message = user_sent_message;
                                break;
                            } else {
                                println!(
                                    "Įvestas vektorius turi būti tokio pat ilgio kaip ir iš\n\
                                    kanalo išėjęs vektorius. Įveskite iš naujo:"
                                )
                            }
                        }

                        break;
                    }
                    2 => {
                        break;
                    }
                    _ => println!("Neteisingas pasirinkimo numeris."),
                }
            }
            Err(_) => println!("Įveskite pasirinkimo numerį:"),
        }
    }
}
