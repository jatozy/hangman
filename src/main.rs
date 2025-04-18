fn main() {
    println!("bitte das geheime wort eingeben");
    let mut raetsel_wort: String = String::new();
    eingeben(&mut raetsel_wort);
    println!("Spieler 2 ist jetzt dran.");
    let mut rate_buchstabe = String::new();
    println!("rate einen Buchstaben des Wortes.");
    eingeben(&mut rate_buchstabe);
    for raetsel_buchstabe in raetsel_wort.chars() {
        if raetsel_buchstabe.to_string() == rate_buchstabe {
            println!("Der Buchstabe ist im Wort vorhanden.")
        }
    }
}

fn eingeben(resultat: &mut String) {
    let ergebnis_benutzereingabe = std::io::stdin().read_line(resultat);
    if ergebnis_benutzereingabe.is_err() {
        println!("Es ist ein Fehler aufgetreten");
        return;
    }
    resultat.pop();
}
