#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T: 'a> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T: 'a> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, 'b: 'a, T, F>(view: &'a DBView<'b, T>, predicate: F) -> DBView<'b, T>
    // 'for<...>' syntax explained here:
    // https://doc.rust-lang.org/book/closures.html
    // 
    // and in more detail here:
    // https://doc.rust-lang.org/stable/nomicon/hrtb.html
    where F: for<'c> Fn(&'c T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a1, 'b1: 'a1, 'a2, 'b2: 'a2, T, F>(view_a: &'a1 DBView<'b1, T>,
                                                      view_b: &'a2 DBView<'b2, T>,
                                                      predicate: F)
                                                      -> (DBView<'b1, T>, DBView<'b2, T>)
    where F: for<'c> Fn(&'c T) -> bool
{
    (filter_one(view_a, &predicate), filter_one(view_b, &predicate))
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB::<T>{ data: data }
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'a, F>(&'a self, predicate: F) -> DBView<'a, T>
        where F: for<'c> Fn(&'c T) -> bool
    {
        self.as_view().select_where(predicate)
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a, F>(&'a mut self, predicate: F) -> DBViewMut<'a, T>
        where F: for<'c> Fn(&'c T) -> bool
    {
        self.as_view_mut().select_where_mut(predicate)
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view<'a>(&'a self) -> DBView<'a, T> {
        let mut v = DBView::<'a, T>{ entries: Vec::new() };
        for i in &self.data {
            v.entries.push(i);
        }
        v
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut v = DBViewMut::<'a, T>{ entries: Vec::new() };
        for i in &mut self.data {
            v.entries.push(i);
        }
        v
    }

    /// Returns the number of entries in the DB
    pub fn len<'a> (&'a self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'b, F>(&'b self, predicate: F) -> DBView<'a, T>
        where F: for<'c> Fn(&'c T) -> bool
    {
        DBView::<'a, T> { entries: self.entries.iter().cloned().filter(|x| predicate(x)).collect() }
    }
    
    /// Returns the number of entries in the DBView
    pub fn len<'b>(&'b self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: for<'c> Fn(&'c T) -> bool
    {
        DBViewMut::<'a, T> { entries: self.entries.into_iter().filter(|x| predicate(x)).collect() }
    }

    /// Returns the number of entries in the DBView
    pub fn len<'b>(&'b self) -> usize {
        self.entries.len()
    }
}

// Bonus A
impl<T> IntoIterator for DB<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

impl<'a, T> IntoIterator for &'a DB<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return (&self.data).into_iter();
    }
}

impl<'a, T> IntoIterator for &'a mut DB<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return (&mut self.data).into_iter();
    }
}

impl<'a, T> IntoIterator for DBView<'a, T> {
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.entries.into_iter();
    }
}

impl<'a, T> IntoIterator for DBViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.entries.into_iter();
    }
}
