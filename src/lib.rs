#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T> where T: 'a {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a,T> where T: 'a {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, T, F>(view: &DBView<'a, T>, predicate: F) -> DBView<'a, T>
    where F: Fn(&T) -> bool
{
    DBView{entries: view.entries.clone().into_iter().filter(|&x| predicate(x)).collect()}
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, T, F>(view_a: &DBView<'a, T>,
                        view_b: &DBView<'b, T>,
                        predicate: F)
                        -> (DBView<'a, T>, DBView<'b, T>)
    where F: Fn(&T) -> bool
{
    (DBView{entries: view_a.entries.clone().into_iter().filter(|&x| predicate(x)).collect()},
     DBView{entries: view_b.entries.clone().into_iter().filter(|&x| predicate(x)).collect()})
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB{data: data}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'a, F>(&'a self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        self.as_view().select_where(predicate)
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a, F>(&'a mut self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        self.as_view_mut().select_where_mut(predicate)
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view<'a>(&'a self) -> DBView<'a, T> {
        let mut ret = DBView{entries: Vec::new()};
        for v in &self.data {
            ret.entries.push(v);
        }
        ret
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut ret = DBViewMut{entries: Vec::new()};
        for v in &mut self.data {
            ret.entries.push(v);
        }
        ret
    }

    /// Returns the number of entries in the DB
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<F>(&self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        DBView{entries: self.entries
            .iter().filter(|x| predicate(x)).cloned().collect()}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        DBViewMut{entries: self.entries.into_iter().filter(|x| predicate(x)).collect()}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

// Bonus A

impl<T> IntoIterator for DB<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DB<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DB<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;
    fn into_iter(mut self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<'a, T> IntoIterator for DBView<'a, T> {
    type Item = &'a T;
    type IntoIter = ::std::vec::IntoIter<&'a T>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, T> IntoIterator for DBViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = ::std::vec::IntoIter<&'a mut T>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
