use std::collections::HashSet;

const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";
const EXP: &str = "**";

struct Operator {
    symbol: &'static str,
    f: fn(isize, isize) -> isize,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

struct Operand {
    value: isize,
}

enum Node {
    InnerNode(Operator),
    Leaf(Operand),
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = calc(&args[0..]);
    println!("result: {}", result);
}

fn calc(args: &[String]) -> isize {
    compute(build_tree(&args))
}

fn build_tree(args: &[String]) -> Node {
    if args.len() == 0 {
        panic!("missing operand");
    }
    let left = Node::Leaf(Operand {
        value: parse_isize(&args[0]),
    });
    if args.len() == 1 {
        return left;
    }
    let (operator_fn, operator_symbol) = get_operator_fn_and_symbol(&args[1]);
    let mut operator = Operator {
        f: operator_fn,
        symbol: operator_symbol,
        left: Some(Box::new(left)),
        right: None,
    };
    let right = build_tree(&args[2..]);
    match right {
        Node::Leaf(_) => {
            operator.right = Some(Box::new(right));
            Node::InnerNode(operator)
        }
        Node::InnerNode(mut right_operator) => {
            let cur_op_priority = get_operator_priority(&operator.symbol);
            let right_op_priority = get_operator_priority(&right_operator.symbol);
            if cur_op_priority < right_op_priority {
                operator.right = Some(Box::new(Node::InnerNode(right_operator)));
                Node::InnerNode(operator)
            } else {
                operator.right = right_operator.left;
                right_operator.left = Some(Box::new(Node::InnerNode(operator)));
                Node::InnerNode(right_operator)
            }
        }
    }
}

fn compute(n: Node) -> isize {
    match n {
        Node::Leaf(operand) => operand.value,
        Node::InnerNode(operator) => (operator.f)(
            compute(*operator.left.unwrap()),
            compute(*operator.right.unwrap()),
        ),
    }
}

fn get_operator_priority(symbol: &str) -> usize {
    let pemdas: [HashSet<&str>; 3] = [
        HashSet::from([PLUS, MINUS]),
        HashSet::from([MULTIPLY, DIVIDE]),
        HashSet::from([EXP]),
    ];
    pemdas
        .iter()
        .position(|h| h.contains(symbol))
        .expect(&format!("symbol {} not found in PEMDAS", symbol))
}

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>()
        .expect(&format!("unexpected argument: {s}, expected a number"))
}

fn get_operator_fn_and_symbol(s: &str) -> (fn(isize, isize) -> isize, &'static str) {
    match s {
        PLUS => (|a, b| a + b, PLUS),
        MINUS => (|a, b| a - b, MINUS),
        MULTIPLY => (|a, b| a * b, MULTIPLY),
        DIVIDE => (|a, b| a / b, DIVIDE),
        EXP => (exp, EXP),
        _ => panic!("unrecognized operator {}", s),
    }
}

fn exp(a: isize, b: isize) -> isize {
    if b < 0 {
        panic!("unsupported exponent: {b}, must not be negative");
    }
    a.pow(b as u32)
}
