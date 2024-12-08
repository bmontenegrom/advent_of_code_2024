#[derive(Debug, Clone)]
struct Posicion {
    x: usize,
    y: usize,
}

impl Posicion {
    fn mover(&self, direccion: Direccion) -> Option<Posicion> {
        match direccion {
            Direccion::Arriba => {
                if self.y == 0 {
                    return None;
                }
                Some(Posicion {
                    x: self.x,
                    y: self.y - 1,
                })
            }
            Direccion::Abajo => Some(Posicion {
                x: self.x,
                y: self.y + 1,
            }),
            Direccion::Izquierda => {
                if self.x == 0 {
                    return None;
                }
                Some(Posicion {
                    x: self.x - 1,
                    y: self.y,
                })
            }
            Direccion::Derecha => Some(Posicion {
                x: self.x + 1,
                y: self.y,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

#[derive(Debug, Clone)]
enum Elemento {
    Obstaculo,
    Libre(bool, Option<Direccion>),
}

#[derive(Debug, Clone)]
struct Guardia {
    posicion: Posicion,
    direccion: Direccion,
}
impl Guardia {
    fn new(posicion: Posicion, direccion: Direccion) -> Guardia {
        Guardia {
            posicion,
            direccion,
        }
    }
}

fn parse_day6(input: &str) -> (Vec<Vec<Elemento>>, Guardia) {
    let mut guardia = Guardia::new(Posicion { x: 0, y: 0 }, Direccion::Arriba);
    let mapa: Vec<Vec<Elemento>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Elemento::Obstaculo,
                    '.' => Elemento::Libre(false, None),
                    '^' => {
                        guardia = Guardia::new(Posicion { x, y }, Direccion::Arriba);
                        Elemento::Libre(true, Some(Direccion::Arriba))
                    }
                    _ => panic!("caracter no esperado"),
                })
                .collect()
        })
        .collect();
    (mapa, guardia)
}

fn visitados(mapa: Vec<Vec<Elemento>>, guardia: Guardia) -> Vec<Posicion> {
    let mut fin = false;
    let mut mapa = mapa.clone();
    let mut guardia = guardia.clone();
    let mut res = vec![guardia.posicion.clone()];
    while !fin {
        if let Some(next_pos) = guardia.posicion.mover(guardia.direccion) {
            match mapa.get(next_pos.y).and_then(|fila| fila.get(next_pos.x)) {
                Some(Elemento::Obstaculo) => {
                    guardia.direccion = match guardia.direccion {
                        Direccion::Arriba => Direccion::Derecha,
                        Direccion::Derecha => Direccion::Abajo,
                        Direccion::Abajo => Direccion::Izquierda,
                        Direccion::Izquierda => Direccion::Arriba,
                    };
                }
                Some(Elemento::Libre(visitado, _)) => {
                    guardia.posicion = next_pos;
                    if !visitado {
                        mapa[guardia.posicion.y][guardia.posicion.x] =
                            Elemento::Libre(true, Some(guardia.direccion));
                        res.push(guardia.posicion.clone());
                    }
                }
                None => {
                    fin = true;
                }
            }
        } else {
            fin = true;
        }
    }
    res
}

fn check_ciclo(mapa: Vec<Vec<Elemento>>, guardia: Guardia) -> bool {
    let mut fin = false;
    let mut mapa = mapa.clone();
    let mut guardia = guardia.clone();
    while !fin {
        if let Some(next_pos) = guardia.posicion.mover(guardia.direccion) {
            match mapa.get(next_pos.y).and_then(|fila| fila.get(next_pos.x)) {
                Some(Elemento::Obstaculo) => {
                    guardia.direccion = match guardia.direccion {
                        Direccion::Arriba => Direccion::Derecha,
                        Direccion::Derecha => Direccion::Abajo,
                        Direccion::Abajo => Direccion::Izquierda,
                        Direccion::Izquierda => Direccion::Arriba,
                    };
                }
                Some(Elemento::Libre(visitado, dir)) => {
                    guardia.posicion = next_pos;
                    if !visitado {
                        mapa[guardia.posicion.y][guardia.posicion.x] =
                            Elemento::Libre(true, Some(guardia.direccion));
                    } else {
                        match dir {
                            Some(direccion) => {
                                if *direccion == guardia.direccion {
                                    return true;
                                }
                            }
                            _ => {
                                panic!("no deberia pasar que este visitado sin dirección")
                            }
                        }
                    }
                }
                None => {
                    fin = true;
                }
            }
        } else {
            fin = true;
        }
    }
    false
}

#[aoc(day6, part1)]
fn day6_part1(input: &str) -> u32 {
    let (mapa, guardia) = parse_day6(input);
    visitados(mapa, guardia).len() as u32
}

#[aoc(day6, part2)]
fn day6_part2(input: &str) -> u32 {
    let (mapa, guardia) = parse_day6(input);
    let mut candidatos = visitados(mapa.clone(), guardia.clone());
    candidatos.remove(0); //elimino posicion inicial
    candidatos
        .iter()
        .filter(|c| {
            let mut new_map = mapa.clone();
            new_map[c.y][c.x] = Elemento::Obstaculo;
            check_ciclo(new_map, guardia.clone())
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day6_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(day6_part1(input), 41);
    }

    #[test]
    fn test_day6_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(day6_part2(input), 6);
    }
}
