#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: (nota bene, or "take special note"): You will need 
/// to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T:'a> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T:'a> {
    entries: Vec<&'a mut T>,
}

// TODO change these functions to use iter filter and iter collect instead
/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, 'b, T, F>(view: &'b DBView<'a, T>,
                                predicate: F) -> DBView<'a, T>
    where F: Fn(&T) -> bool
{
    let mut vec : Vec<&'a T> = Vec::new();
    for elem in &view.entries {
        if predicate(elem) {
            vec.push(elem)
        }
    }
    DBView{entries: vec}
}

/// Filters two DBView structs using the same predicate, producing two separate
/// results. This is the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, 'c, T, F>(view_a: &'c DBView<'a, T>,
                        view_b: &DBView<'b, T>,
                        predicate: F)
                        -> (DBView<'a, T>, DBView<'b, T>)
    where F: Fn(&T) -> bool
{
    let mut vec_a : Vec<&'a T> = Vec::new();
    let mut vec_b : Vec<&'b T> = Vec::new();
    for elem in &view_a.entries {
        if predicate(elem) {
            vec_a.push(elem)
        }
    }
    for elem in &view_b.entries {
        if predicate(elem) {
            vec_b.push(elem)
        }
    }
    (DBView{entries: vec_a}, DBView{entries: vec_b})
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB{data: data}
    }

    /// Creates a new DBView containing all entries in `self`
    /// which satisfy `predicate`
    pub fn select_where<'a, F>(&'a self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut vec : Vec<&'a T> = Vec::new();
        for elem in &self.data {
          if predicate(elem) {
              vec.push(elem)
          }
        }
        DBView{entries: vec}
    }

    /// Creates a new DBView containing all entries in `self` 
    /// which satisfy `predicate`
    pub fn select_where_mut<'a, F>(&'a mut self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut vec : Vec<&'a mut T> = Vec::new();
        for elem in &mut self.data {
          if predicate(elem) {
              vec.push(elem)
          }
        }
        DBViewMut{entries: vec}
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view<'a>(&'a self) -> DBView<'a, T> {
        let mut vec : Vec<&'a T> = Vec::new();
        for elem in &self.data {
            vec.push(elem)
        }
        DBView{entries: vec}
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut vec : Vec<&'a mut T> = Vec::new();
        for elem in &mut self.data {
            vec.push(elem)
        }
        DBViewMut{entries: vec}
    }

    /// Returns the number of entries in the DB
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> DBView<'a, T> {
    /// Creates a new DBView containing all entries in `self`
    /// which satisfy `predicate`
    pub fn select_where<F>(&self, predicate: F) -> DBView<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut vec : Vec<&'a T> = Vec::new();
        for elem in &self.entries {
            if predicate(elem) {
                vec.push(elem)
            }
        }
        DBView{entries: vec}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which
    /// satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        let mut vec : Vec<&'a mut T> = Vec::new();
        for elem in self.entries {
            if predicate(elem) {
                vec.push(elem)
            }
        }
        DBViewMut{entries: vec}
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
