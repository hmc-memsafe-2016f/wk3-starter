#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: (nota bene, or "take special note"): You will need to be explcit about the liftimes in this
/// struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T: 'a> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explicit about the lifetimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T: 'a> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, 'b: 'a, T, F>(view: &'a DBView<'b, T>, predicate: F) -> DBView<'a, T>
    where F: for<'c> Fn(&'c T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, T, F>(view_a: &DBView<'a, T>,
                        view_b: &DBView<'b, T>,
                        predicate: F)
                        -> (DBView<'a, T>, DBView<'b, T>)
    where F: for<'c> Fn(&'c T) -> bool
{
    // couldn't get this to work like filter_one, or by calling filter_one
    (DBView::<'a, T>{entries: view_a.entries
                          .clone()
                          .into_iter()
                          .filter(|&x| predicate(x))
                          .collect()},
     DBView::<'b, T>{entries: view_b.entries
                          .clone()
                          .into_iter()
                          .filter(|&x| predicate(x))
                          .collect()})
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB::<T>{data: data}
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
        let mut new_view = DBView::<'a, T>{entries: Vec::new()};
        for record in & self.data {
            new_view.entries.push(record);
        }

        new_view
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut new_view = DBViewMut::<'a, T>{entries: Vec::new()};
        for record in &mut self.data {
            new_view.entries.push(record);
        }

        new_view
    }

    /// Returns the number of entries in the DB
    pub fn len<'a>(&'a self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'b, F>(&'b self, predicate: F) -> DBView<'a, T>
        where F: for<'c> Fn(&'c T) -> bool
    {
        DBView::<'a, T>{entries: self.entries
                            .iter()
                            .filter(|x| predicate(x))
                            .cloned()
                            .collect()}
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
        DBViewMut::<'a, T>{entries: self.entries
                               .into_iter()
                               .filter(|x| predicate(x))
                               .collect()}
    }

    /// Returns the number of entries in the DBView
    pub fn len<'b>(&'b self) -> usize {
        self.entries.len()
    }
}

// Bonus A
//
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
    fn into_iter(self) -> Self::IntoIter {
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
