use std::vec;
use std::slice;

#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T: 'a> {
    entries: Vec<&'a T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T: 'a> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
///
/// NB: (nota bene, or 'take special note'): You should modify the signature so that there is **no
/// lifetime elision**
pub fn filter_one<'a, T, F>(view: &DBView<'a, T>, predicate: F) -> DBView<'a, T>
    where F: Fn(&T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
///
/// NB: Modify the signature so that there is **no lifetime elision**
pub fn filter_two<'a, 'b, T, F>(view_a: &DBView<'a, T>,
                                view_b: &DBView<'b, T>,
                                predicate: F)
                                -> (DBView<'a, T>, DBView<'b, T>)
    where F: Fn(&T) -> bool
{
    (view_a.select_where(&predicate), view_b.select_where(&predicate))
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB { data: data }
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn select_where<F>(&self, predicate: F) -> DBView<T>
        where F: Fn(&T) -> bool
    {
        DBView { entries: self.data.iter().filter(|item| predicate(item)).collect() }
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn select_where_mut<F>(&mut self, predicate: F) -> DBViewMut<T>
        where F: Fn(&T) -> bool
    {
        DBViewMut { entries: self.data.iter_mut().filter(|item| predicate(item)).collect() }
    }

    /// Returns a DBView consisting on the entirety of `self`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn as_view(&self) -> DBView<T> {
        DBView { entries: self.data.iter().collect() }
    }

    /// Returns a DBView consisting on the entirety of `self`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn as_view_mut(&mut self) -> DBViewMut<T> {
        DBViewMut { entries: self.data.iter_mut().collect() }
    }

    /// Returns the number of entries in the DB
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn select_where<F>(&self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        DBView { entries: self.entries.iter().cloned().filter(|item| predicate(item)).collect() }
    }

    /// Returns the number of entries in the DBView
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        DBViewMut {
            entries: self.entries
                .into_iter()
                .filter(|item: &&mut T| predicate(*item))
                .collect(),
        }
    }

    /// Returns the number of entries in the DBView
    ///
    /// NB: Modify the signature so that there is **no lifetime elision**
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}


impl<T> IntoIterator for DB<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DB<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DB<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<'a, T> IntoIterator for DBView<'a, T> {
    type Item = &'a T;
    type IntoIter = vec::IntoIter<&'a T>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, T> IntoIterator for DBViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = vec::IntoIter<&'a mut T>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

struct Closure<E, F> {
    environment: E,
    func: F,
}

impl<E, F, O> Closure<E, F> where F: for <'a> Fn(&'a E) -> &'a O {
    fn call(&self) -> &O {
        (self.func)(&self.environment)
    }
}
