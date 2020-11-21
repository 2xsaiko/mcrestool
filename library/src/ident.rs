use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier {
    inner: String,
}

impl Identifier {
    pub fn from_components(namespace: &str, path: &str) -> Self {
        Identifier::from(format!("{}:{}", namespace, path))
    }

    pub fn set_namespace(&mut self, namespace: &str) {
        match self.get_seperator() {
            None => {
                self.inner.insert(0, ':');
                self.inner.insert_str(0, namespace);
            }
            Some(idx) => {
                self.inner.replace_range(0..idx, namespace);
            }
        }
    }

    pub fn set_path(&mut self, path: &str) {
        match self.get_seperator() {
            None => {
                self.inner.clear();
                self.inner.push_str(path);
            }
            Some(idx) => {
                self.inner.replace_range(idx + 1.., path);
            }
        }
    }

    pub fn into_inner(self) -> String { self.inner }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier { inner: s }
    }
}

impl From<&Ident> for Identifier {
    fn from(id: &Ident) -> Self {
        id.to_identifier()
    }
}

impl Deref for Identifier {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        Ident::new(&self.inner)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[repr(transparent)]
pub struct Ident {
    inner: str,
}

impl Ident {
    /// Directly wraps a string slice as an `Ident` slice.
    ///
    /// Works identically to [`std::path::Path::new`]
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const str as *const Ident) }
    }

    pub fn namespace(&self) -> &str {
        self.get_seperator().map(|idx| &self.inner[..idx]).unwrap_or("minecraft")
    }

    pub fn path(&self) -> &str {
        &self.inner[self.get_seperator().map(|idx| idx + 1).unwrap_or(0)..]
    }

    pub fn as_str(&self) -> &str { &self.inner }

    pub fn to_identifier(&self) -> Identifier {
        Identifier::from(self.inner.to_string())
    }

    fn get_seperator(&self) -> Option<usize> {
        self.inner.find(':')
    }
}