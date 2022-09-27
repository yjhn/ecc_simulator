use rand::Rng;

/*
Siunčia pranešimą kanalu su nurodyta tikimybe
iškraipydama kiekvieną bitą.
Parametrai:
    message - siunčiamas pranešimas
    p - tikimybė, kad bitas bus iškraipytas
*/
pub fn send(message: &mut [bool], p: f64, rng: &mut impl Rng) {
    //let mut rng = SmallRng::from_entropy();
    for bit in message {
        if rng.gen::<f64>() < p {
            *bit = !*bit;
        }
    }
}

/*
Siunčia pranešimą kanalu su nurodyta tikimybe
iškraipydama kiekvieną bitą.
Parametrai:
    message - siunčiamas pranešimas
    p - tikimybė, kad bitas bus iškraipytas
Grąžina: iškraipytų bitų pozicijas pranešime
*/
pub fn send_with_error_info(message: &mut [bool], p: f64, rng: &mut impl Rng) -> Vec<usize> {
    //let mut rng = SmallRng::from_entropy();
    let mut errors = Vec::new();

    for (pos, bit) in message.iter_mut().enumerate() {
        if rng.gen::<f64>() < p {
            *bit = !*bit;
            errors.push(pos);
        }
    }

    errors
}
