//https://jimskapt.github.io/rust-book-fr/ch20-02-multithreaded.html
use salutations::GroupeTaches; //appel de lib.rs
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap(); // ip à laquelle le site est citué 
    let groupe = GroupeTaches::new(4); // création d'un groupe de 4 taches

    for flux in ecouteur.incoming() { 
        let flux = flux.unwrap();  // crée les flux avec une boucle for celon les flux entrants

        groupe.executer(|| {
            gestion_connexion(flux); // appelle le fct gestion_connexion et lui donne en paramètres les fluxs entrants
        });
    }
}
fn gestion_connexion(mut flux: TcpStream) {
//crée une variable tampon est va celon le débit de la requetes redirigé vers une page html
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (ligne_statut, nom_fichier) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contenu = fs::read_to_string(nom_fichier).unwrap(); //converti octet en string

    let reponse = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        ligne_statut,
        contenu.len(),
        contenu
    );

    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}

