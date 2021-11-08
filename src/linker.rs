use crate::hir;

pub struct Linker {

}

impl Linker {
    pub fn deps(&self, function: &hir::Function) -> Deps {
        Deps {}
    }
}

pub struct Deps {

}

impl Deps {
    pub fn globals(&self) -> impl IntoIterator<Item=&hir::Global> {
        []
    }

    pub fn functions(&self) -> impl IntoIterator<Item=&hir::Function> {
        []
    }
}
