#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: (nota bene, or "take special note"): You will need to be explcit about the liftimes in this
/// struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T:'a> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explicit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T:'a> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, T, F>(view: &DBView<'a,T>, predicate: F) -> DBView<'a, T>
    where F: Fn(&T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, T, F>(view_a: &DBView<'a,T>,
                        view_b:  &DBView<'b,T>,
                        predicate: F)
                        -> (DBView<'a, T>, DBView<'b, T>)
    where F: Fn(&T) -> bool
{
    (view_a.select_where(&predicate), view_b.select_where(&predicate))
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB{data: data}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<F>(&self, predicate: F) -> DBView<T>
        where F: Fn(&T) -> bool
    {
        let mut new_vec = Vec::new();
        for x in &self.data {
            if predicate(x){
                new_vec.push(x);
            }
        }
        DBView{entries: new_vec}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a,F>(&'a mut self, predicate: F) -> DBViewMut<'a,T>
        where F: Fn(&T) -> bool
    {
        let mut new_vec = Vec::new();
        for x in &mut self.data {
            if predicate(x){
                new_vec.push(x);
            }
        }
        DBViewMut{entries: new_vec}
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view<'a>(&'a self) -> DBView<'a, T> {
        let mut new_vec = Vec::new();
        for x in &self.data {
            new_vec.push(x);
        }
        DBView{entries: new_vec}
    }

    /// Returns a DBViewMut consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut new_vec = Vec::new();
        for x in &mut self.data {
            new_vec.push(x);
        }
        DBViewMut{entries: new_vec}
    }

    /// Returns the number of entries in the DB
    pub fn len(&self) -> usize {
        (*self).data.len()
    }
}

impl<'a,T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<F>(&self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut new_vec = Vec::new();
        for x in &self.entries {
            if predicate(x){
                new_vec.push(*x);
            }
        }
        DBView{entries: new_vec}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a,T> DBViewMut<'a,T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut new_vec = Vec::new();
        for x in self.entries{
            if predicate(x){
                new_vec.push(x);
            }
        }
        DBViewMut{entries: new_vec}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

// Bonus A
//
// impl<T> IntoIterator for DB<T> {
//     type Item = T;
//     // TODO
// }
//
// impl<T> IntoIterator for &DB<T> {
//     type Item = &T;
//     // TODO
// }
//
// impl<T> IntoIterator for &mut DB<T> {
//     type Item = &mut T;
//     // TODO
// }
//
// impl<T> IntoIterator for DBView<T> {
//     type Item = &T;
//     // TODO
// }
//
// impl<T> IntoIterator for DBViewMut<T> {
//     type Item = &mut T;
//     // TODO
// }
