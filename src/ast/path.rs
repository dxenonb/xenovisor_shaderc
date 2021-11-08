use std::{fmt, hash::{Hash}};

#[derive(Clone)]
pub struct Path {
    def: String,
    slices: Vec<[usize; 2]>,
}

impl Path {
    // fn empty() -> Path {
    //     let def = String::new();
    //     let slices = Vec::new();
    //     Path {
    //         def,
    //         slices,
    //     }
    // }

    pub fn component(&self, i: usize) -> Option<&str> {
        let i = self.slices.get(i)?;
        Some(&self.def[i[0]..i[1]])
    }

    pub fn components(&self) -> impl Iterator<Item=&str> {
        self.slices.iter()
            .map(move |slice| &self.def[slice[0]..slice[1]])
    }

    pub fn len(&self) -> usize {
        self.slices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slices.len() == 0
    }

    pub fn parent(&mut self) -> bool {
        self.slices.pop().is_some()
    }

    pub fn as_slice(&self) -> PathSlice {
        PathSlice {
            path: self,
            rem: &self.slices[..],
        }
    }
}

impl<S: AsRef<str>, I: Iterator<Item=S>> From<I> for Path {
    fn from(iter: I) -> Self {
        let components_estimate = iter.size_hint().0;
        let mut def = String::with_capacity(components_estimate * 2);
        let mut slices = Vec::with_capacity(components_estimate);

        for i in iter {
            let s = i.as_ref();
            let slice = [def.len(), def.len() + s.len()];
            def.push_str(s);
            slices.push(slice);
        }

        Path {
            def,
            slices
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        for (lhs, rhs) in self.components().zip(other.components()) {
            if lhs != rhs {
                return false;
            }
        }
        return true;
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.components() {
            i.hash(state);
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "ast::Path(<empty>)");
        }

        write!(f, "ast::Path({}", self.component(0).unwrap())?;

        if self.slices.len() == 1 {
            return write!(f, ")");
        }

        let slices = &self.slices[1..];
        for i in slices {
            let s = &self.def[i[0]..i[1]];
            write!(f, "::{}", s)?;
        }

        write!(f, ")")?;
        Ok(())
    }
}

pub struct PathSlice<'a> {
    path: &'a Path,
    rem: &'a [[usize; 2]],
}

impl PathSlice<'_> {
    pub fn pop(&mut self) -> bool {
        if self.rem.len() <= 1 {
            return false;
        }
        self.rem = &self.rem[..self.rem.len() - 2];

        self.rem.len() >= 1
    }

    pub fn component(&self, i: usize) -> Option<&str> {
        let i = self.rem.get(i)?;
        Some(&self.path.def[i[0]..i[1]])
    }

    pub fn len(&self) -> usize {
        self.rem.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rem.len() == 0
    }
}

impl fmt::Debug for PathSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "ast::Path(<empty>)");
        }

        write!(f, "ast::Path({}", self.component(0).unwrap())?;

        if self.rem.len() == 1 {
            return write!(f, ")");
        }

        let slices = &self.rem[1..];
        for i in slices {
            let s = &self.path.def[i[0]..i[1]];
            write!(f, "::{}", s)?;
        }

        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ast::parse_path;

    #[test]
    fn path_compare() {
        let p1 = parse_path("use main::vert;").unwrap();
        let p2 = parse_path("use main::vert;").unwrap();
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }

    #[test]
    fn path_compare_modified() {
        let p1 = parse_path("use main;").unwrap();
        let mut p2 = parse_path("use main::vert;").unwrap();
        p2.parent();
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }
}
