// pub enum Expr {
//     Binary: struct {
//         pub left: Box<Expr>,
//         pub operator: Token,
//         pub right: Box<Expr>,
//     }
// }

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
