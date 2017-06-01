
trait ForceUnwrap {
    type Target;
    fn uw(self, msg: &str) -> Self::Target;
}

impl<T> ForceUnwrap for Option<T> {
    type Target = T;
    fn uw(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => { error!("option fail: {}", msg); panic!("{}", msg) },
        }
    }
}
