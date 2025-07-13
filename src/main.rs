use std::io::{self};

fn main() {
    println!("bitte das geheime wort eingeben");
    let raetsel_wort=eingeben().unwrap();
    let mut loesung: String = String::new();
    for _ in raetsel_wort.chars() {
        loesung.push('_');
    }
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("Spieler 2 ist jetzt dran.");
    let mut anzahl_fehler: u16 = 0;
    while loesung.contains('_') && anzahl_fehler < 5 {
        println!("Das Wort ist {loesung}");
        println!("Bisherige Fehler: {anzahl_fehler} von 5");
        println!("rate einen Buchstaben des Wortes.");
        let rate_buchstabe=eingeben().unwrap();
        if raetsel_wort.contains(&rate_buchstabe) {
            for (position, raetsel_buchstabe) in raetsel_wort.chars().enumerate() {
                if raetsel_buchstabe.to_string() == rate_buchstabe {
                    println!("Der Buchstabe ist im Wort vorhanden.");
                    loesung.replace_range(position..position + 1, &rate_buchstabe);
                }
            }
        } else {
            anzahl_fehler = anzahl_fehler + 1;
            println!("Der Buchstabe ist nicht im Wort vorhanden.");
        }
    }
    if anzahl_fehler == 5 {
        println!("Du hast verloren");
        println!("Das zu erratende Wort war {raetsel_wort}");
    } else {
        println!("Du hast gewonnen");
        println!("Die Lösung ist {loesung}");
    }
}

fn eingeben()->Result<String, io::Error> {
    let mut eingabe = String::new();
    std::io::stdin().read_line(&mut eingabe)?;
    eingabe.pop();
    eingabe.pop();
    return Ok(eingabe);
}
