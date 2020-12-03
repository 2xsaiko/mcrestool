use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use ffmtutil::{BinDeserialize, BinSerialize, BinSerializer};

#[derive(Debug, Clone, Eq, BinDeserialize)]
pub struct Identifier {
    inner: String,
}

impl Identifier {
    pub fn from_components(namespace: &str, path: &str) -> Self {
        Identifier::from(format!("{}:{}", namespace, path))
    }

    /// Sets the namespace of this `Identifier`, or removes it if `namespace` is
    /// `None`.
    pub fn set_namespace(&mut self, namespace: Option<&str>) {
        match namespace {
            None => {
                if let Some(idx) = self.get_seperator() {
                    self.inner.drain(..=idx);
                }
            }
            Some(namespace) => match self.get_seperator() {
                None => {
                    self.inner.insert(0, ':');
                    self.inner.insert_str(0, namespace);
                }
                Some(idx) => {
                    self.inner.replace_range(..idx, namespace);
                }
            },
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

    pub fn into_inner(self) -> String {
        self.inner
    }

    pub fn as_ident(&self) -> &Ident {
        Ident::new(&self.inner)
    }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier { inner: s }
    }
}

impl Borrow<Ident> for Identifier {
    fn borrow(&self) -> &Ident {
        self.as_ident()
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
        self.as_ident()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_ident().fmt(f)
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ident().hash(state);
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.as_ident() == other.as_ident()
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ident().cmp(other.as_ident())
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ident().partial_cmp(other.as_ident())
    }
}

impl BinSerialize for Identifier {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> ffmtutil::Result<()> {
        (**self).serialize(serializer)
    }
}

#[repr(transparent)]
#[derive(Debug, Eq)]
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
        self.namespace_raw().unwrap_or("minecraft")
    }

    pub fn namespace_raw(&self) -> Option<&str> {
        self.get_seperator().map(|idx| &self.inner[..idx])
    }

    pub fn path(&self) -> &str {
        &self.inner[self.get_seperator().map(|idx| idx + 1).unwrap_or(0)..]
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn trim(&self) -> &Ident {
        match self.namespace_raw() {
            Some("minecraft") => Ident::new(self.path()),
            _ => self,
        }
    }

    pub fn to_identifier(&self) -> Identifier {
        Identifier::from(self.inner.to_string())
    }

    fn get_seperator(&self) -> Option<usize> {
        self.inner.find(':')
    }
}

impl ToOwned for Ident {
    type Owned = Identifier;

    fn to_owned(&self) -> Self::Owned {
        self.to_identifier()
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespace().hash(state);
        self.path().hash(state);
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.namespace() == other.namespace() && self.path() == other.path()
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.namespace().cmp(other.namespace()) {
            Ordering::Equal => self.path().cmp(other.path()),
            x @ _ => x,
        }
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.namespace(), self.path())
    }
}

impl BinSerialize for Ident {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> ffmtutil::Result<()> {
        self.trim().as_str().serialize(serializer)
    }
}
