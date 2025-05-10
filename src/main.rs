fn main() {
    println!("bitte das geheime wort eingeben");
    let mut raetsel_wort: String = String::new();
    eingeben(&mut raetsel_wort);
    let mut loesung: String = String::new();
    for _ in raetsel_wort.chars() {
        loesung.push('_');
    }
    println!("Spieler 2 ist jetzt dran.");
    while loesung.contains('_') {
        println!("Das Wort ist {loesung}");
        let mut rate_buchstabe = String::new();
        println!("rate einen Buchstaben des Wortes.");
        eingeben(&mut rate_buchstabe);
        for (position, raetsel_buchstabe) in raetsel_wort.chars().enumerate() {
            if raetsel_buchstabe.to_string() == rate_buchstabe {
                println!("Der Buchstabe ist im Wort vorhanden.");
                loesung.replace_range(position..position + 1, &rate_buchstabe);
            }
        }
    }
    println!("Die Lösung ist {loesung}");
}

fn eingeben(resultat: &mut String) {
    let ergebnis_benutzereingabe = std::io::stdin().read_line(resultat);
    if ergebnis_benutzereingabe.is_err() {
        println!("Es ist ein Fehler aufgetreten");
        return;
    }
    resultat.pop();
}
