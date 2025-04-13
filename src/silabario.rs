#![allow(dead_code, unused_mut, unused_assignments)]

use std::char;
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
pub const CONJUNTO_DE_VOCALES: [char; 20] = ['a', 'e', 'i', 'o', 'u', 'à', 'á', 'ä', 'è', 'é', 'ë', 'ì', 'í', 'ï', 'ò', 'ó', 'ö', 'ù', 'ú', 'ü'];

// grupos de vocales
const ABIERTO_NORMAL: [char; 10] = ['a', 'ä', 'â', 'å', 'e', 'ë', 'ê', 'o', 'ö', 'ô'];
const ABIERTO_ACENTADO: [char; 6] = ['á', 'à', 'é', 'è', 'ó', 'ò'];
const ABIERTO_COMPLETO: [char; 9] = ['a', 'e', 'o', 'á', 'à', 'é', 'è', 'ó', 'ò'];
const CERRADO_NORMAL: [char; 4] = ['i', 'î', 'u', 'û'];
const CERRADO_ACENTADO: [char; 4] = ['í', 'ì', 'ú', 'ù'];
const ULTIMA: [char; 4] = ['e', 'é', 'i', 'í'];

// grupos de consonantes
const ANTES_DEL_GRUPO_IZQUIERDO: [char; 8] = ['b', 'v', 'c', 'k', 'f', 'g', 'p', 't'];
const ANTES_DEL_GRUPO_DERECHO: [char; 9] = ['b', 'v', 'c', 'd', 'k', 'f', 'g', 'p', 't'];
const GRUPO_EXTRANJERO: [char; 5] = ['s', 'l', 'r', 'n', 'c'];
const CONSONANTE_PARES: [&str; 10] = ["pt", "ct", "cn", "ps", "mn", "gn", "ft", "pn", "cz", "ts"];

pub fn es_consonante(letra: char) -> bool {
    // Devuelve si un carácter es una consonante o no.
    !CONJUNTO_DE_VOCALES.contains(&letra)
}

fn ataque(palabra: &Vec<char>, longitud: usize, mut posición: usize) -> usize {
    //Encuentra el final del inicio/ataque de la sílaba.
    let mut ultima_consonante: char = 'a';

    while posición < longitud && es_consonante(palabra[posición]) && palabra[posición] != 'y' {
        ultima_consonante = palabra[posición];
        posición += 1;
    }

    if longitud <= posición {
        return posición;
    }

    let c1: char = palabra[posición];

    if posición < longitud - 1 {

        if c1 == 'u' {

            if ultima_consonante == 'q' {
                posición += 1;
            }

            else if ultima_consonante == 'g' {

                if ULTIMA.contains(&palabra[posición + 1]) {
                    posición += 1;
                }

            }

        }

        else if c1 == 'ü' && ultima_consonante == 'g' {
            posición += 1;
        }

    }

    posición

}

fn nucleo(palabra: &Vec<char>, longitud: usize, mut posición: usize) -> usize {

    // Encuentra el final del núcleo de la sílaba.
    let mut anterior: usize = 0;

    // ¿No tiene núcleo?
    if posición >= longitud {
        return posición;
    }

    // Salta una letra 'y' al inicio del núcleo, es como consonante
    if palabra[posición] == 'y' {
        posición += 1;
    }

    // Vocal abierta o cerrada con acento escrito
    if posición < longitud {

        let cr: char = palabra[posición];

        if ABIERTO_ACENTADO.contains(&cr) || ABIERTO_NORMAL.contains(&cr) {
            anterior = 0;
            posición += 1;
        }


        else if CERRADO_ACENTADO.contains(&cr) || cr == 'ü' {
            return posición + 1;
        }

        else if CERRADO_NORMAL.contains(&cr) {
            anterior = 2;
            posición += 1;
        }

    }

    // Si se ha insertado 'h' en el núcleo entonces no determina diptongo ni hiato
    let mut aitch: bool = false;

    if posición < longitud && palabra[posición] == 'h' {
        posición += 1;
        aitch = true;
    }

    // Segunda vocal
    if posición < longitud {
        
        let mut cr: char = palabra[posición];

        if ABIERTO_COMPLETO.contains(&cr) { // vocal abierta

            if anterior == 0 {

                if aitch {
                    posición -= 1;
                }

                return posición;

            } 

            else {
                posición += 1;
            }

        }

        else if CERRADO_ACENTADO.contains(&cr) { // Diptongo

            if anterior != 0 {
                posición += 1;
            }
            
            else if aitch {
                posición -= 1;
            }

            return posición;
            
        }

        else if CERRADO_NORMAL.contains(&cr) || cr == 'ü' {

            if posición < longitud - 1 {

                cr = palabra[posición + 1];

                if !es_consonante(cr) {

                    if palabra[posición - 1] == 'h' {
                        posición -= 1;
                    }

                    return posición;

                }
                
            }

            if palabra[posición] != palabra[posición - 1] {
                posición += 1;
            }

            // es un diptongo descendiente
            return posición;

        }

    }

    // ¿Tercera vocal?
    if posición < longitud && CERRADO_NORMAL.contains(&palabra[posición]) {
        return posición + 1;
    }

    posición

}

fn coda(palabra: &Vec<char>, longitud: usize, posición: usize) -> usize {
    
    // Encuentra el final de la coda de la sílaba y de la sílaba misma.
    if posición >= longitud || !es_consonante(palabra[posición]) {
        return posición;
    }
    
    else if posición == longitud - 1 {
        return posición + 1;
    }

    let (c1, c2): (char, char) = (palabra[posición], palabra[posición + 1]);

    if !es_consonante(c2) {
        return posición;
    }

    if posición < (longitud - 2) {

        let c3: char = palabra[posición + 2];
        
        if !es_consonante(c3) {

            // ll, ch, rr antes de vocal
            if c1 == 'l' && c2 == 'l' {
                return posición;
            }

            if c1 == 'c' && c2 == 'h' {
                return posición;
            }

            if c1 == 'r' && c2 == 'r' {
                return posición;
            }

            // Una consonante + 'h' comienza una sílaba, excepto los grupos sh y rh
            if c1 != 's' && c1 != 'r' && c2 == 'h' {
                return posición;
            }
            
            /*
            Si la letra 'y' está precedida por alguna
            letra 's', 'l', 'r', 'n' o 'c' luego
            una nueva sílaba comienza en la consonante anterior
            de lo contrario, comienza en la letra 'y'
            */
            if c2 == 'y' {

                if GRUPO_EXTRANJERO.contains(&c1) {
                    return posición;
                }

                return posición + 1;

            }

            // grupos: gl - kl - bl - vl - pl - fl - tl
            if ANTES_DEL_GRUPO_IZQUIERDO.contains(&c1) && c2 == 'l' {
                return posición;
            }

            // grupos: gr - kr - dr - tr - br - vr - pr - fr
            if ANTES_DEL_GRUPO_DERECHO.contains(&c1) && c2 == 'r' {
                return posición;
            }

            return posición + 1;

        }
        
        else {

            // ¿Tres consonantes hasta el final, palabras extranjeras?
            if (posición + 3) == longitud && c2 == 'y' {

                if GRUPO_EXTRANJERO.contains(&c1) {
                    return posición;
                }

                // 'y' al final como vocal con c2 o 3 consonantes al final
                if c3 == 'y' {
                    return posición + 1;
                }
                
                else {
                    return posición + 3;
                }

            }

            // y como vocal
            if c2 == 'y' {

                if GRUPO_EXTRANJERO.contains(&c1) {
                    return posición;
                }

                return posición + 1;

            }

            /*
            Los grupos pt, ct, cn, ps, mn, gn, ft, pn, cz, tz y ts comienzan una sílaba
            cuando esté precedido por otra consonante
            */
            if CONSONANTE_PARES.contains(&&palabra.iter().collect::<String>()[posición + 1 .. posición + 3]) {
                return posición + 1;
            }


            /*
            Los grupos consonánticos formados por una consonante
            después de la letra 'l' o 'r' no puede ser
            separados y siempre comienzan sílaba
            'ch' o 'y' como vocal
            */
            if (c3 == 'l' || c3 == 'r') || (c2 == 'c' && c3 == 'h') || (c3 == 'y') {
                return posición + 1;
            }
            
            else {
                return posición + 2;
            }

        }
    }
    
    else {

        if c2 == 'y' {
            return posición;
        }

        // La palabra termina con dos consonantes.
        return posición + 2;

    }

}

pub fn silabizar(palabra: &str) -> Vec<Vec<char>> {

    let mut palabra_chars: Vec<char> = palabra.chars().collect::<Vec<char>>();

    let (mut posiciones, mut posición): (Vec<usize>, usize) = (Vec::new(), 0);
    let longitud: usize = palabra_chars.len();

    // revisar todas las sílabas y las procesa
    while posición < longitud {

        posiciones.push(posición);

        posición = ataque(&palabra_chars, longitud, posición);
        posición = nucleo(&palabra_chars, longitud, posición);
        posición = coda(&palabra_chars, longitud, posición);

    }

    posiciones.push(longitud);

    //silabás
    posiciones.iter().zip(posiciones.iter().skip(1)).map(|(start, end)| palabra_chars[*start..*end].iter().map(|c: &char| *c).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
    
}
