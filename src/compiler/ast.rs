#[derive(Debug, PartialEq)]
pub enum Node {
    Integer { value: i32 },
    OperatorAdd { lhs: Box<Node>, rhs: Box<Node> },
    OperatorSub { lhs: Box<Node>, rhs: Box<Node> },
    OperatorMul { lhs: Box<Node>, rhs: Box<Node> },
    OperatorDiv { lhs: Box<Node>, rhs: Box<Node> },
}
