use super::prelude::*;

pub fn literal<'source>(token: &TokenData<'source>) -> Literal {
    use std::str::FromStr;

    let (kind, value) = match token.kind() {
        Token::IntLiteral => (LiteralKind::Int, super::parse_int(&token)),
        Token::FloatLiteral => (LiteralKind::Float, super::parse_float(&token)),
        Token::StrLiteral => (
            LiteralKind::String,
            LiteralValue::String(
                token.source()[1..token.source().len() - 1].to_owned(),
            ),
        ),
        Token::BoolLiteral => (
            LiteralKind::Bool,
            LiteralValue::Bool(
                bool::from_str(token.source()).expect("Failed to parse bool"),
            ),
        ),
        Token::VectorLiteral => {
            (LiteralKind::Vector, super::parse_vector(&token))
        }

        _ => unreachable!(),
    };

    Literal { kind, value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let int = TokenData {
            kind: Token::IntLiteral,
            source: "123456",
            range: (0, 5),
        };

        assert_eq!(
            literal(&int),
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(123456)),
            }
        );
    }

    #[test]
    fn float() {
        let float = TokenData {
            kind: Token::FloatLiteral,
            source: "1.23456",
            range: (0, 5),
        };

        assert_eq!(
            literal(&float),
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(1.23456)),
            }
        );
    }

    #[test]
    fn string() {
        let string = TokenData {
            kind: Token::StrLiteral,
            source: "Test String",
            range: (0, 10),
        };

        assert_eq!(
            literal(&string),
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("Test String")),
            }
        );
    }

    #[test]
    fn bool() {
        let true_bool = TokenData {
            kind: Token::BoolLiteral,
            source: "true",
            range: (0, 3),
        };

        assert_eq!(
            literal(&true_bool),
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(true),
            }
        );

        let false_bool = TokenData {
            kind: Token::BoolLiteral,
            source: "false",
            range: (0, 4),
        };

        assert_eq!(
            literal(&false_bool),
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(false),
            }
        );
    }

    #[test]
    fn vector() {
        int_vector();
        float_vector();
        string_vector();
        bool_vector();
    }

    fn int_vector() {
        let int_vector = TokenData {
            kind: Token::VectorLiteral,
            source: "[1, 2, 3, 4, 5]",
            range: (0, 14),
        };

        let vec_literal = vec![
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(1)),
            },
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(2)),
            },
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(3)),
            },
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(4)),
            },
            Literal {
                kind: LiteralKind::Int,
                value: LiteralValue::Int(IntType::_i32(5)),
            },
        ];

        assert_eq!(
            literal(&int_vector),
            Literal {
                kind: LiteralKind::Vector,
                value: LiteralValue::Vector(vec_literal),
            }
        );
    }

    fn float_vector() {
        let float_vector = TokenData {
            kind: Token::VectorLiteral,
            source: "[1.23, 2.34, 3.45, 4.56, 5.67]",
            range: (0, 14),
        };

        let vec_literal = vec![
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(1.23)),
            },
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(2.34)),
            },
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(3.45)),
            },
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(4.56)),
            },
            Literal {
                kind: LiteralKind::Float,
                value: LiteralValue::Float(FloatType::_f32(5.67)),
            },
        ];

        assert_eq!(
            literal(&float_vector),
            Literal {
                kind: LiteralKind::Vector,
                value: LiteralValue::Vector(vec_literal),
            }
        );
    }

    fn string_vector() {
        let string_vector = TokenData {
            kind: Token::VectorLiteral,
            source: "[\"test\", \"test\", \"test\", \"test\", \"test\"]",
            range: (0, 14),
        };

        let vec_literal = vec![
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("test")),
            },
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("test")),
            },
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("test")),
            },
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("test")),
            },
            Literal {
                kind: LiteralKind::String,
                value: LiteralValue::String(String::from("test")),
            },
        ];

        assert_eq!(
            literal(&string_vector),
            Literal {
                kind: LiteralKind::Vector,
                value: LiteralValue::Vector(vec_literal),
            }
        );
    }

    fn bool_vector() {
        let bool_vector = TokenData {
            kind: Token::VectorLiteral,
            source: "[true, false, true, false, true]",
            range: (0, 14),
        };

        let vec_literal = vec![
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(true),
            },
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(false),
            },
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(true),
            },
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(false),
            },
            Literal {
                kind: LiteralKind::Bool,
                value: LiteralValue::Bool(true),
            },
        ];

        assert_eq!(
            literal(&bool_vector),
            Literal {
                kind: LiteralKind::Vector,
                value: LiteralValue::Vector(vec_literal),
            }
        );
    }
}
