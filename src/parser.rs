peg::parser! {
    pub grammar parser() for str {
        pub rule calc() -> f64
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
              "(" _ c:calc() _ ")" { c }
          }

        rule number() -> f64
          = n:$(['0'..='9']+ ("." ['0'..='9']+)?) {?
              n.parse().or(Err("Can't parse a number"))
          }

        rule _() = quiet!{ " "* }

    }
}
