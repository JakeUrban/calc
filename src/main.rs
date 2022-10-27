use std::collections::HashSet;

const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";
const EXP: &str = "**";

struct Operator {
    symbol: &'static str,
    f: fn(isize, isize) -> isize,
}

struct Operand {
    value: isize,
}

struct Node {
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
    operator: Option<Operator>,
    operand: Option<Operand>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = calc(&args[0..]);
    println!("result: {}", result);
}

fn calc(args: &[String]) -> isize {
    compute(&build_tree(&args))
}

fn build_tree(args: &[String]) -> Node {
    if args.len() == 0 {
        panic!("missing operand");
    }
    let leaf = Node {
        left: None,
        right: None,
        operator: None,
        operand: Some(Operand {
            value: parse_isize(&args[0]),
        }),
    };
    if args.len() == 1 {
        return leaf;
    }
    let mut cur = Node {
        left: Some(Box::new(leaf)),
        right: None,
        operator: Some(parse_operator(&args[1])),
        operand: None,
    };
    let mut right = build_tree(&args[2..]);
    if let Some(_) = right.operand {
        cur.right = Some(Box::new(right));
        return cur;
    }
    let cur_op_idx = get_idx_for_op(cur.operator.as_ref().unwrap());
    let right_op_idx = get_idx_for_op(right.operator.as_ref().unwrap());
    let root: Node;
    if cur_op_idx < right_op_idx {
        cur.right = Some(Box::new(right));
        root = cur;
    } else {
        cur.right = right.left;
        right.left = Some(Box::new(cur));
        root = right;
    }
    return root;
}

fn compute(n: &Node) -> isize {
    let result: isize;
    if let Some(_) = n.operand {
        result = n.operand.as_ref().unwrap().value;
    } else {
        result = (n.operator.as_ref().unwrap().f)(
            compute(n.left.as_ref().unwrap().as_ref()),
            compute(n.right.as_ref().unwrap().as_ref())
        );
    }
    result
}

fn get_idx_for_op(op: &Operator) -> usize {
    let pemdas: [HashSet<&str>; 3] = [
        HashSet::from([PLUS, MINUS]),
        HashSet::from([MULTIPLY, DIVIDE]),
        HashSet::from([EXP]),
    ];
    pemdas
        .iter()
        .position(|h| h.contains(op.symbol))
        .expect(&format!("symbol {} not found in PEMDAS", op.symbol))
}

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>()
        .expect(&format!("unexpected argument: {s}, expected a number"))
}

fn parse_operator(s: &str) -> Operator {
    match s {
        "+" => Operator {
            f: |a, b| a + b,
            symbol: PLUS,
        },
        "-" => Operator {
            f: |a, b| a - b,
            symbol: MINUS,
        },
        "*" => Operator {
            f: |a, b| a * b,
            symbol: MULTIPLY,
        },
        "/" => Operator {
            f: |a, b| a / b,
            symbol: DIVIDE,
        },
        "**" => Operator {
            f: exp,
            symbol: EXP,
        },
        _ => panic!("unrecognized operator {}", s),
    }
}

fn exp(a: isize, b: isize) -> isize {
    if b < 0 {
        panic!("unsupported exponent: {b}, must not be negative");
    }
    a.pow(b as u32)
}
