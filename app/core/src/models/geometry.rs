#[derive(Clone, Copy, Debug)]
pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Size { w, h }
    }
}


