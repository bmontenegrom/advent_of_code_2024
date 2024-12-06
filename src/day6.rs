

#[derive(Debug)]
struct Posicion {
    x: usize,
    y: usize,
}

impl Posicion {
    fn mover(&self, direccion: Direccion) -> Posicion {
        match direccion {
            Direccion::Arriba => Posicion {
                x: self.x,
                y: self.y - 1,
            },
            Direccion::Abajo => Posicion {
                x: self.x,
                y: self.y + 1,
            },
            Direccion::Izquierda => Posicion {
                x: self.x - 1,
                y: self.y,
            },
            Direccion::Derecha => Posicion {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

#[derive(Debug)]
enum Elemento {
    Obstaculo,
    Libre(bool),
}

#[derive(Debug)]
struct Guardia{
    posicion: Posicion,
    direccion: Direccion,
}
impl Guardia {
    fn new(posicion: Posicion, direccion: Direccion) -> Guardia {
        Guardia { posicion, direccion }
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
                    '.' => Elemento::Libre(false),
                    '^' => {
                        guardia = Guardia::new(Posicion { x, y }, Direccion::Arriba);
                        Elemento::Libre(true)
                    }
                    'v' => {
                        guardia = Guardia::new(Posicion { x, y }, Direccion::Abajo);
                        Elemento::Libre(true)
                    }
                    '<' => {
                        guardia = Guardia::new(Posicion { x, y }, Direccion::Izquierda);
                        Elemento::Libre(true)
                    }
                    '>' => {
                        guardia = Guardia::new(Posicion { x, y }, Direccion::Derecha);
                        Elemento::Libre(true)
                    }
                    _ => panic!("caracter no esperado"),
                })
                .collect()
        })
        .collect();
    (mapa, guardia)
}

#[aoc(day6, part1)]
fn day6_part1(input: &str) -> u32 {
    let (mut mapa, mut guardia) = parse_day6(input);
    let mut cout = 1;
    let mut fin = false;
    while !fin {
        let next_pos = guardia.posicion.mover(guardia.direccion);
        match mapa.get(next_pos.y).and_then(|fila| fila.get(next_pos.x)) {
            Some(Elemento::Obstaculo) => {
                guardia.direccion = match guardia.direccion {
                    Direccion::Arriba => Direccion::Derecha,
                    Direccion::Derecha => Direccion::Abajo,
                    Direccion::Abajo => Direccion::Izquierda,
                    Direccion::Izquierda => Direccion::Arriba,
                };
            }
            Some(Elemento::Libre(visitado)) => {
                guardia.posicion = next_pos;
                if !visitado {
                    mapa[guardia.posicion.y][guardia.posicion.x] = Elemento::Libre(true);
                    cout += 1;
                }
            }
            None => {
                fin = true;
            }
        }
        
    }
    cout
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
}
