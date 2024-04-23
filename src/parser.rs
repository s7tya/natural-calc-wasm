peg::parser! {
    pub grammar parser() for str {
        pub rule arithmetic() -> f64
          = precedence! {
              l:(@) _ "+" _ r:@ { l + r }
              l:(@) _ "-" _ r:@ { l - r }
              --
              l:(@) _ "*" _ r:@ { l * r }
              l:(@) _ "/" _ r:@ { l / r }
              --
              l:(@) _ "^" _ r:@ { l.powf(r) }
              l:(@) _ "%" _ r:@ { l % r }
              --
              n:number() { n }
              "(" _ c:arithmetic() _ ")" { c }
          }

        rule number() -> f64
          = n:$(['0'..='9']+ ("." ['0'..='9']+)?) {?
              n.parse().or(Err("Can't parse a number"))
          }

        rule ident() -> &'input str
          = $(quiet!{[ c if c.is_ascii_alphabetic() ][ c if c.is_ascii_alphanumeric() ]*}) / expected!("identifier")

        rule _() = quiet!{ " "* }

    }
}
