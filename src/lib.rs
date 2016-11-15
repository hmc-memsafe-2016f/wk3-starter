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
    entries: Vec<&'a T>
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T> where T : 'a {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, 'b, T, F>(view: &'b DBView<'a, T>, predicate: F) -> DBView<'a, T>
    where F: Fn(&T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, 'c, T, F>(view_a: &'c DBView<'a, T>,
                        view_b: &'c DBView<'b, T>,
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
    pub fn select_where<'a, 'b, F>(&'a self, predicate: F) -> DBView<'b, T>
        where F: Fn(&T) -> bool,
              'a : 'b
    {
        let mut entries_vec = Vec::new();
        for x in &(self.data) {
            if predicate(x) {
                entries_vec.push(x);
            }
        }
        DBView{entries: entries_vec}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a, 'b, F>(&'a mut self, predicate: F) -> DBViewMut<'b, T>
        where F: Fn(&T) -> bool, 'a : 'b
    {
        let mut entries_vec = Vec::new();
        for x in &mut(self.data) {
            if predicate(x) {
                entries_vec.push(x);
            }
        }
        DBViewMut{entries: entries_vec}
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view(&self) -> DBView<T> {
        self.select_where(|_| true)
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut(&mut self) -> DBViewMut<T> {
        self.select_where_mut(|_| true)
    }

    /// Returns the number of entries in the DB
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'b, F>(&'b self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut entries_vec = Vec::new();
        for x in & self.entries {
            if predicate(x) {
                entries_vec.push(*x);
            }
        }
        DBView{entries: entries_vec}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a,T>
        where F: Fn(&T) -> bool
    {
        // got idea to use retain from https://www.reddit.com/r/rust/comments/2u42oy/remove_elements_from_a_vector_based_on_a_condition/
        // self.entries.retain(|&x| predicate(x)); <----- Why does this not work?
        DBViewMut{entries: self.entries.into_iter().filter(|ref x| predicate(x)).collect::<Vec<_>>()}
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
