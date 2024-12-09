use std::collections::VecDeque;

#[derive(Debug)]
enum Operacion {
    Sum,
    Multiplication,
}

#[derive(Debug)]
struct Operador {
    termino: u64,
    operacion: Operacion,
}

impl Operador {
    fn new(termino: u64, operacion: Operacion) -> Operador {
        Operador { termino, operacion }
    }
}
#[derive(Debug)]
struct Ecuacion {
    value: u64,
    terms: Vec<u64>,
}

impl Ecuacion {
    fn new(value: u64, terms: Vec<u64>) -> Ecuacion {
        Ecuacion { value, terms }
    }
}

fn parse_day7(input: &str) -> Vec<Ecuacion> {
    input
        .lines()
        .map(|l| {
            let (value, terms) = l
                .split_once(": ")
                .expect("deberia poder separar la linea en : y espacio");
            let terms = terms
                .split(" ")
                .map(|c| c.parse::<u64>().expect("deberia ser un numero"))
                .collect();
            Ecuacion::new(value.parse::<u64>().expect("deberia ser un numero"), terms)
        })
        .collect()
}

fn test_eq(ecuacion: &Ecuacion) -> bool {
    let mut terms: VecDeque<u64> = ecuacion.terms.clone().into();
    let mut stack = vec![];
    let mut termino = terms.pop_front().expect("deberia haber al menos 2 valores");
    stack.push(Operador::new(termino, Operacion::Sum));
    stack.push(Operador::new(termino, Operacion::Multiplication));
    while !terms.is_empty() {
        termino = terms
            .pop_front()
            .expect("deberia haber al menos 1 elemento");
        let mut new_stack = Vec::new();
        for operador in stack {
            let mut acc = operador.termino;
            match operador.operacion {
                Operacion::Sum => acc += termino,
                Operacion::Multiplication => acc *= termino,
            }
            if acc < ecuacion.value {
                new_stack.push(Operador::new(acc, Operacion::Sum));
                new_stack.push(Operador::new(acc, Operacion::Multiplication));
            } else if acc == ecuacion.value && terms.is_empty() {
                return true;
            }
        }
        stack = new_stack;
        if stack.is_empty() {
            return false;
        }
    }
    false
}

#[aoc(day7, part1)]
fn day7_part1(input: &str) -> u64 {
    let ecuaciones = parse_day7(input);
    ecuaciones.iter().fold(0, |mut acc, e| {
        if test_eq(e) {
            acc += e.value;
        }
        acc
    })
}

mod test {
    use super::*;
    #[test]
    fn test_parser() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(day7_part1(input), 3749);
    }
}
