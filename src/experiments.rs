use super::*;
use rand::Rng;
use rand::SeedableRng;

const TEST_STR: &str = "Pagrindinė informacija Šis puslapis skirtas Kodavimo teorijos dalyko pratyboms programų sistemų studentams. Pratybos vyksta antradieniais 10:15 ir 12:00 nuotoliniu būdu per MS Teams, komandoje \"Kodavimo teorija (pratybos) 2021\". Prisijungimo prie komandos kodas - 40f5ytn (MS Teams programoje spaudžiate \"Join or create a team\", tada \"Join a team with a code\" lauke įrašote kodą ir spaudžiate \"Join team\"). Pratybų tvarka Kiekvienam studentui reikės atlikti po vieną praktinę užduotį. Užduotys paskirstytos atsitiktinai. Studentai savarankiškai skaitys algoritmų aprašymus, žiūrės užduočių aptarimų vaizdo įrašus ir bandys atlikti užduotį. Pratybų laikas skirtas individualioms konsultacijoms (galėsite užduoti klausimus, jei kas neaišku) ir užduočių atsiskaitymams. Kad visi norintys spėtų atsiskaityti savo užduotį, pirmenybė pratybų metu skiriama atsiskaitantiems. Vienos užduoties atsiskaitymas trunka apie pusvalandį, todėl per vienas pratybas spės atsiskaityti trys studentai, t. y. kiekvieną savaitę galės atsiskaityti iki 6 studentų. Jei atsiskaitančių bus mažiau, galėsiu skirti laiko ir konsultacijoms. Siekdamas suvaldyti norinčių atsiskaityti srautus, prašau pasirinkti atsiskaitymo datą (t. y. užsiregistruoti atsiskaitymui) šioje apklausoje. Atsiskaityti užduotį galės tik iš anksto užsiregistravę. Rekomenduoju visiems jau semestro pradžioje užsiregistruoti atsiskaitymui, nes vietų kiekis ribotas. Pasirinktą laiką bet kada galėsite pakeisti į bet kurį kitą laisvą būsimą laiką (pavyzdžiui, jei, atlikęs užduotį anksčiau, norėsite ją atsiskaityti anksčiau). Praktika rodo, kad dauguma studentų užduotį nori atsiskaityti semestro pabaigoje, tačiau turėkite omenyje, kad (atsižvelgiant į šio dalyko klausytojų skaičių) visiems norintiems atsiskaityti reikės bent 7 savaičių, todėl dalis jūsų atsiskaityti turės lapkritį, nes gruodį atsiskaitymui vietų visiems neužteks. Aišku, jei norite, atsiskaityti galite ir anksčiau (dalis studentų atsiskaito jau rugsėjį ar spalį). Atliktą praktinę užduotį reikia atsiskaityti per pratybas ir po atsiskaitymo įkelti į programos pateikimo užduotį. Kuo anksčiau įkelsite, tuo anksčiau užskaitysiu, kad atsiskaitėte. Galutinė įkėlimo data - gruodžio 22 d. 23:59. Jei užduoties atsiskaitymo per pratybas metu rasiu trūkumų, dar galėsite juos pašalinti prieš įkeldami užduotį. Svarbu! Turėkite omenyje, kad praktinė užduotis yra gana didelės apimties. Jokiu būdu nepalikite jos paskutinei minutei, nes nespėsite jos atlikti. Klausimus taip pat užduoti galite bet kuriuo metu ir el. paštu gintaras.skersys@mif.vu.lt arba asmeniškai per MS Teams. Aš atsakysiu, kai tik rasiu tam laiko. Esant reikalui, galėsime ir susiskambinti per MS Teams. Nuorodos: Užduočių paskirstymas Užduotys Užduočių aptarimų vaizdo įrašai Praktinės užduoties atsiskaitymo tvarkaraštis Programos pateikimo užduotis Programavimo kalba Programavimo kalbą pasirinkti galite savo nuožiūra. Galite naudoti ir .NET platformos reikalaujančias kalbas. Nenaudokite programavimo kalbų, programos paleidimui reikalaujančių specialių priemonių, pavyzdžiui, Maple, Matlab, Python, PHP... Išimtis - Java, ją galite naudoti. Jei visgi norite programuoti kalba, reikalaujančia specialių priemonių, kreipkitės į mane, bandysime tartis (gal aš galėsiu tas priemones susiinstaliuoti, arba gal jūs galėsite sugeneruoti vykdomąjį failą, kurio paleidimui specialių priemonių nereikės). Pastaba: jei programa parašyta C# kalba, kartais naudotojui kyla problemų įvedant realiuosius skaičius (pavyzdžiui, kanalo klaidos tikimybę). Tegu programa naudotojui leidžia įvesti realiuosius skaičius bet kuriuo formatu - ir su tašku, ir su kableliu. Prieš siųsdami patikrinkite, ar įmanoma įvesti realųjį skaičių, jei Windows sistemoje nustatyta, kad realieji skaičiai įvedami su kableliu. Darbo atsiskaitymui reikia pateikti (parodyti per pratybas ir paskui įkelti į VMA): Programą (Microsoft Windows .exe ar Java .class arba .jar failus. Jei programuojama Java, pageidautina, kad būtų pateikti ir .bat failai kompiliavimui bei vykdymui). Jei programavote naudodami kokią nors programavimo aplinką, tai turėkite omenyje, kad greičiausiai aš jos neturiu. Todėl paruoškite programą taip, kad ją būtų galima paleisti be tos aplinkos. C/C++/C# kalbų atveju dažnai užtenka kartu atsiųsti reikiamas dinamines bibliotekas. Java kalbos atveju paruoškite .class ar .jar failus. Jei realizuota tik dalis užduoties, pateikti programą(-as), leidžiančias išbandyti realizuotas dalis. Pavyzdžiui, jei realizuotas tik kodavimas, pateikti programą, kuri leistų nurodyti kodo parametrus ir užkoduotų nurodytą vektorių. Dalys, kurioms išbandyti nepateiktos programos, nebus vertinamos. Programos tekstus (source code) su komentarais. Kiekvienai funkcijai ir procedūrai turi būti nurodyta: ką jinai atlieka, kokius ima parametrus ir kokį rezultatą grąžina. Kiekvienam stambesniam kodo gabalui irgi turi būti nurodyta, ką jis atlieka. Ataskaitą (pageidautina - PDF formatu), kurioje turi būti nurodyta: kurios užduoties dalys realizuotos ir kurios ne, kam ir kokios panaudotos trečiųjų šalių funkcijų bibliotekos, (neprivaloma, bet pageidautina) kiek laiko užtruko užduoties atlikimas (valandomis) iš viso ir kiekvienam etapui atskirai: literatūros skaitymui ir kodo veikimo aiškinimuisi, projektavimui, programavimui, klaidų ieškojimui ir taisymui, ataskaitos ruošimui, kaip paleisti programą (kur yra paleidžiamasis failas, kaip jis vadinasi, kokius parametrus reikia nurodyti ir pan., t. y. visa informacija, kurią reikia žinoti, norint paleisti programą), kur ir kokie yra programos tekstų failai ir kas juose pateikiama (pavyzdžiui: \"source/kodavimas/kanalas.java - realizuotas pranešimo siuntimas kanalu\" ir t.t.), vartotojo sąsajos aprašymas su naudojimo pavyzdžiais (akcentuoti, kokius duomenis galima įvesti ir ką jie reiškia, pavyzdžiui, jei reikia, nurodyti, kaip įvedami ir išvedami baigtinio kūno elementai), padaryti programiniai sprendimai (pavyzdžiui, kas daroma, jei, tekstą suskaidžius vektoriais, negaunamas pilnas vektorius; kokiu būdu pradinis tekstas paverčiamas vektoriais, kuriuos paskui koduojame; kokiu būdu vektoriai siunčiami kanalu; ir t.t.), atliktų eksperimentų aprašymas - kokie eksperimentai atlikti, su kokiais parametrais, kokie gauti rezultatai. Pavyzdžiui, eksperimentais galima bandyti nustatyti, kaip klaidų taisymo efektyvumas ir vykdymo laikas priklauso nuo kodo parametrų ir (ar) kanalo klaidos tikimybės. Bent vieno eksperimento rezultatai turi būti pateikti grafiku. naudotos literatūros sąrašas. Darbas priimamas tik tuo atveju, jei pateikiamos visos trys dalys: pradiniai tekstai, pati programa, ataskaita. Atsiskaičius darbą pratybų metu, viską sudėti į archyvą (.zip, .rar, .7z ar pan.) ir įkelti į programos pateikimo užduotį. Archyvo pavadinimas turėtų būti suformuotas taip: „užduoties_numeris - programavimo_kalba - vardas pavardė“, pavyzdžiui, galėtų būti „A5 - Java - Gintaras Skersys.zip“ (be kabučių). Praktinė užduotis laikoma galutinai atsiskaityta tik tada, kai įvertinu įkeltą darbą. Perspėjimai Plagijavimas. Studentų bendradarbiavimas atliekant praktinę užduotį yra pageidautinas. Labai gerai, jei jūs kartu aiškinatės, kaip koks dekodavimo algoritmas veikia ir pan. Bet pati programa turi būti realizuota savarankiškai. Kiekvienas iš jūsų pats turi parašyti savo programos matematinę (veiksmai su baigtinio kūno elementais, matricomis, polinomais) ir klaidas taisančių kodų dalį nuo pradžios iki galo. Jei pastebėsiu, kad pasinaudota kitų studentų darbais ar Internete pateiktomis realizacijomis, galutinis egzamino įvertinimas bus nepatenkinamas (kad ir kiek balų būtų gauta už kitas įvertinimo dalis: pranešimus, egzamino laikymą, aktyvumą; tai galioja ir tuo atveju, jei tai pastebėsiu jau po to, kai užduotis buvo atsiskaityta). Kitas algoritmas. Jei realizuosite kitą algoritmą, negu nurodyta užduoties sąlygoje, ir manęs apie tai neinformuosite, laikysiu tai bandymu mane apgauti ir už užduotį balų neskirsiu visai. Realizuota tik dalis užduoties. Jei realizuosite tik dalį, ir manęs apie tai neinformuosite, tikėdamasis, kad programos vertinimo metu aš to nepastebėsiu, laikysiu tai bandymu mane apgauti ir už užduotį balų neskirsiu visai. Jei užduotis nėra realizuota iki galo, ataskaitoje turi būti tiksliai dokumentuota, kas padaryta ir kas ne. Trečiųjų šalių funkcijų bibliotekos. Kiekvienas iš jūsų pats turi parašyti savo programos klaidas taisančių kodų dalį nuo pradžios iki galo, įskaitant ir (jei reikia) funkcijas atlikti matematiniams veiksmams su baigtinio kūno elementais, matricomis, polinomais. Jei matematinei daliai (veiksmams su matricomis, polinomais) realizuoti panaudosite trečiųjų šalių funkcijų bibliotekas, laikysiu, kad atlikote tik dalį užduoties. Šiuo metu tai reiškia, kad dėl to jūsų įvertinimas sumažės 10% maksimalaus galimo įvertinimo. Be abejo, apie tai, kad panaudojote trečiųjų šalių funkcijų bibliotekas, turite informuoti darbo ataskaitoje. Tarnybinio pobūdžio funkcijų bibliotekas (pavyzdžiui, įvykių registravimo (angl. logging), darbo su paveikslėliais ir pan.) bei programos projektavimo karkasus naudoti galite. Bet vėlgi tai turi būti nurodyta darbo ataskaitoje. Last modified: Monday, 6 September 2021, 9:06 PM";
const TEST_STR_BITS_LEN: f64 = (TEST_STR.len() * 8) as f64;

#[test]
fn experiment_error_probability_string() {
    let message = text_string_to_bools(TEST_STR);
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut result_file = File::create("experimento_rezultatai.csv").unwrap();
    let mut result_data = String::with_capacity(30_000);

    for i in 1..=1000 {
        let p = i as f64 / 1000f64;
        let (err_count_unencoded, err_count_encoded) =
            string(&message, TEST_STR_BITS_LEN, p, &mut rng);
        result_data
            .push_str(format!("{},{},{},\n", p, err_count_unencoded, err_count_encoded).as_str());
        println!("{}", i);
    }

    result_file.write(result_data.as_bytes()).unwrap();
}

fn string(message: &Vec<bool>, len: f64, p: f64, rng: &mut impl Rng) -> (f64, f64) {
    let mut message_to_send = message.clone();

    let mut encoded_message = encode(&message);
    send(&mut encoded_message, p, rng);
    send(&mut message_to_send, p, rng);
    let decoded_message = decode(&encoded_message);

    let err_count_unencoded = hamming_distance(&message, &message_to_send);
    let err_count_encoded = hamming_distance(&message, &decoded_message);

    (
        err_count_unencoded as f64 / len,
        err_count_encoded as f64 / len,
    )
}

fn hamming_distance(first: &[bool], second: &[bool]) -> usize {
    let mut dist = 0;

    for (x1, x2) in first.iter().zip(second) {
        if x1 != x2 {
            dist += 1;
        }
    }

    dist
}

fn send(message: &mut [bool], p: f64, rng: &mut impl Rng) {
    for bit in message {
        if rng.gen::<f64>() < p {
            *bit = !*bit;
        }
    }
}
