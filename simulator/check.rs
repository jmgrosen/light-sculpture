macro_rules! check(
    ($inp:expr) => (
        {
            let ret = $inp;
            match es::get_error() {
                0 => ret,
                err => fail!("OpenGL error: %?", err)
            }
        }
    )
)
