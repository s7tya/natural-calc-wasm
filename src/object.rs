use std::ops;

#[derive(Debug, Clone)]
pub enum Object {
    Void,
    Int(i64),
    Float(f64),
}

impl ops::Add<Object> for Object {
    type Output = Object;

    fn add(self, rhs: Object) -> Self::Output {
        let left = match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        let right = match rhs {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        Object::Float(left + right)
    }
}

impl ops::Sub for Object {
    type Output = Object;

    fn sub(self, rhs: Object) -> Self::Output {
        let left = match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        let right = match rhs {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        Object::Float(left - right)
    }
}

impl ops::Mul for Object {
    type Output = Object;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        let right = match rhs {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        Object::Float(left * right)
    }
}

impl ops::Div for Object {
    type Output = Object;

    fn div(self, rhs: Self) -> Self::Output {
        let left = match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        let right = match rhs {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        Object::Float(left / right)
    }
}

impl ops::Rem for Object {
    type Output = Object;

    fn rem(self, rhs: Self) -> Self::Output {
        let left = match self {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        let right = match rhs {
            Self::Int(v) => v as f64,
            Self::Float(v) => v,
            Self::Void => {
                panic!()
            }
        };

        Object::Float(left % right)
    }
}
