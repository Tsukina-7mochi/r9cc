#[derive(Debug, PartialEq)]
pub enum Node {
    Integer { value: i32 },
    OperatorAdd { lhs: Box<Node>, rhs: Box<Node> },
    OperatorSub { lhs: Box<Node>, rhs: Box<Node> },
    OperatorMul { lhs: Box<Node>, rhs: Box<Node> },
    OperatorDiv { lhs: Box<Node>, rhs: Box<Node> },
    OperatorLt { lhs: Box<Node>, rhs: Box<Node> },
    OperatorLtEq { lhs: Box<Node>, rhs: Box<Node> },
    OperatorEq { lhs: Box<Node>, rhs: Box<Node> },
    OperatorNe { lhs: Box<Node>, rhs: Box<Node> },
}
