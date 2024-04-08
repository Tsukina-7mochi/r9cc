#[derive(Debug, PartialEq)]
pub enum Node {
    Block {
        statements: Vec<Node>,
    },
    Integer {
        value: i32,
    },
    LocalVariable {
        identifier: String,
        offset: usize,
    },
    Return {
        value: Box<Node>,
    },
    If {
        condition: Box<Node>,
        statement: Box<Node>,
        end_label: String,
    },
    IfElse {
        condition: Box<Node>,
        statement: Box<Node>,
        end_label: String,
        else_statement: Box<Node>,
        else_label: String,
    },
    While {
        condition: Box<Node>,
        statement: Box<Node>,
        begin_label: String,
        end_label: String,
    },
    For {
        initializer: Option<Box<Node>>,
        condition: Option<Box<Node>>,
        updater: Option<Box<Node>>,
        statement: Box<Node>,
        begin_label: String,
        end_label: String,
    },
    OperatorAdd {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorSub {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorMul {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorDiv {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorLt {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorLtEq {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorEq {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorNe {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OperatorAssign {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Node {
    pub fn is_left_value(&self) -> bool {
        match self {
            Self::LocalVariable {
                identifier: _,
                offset: _,
            } => true,
            _ => false,
        }
    }
}
