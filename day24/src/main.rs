#![cfg_attr(test, feature(test))]

use util::*;

type Out = String;

type S = &'static str;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Gate {
    And,
    Xor,
    Or,
}

struct In {
    inputs: BTreeMap<S, bool>,
    gates: BTreeMap<S, (S, S, Gate)>,
}

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once("\n\n").unwrap();

    let inputs = a
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(": ").unwrap();
            (k, v == "1")
        })
        .collect();

    let gates = b
        .lines()
        .map(|l| {
            let [a, g, b, _, k] = l.split_whitespace().collect::<Vec<_>>().try_into().unwrap();
            let g = match g {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                _ => Gate::Xor,
            };
            (k, (a, b, g))
        })
        .collect();

    In { inputs, gates }
}

fn resolve(wire: S, states: &mut BTreeMap<S, bool>, gates: &BTreeMap<S, (S, S, Gate)>) -> bool {
    if let Some(v) = states.get(&wire) {
        return *v;
    }

    let (a, b, gate) = gates[&wire];
    let a = resolve(a, states, gates);
    let b = resolve(b, states, gates);
    let c = match gate {
        Gate::And => a && b,
        Gate::Xor => a ^ b,
        Gate::Or => a || b,
    };
    states.insert(wire, c);
    c
}

fn get_number(
    letter: char,
    wires: &BTreeSet<S>,
    states: &mut BTreeMap<S, bool>,
    gates: &BTreeMap<S, (S, S, Gate)>,
) -> u64 {
    let mut n = 0_u64;
    for wire in wires.iter().filter(|w| w.starts_with(letter)).rev() {
        n <<= 1;
        if resolve(wire, states, gates) {
            n |= 1;
        }
    }
    n
}

fn part1(input: &In) -> Out {
    let mut states = input.inputs.clone();
    let gates = input.gates.clone();
    let wires: BTreeSet<S> = states.keys().chain(gates.keys()).copied().collect();
    get_number('z', &wires, &mut states, &gates).to_string();
    "".into()
}

fn swap<K: Ord, V>(map: &mut BTreeMap<K, V>, a: K, b: K) {
    let mut a_val = map.remove(&a).unwrap();
    let b_val = map.get_mut(&b).unwrap();
    std::mem::swap(&mut a_val, b_val);
    map.insert(a, a_val);
}

#[allow(dead_code)]
fn visualize(connections: &BTreeMap<(S, S, Gate), S>) {
    println!("digraph G {{");
    for ((a, b, g), c) in connections {
        let q = match g {
            Gate::And => "red",
            Gate::Xor => "green",
            Gate::Or => "blue",
        };
        println!("  {a} -> {c}[color={q}]");
        println!("  {b} -> {c}[color={q}]");
    }
    println!("}}");
}

#[allow(dead_code)]
fn diagnose(connections: &BTreeMap<(S, S, Gate), S>) {
    let get = |a: &str, b: &str, g: Gate| {
        Option::or(connections.get(&(a, b, g)), connections.get(&(b, a, g))).copied()
    };

    let half_adds: [_; 45] =
        std::array::from_fn(|i| get(&format!("x{i:02}"), &format!("y{i:02}"), Gate::Xor).unwrap());

    println!("{half_adds:?}");

    let half_carries: [_; 45] =
        std::array::from_fn(|i| get(&format!("x{i:02}"), &format!("y{i:02}"), Gate::And).unwrap());

    // special mention: this revealed z19 as out of place before I came up with the loop
    println!("{half_carries:?}");

    // swapped nodes end up causing this to panic because it assumes the adder is structured correctly
    // this serves as a sort of canary, but relies on manual labor, sometimes trial and error, to identify what the necessary swap is
    // this just gives a "vicinity", which was more than good enough for the input data given
    let mut carry = half_carries[0];
    for i in 1..=44 {
        println!("{i}");
        let foo = get(carry, half_adds[i], Gate::And).unwrap();
        println!("{foo:?}");
        let bar = get(foo, half_carries[i], Gate::Or).unwrap();
        println!("{bar:?}");
        carry = bar;
    }
}

fn part2(input: &In) -> Out {
    let gates = &input.gates;
    let mut connections: BTreeMap<(S, S, Gate), S> = gates.iter().map(|(k, v)| (*v, *k)).collect();
    assert_eq!(connections.len(), gates.len());

    // The Workshop
    let swaps = [
        ["z11", "wpd"],
        ["jqf", "skh"],
        ["z19", "mdd"],
        ["z37", "wts"],
    ];

    for [a, b] in swaps {
        swap(&mut connections, gates[a], gates[b]);
    }

    // diagnose(&connections);

    let mut wires = swaps.into_iter().flatten().collect::<Vec<_>>();
    wires.sort();
    wires.join(",")
}

util::register!(parse, part1, part2, @alt);
