use std::io::Read;

fn main() {
    println!("bitte das geheime wort eingeben");
    let mut raetsel_wort: String = String::new();
    let mut ergebnis_benutzereingabe = std::io::stdin().read_line(&mut raetsel_wort);
    if ergebnis_benutzereingabe.is_err() {
        println!("Es ist ein Fehler aufgetreten");
        return;
    }
    raetsel_wort.pop();
    println!("das eingegebene wort ist >{raetsel_wort}<");
    println!("Spieler 2 ist jetzt dran.");
    let mut rate_buchstabe = String::new();
    println!("rate einen Buchstaben des Wortes.");
    ergebnis_benutzereingabe = std::io::stdin().read_line(&mut rate_buchstabe);
    if ergebnis_benutzereingabe.is_err() {
        println!("Es ist ein Fehler aufgetreten");
        return;
    }
    rate_buchstabe.pop();
    for raetsel_buchstabe in raetsel_wort.chars() {
        if raetsel_buchstabe.to_string() == rate_buchstabe {
            println!("Der Buchstabe ist im Wort vorhanden.")
        }
    }
}
