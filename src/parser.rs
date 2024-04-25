use crate::ast::*;

peg::parser! {
    pub grammar parser() for str {
        pub rule program() -> Program
          = __? s:statement() ** __ { s } / expected!("Program")

        rule statement() -> Statement
          = a:assign() { Statement::Assign(a) } / e:expr() { Statement::Expr(e) }

        rule assign() -> Assign
          = i:ident() _ "=" _ e:expr() { Assign { target: i, value: e }}

        rule expr() -> Expr
          = precedence! {
            l:(@) _ "+" _ r:@ { Expr::BinExp(BinExp{ op: "+".to_string(), left: Box::new(l), right: Box::new(r)})}
            l:(@) _ "-" _ r:@ { Expr::BinExp(BinExp{ op: "-".to_string(), left: Box::new(l), right: Box::new(r)})}
            --
            l:(@) _ "*" _ r:@ { Expr::BinExp(BinExp{ op: "*".to_string(), left: Box::new(l), right: Box::new(r)})}
            l:(@) _ "/" _ r:@ { Expr::BinExp(BinExp{ op: "/".to_string(), left: Box::new(l), right: Box::new(r)})}
            l:(@) _ "%" _ r:@ { Expr::BinExp(BinExp{op: "%".to_string(), left: Box::new(l), right: Box::new(r)}) }
            --
            l:(@) _ "^" _ r:@ { Expr::BinExp(BinExp{op: "^".to_string(), left: Box::new(l), right: Box::new(r)}) }
            --
            l:literal() { Expr::Literal(l) }
            i:ident() { Expr::Ident(i) }
            "(" _ e:expr() _ ")" { e }
          }

        rule ident() -> Ident
          = i:$(['a'..='z']+) { Ident(i.to_string()) } / expected!("Identifier")

        rule literal() -> Literal
          =  float() / number() / string()

        rule string() -> Literal
          = "\"" s:$((!"\"" [_])+) "\"" { Literal::String(s.to_string())}

        rule number() -> Literal
          = n:$("0" / ['1'..='9']['0'..='9']*) {?
            let value: i64 = n.parse().or(Err("Can't parse a number"))?;
            Ok(Literal::Int(value))
          }

        rule float() -> Literal
          = n:$(['0'..='9']+ "." ['0'..='9']+) {?
                let value: f64 = n.parse().or(Err("Can't parse a number"))?;
                Ok(Literal::Float(value))
          }

        rule _() = quiet!{ ([' ' | '\t'] / comment())* }

        rule __() = quiet!{ ([' ' | '\t' | '\n' ] / comment())* }

        rule comment() = quiet!{ "//" (!"\n" [_])* ("\n" / ![_]) }
    }
}
