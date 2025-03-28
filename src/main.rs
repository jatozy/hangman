fn main() {
    println!("bitte das geheime wort eingeben");
    let mut raetsel_wort: String = String::new();
    let ergebnis_benutzereingabe = std::io::stdin().read_line(&mut raetsel_wort);
    if ergebnis_benutzereingabe.is_err() {
        println!("Es ist ein Fehler aufgetreten");
        return;
    }
    raetsel_wort.pop();
    println!("das eingegebene wort ist >{raetsel_wort}<");
}
