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

struct InnerNode {
    right: Box<Node>,
    left: Box<Node>,
    operator: Operator,
}

enum Node {
    InnerNode(InnerNode),
    Leaf(Operand)
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
    let left = Node::Leaf(Operand { value: parse_isize(&args[0]) });
    if args.len() == 1 {
        return left;
    }
    let operator = parse_operator(&args[1]);
    let right = build_tree(&args[2..]);
    match right {
        Node::Leaf(_) => Node::InnerNode(InnerNode {
            left: Box::new(left),
            right: Box::new(right),
            operator
        }),
        Node::InnerNode(mut inner_right) => {
            let cur_op_idx = get_idx_for_op(&operator);
            let right_op_idx = get_idx_for_op(&inner_right.operator);
            if cur_op_idx < right_op_idx {
                Node::InnerNode(InnerNode {
                    left: Box::new(left),
                    right: Box::new(Node::InnerNode(inner_right)),
                    operator
                })
            } else {
                let new_left = InnerNode {
                    left: Box::new(left),
                    right: inner_right.left,
                    operator
                };
                inner_right.left = Box::new(Node::InnerNode(new_left));
                Node::InnerNode(inner_right)
            }
        }
    }
}

fn compute(n: &Node) -> isize {
    match n {
        Node::Leaf(operand) => operand.value,
        Node::InnerNode(inner_node) => {
            (inner_node.operator.f)(
                compute(&inner_node.left),
                compute(&inner_node.right)
            )
        }
    }
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
