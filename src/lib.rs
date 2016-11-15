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
pub fn filter_one<'a, 'b, T, F>(view: &'a DBView<'b, T>, predicate: F) -> DBView<'b, T>
    where F: Fn(&'b T) -> bool
{
    let mut new_view: DBView<'b, T> = DBView::new();
    for record in & view.entries {
        if predicate(record) {
            new_view.entries.push(record);
        }
    }
    new_view
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, T, F>(view_a: &'a DBView<'b, T>,
                        view_b: &'a DBView<'b, T>,
                        predicate: F)
                        -> (DBView<'b, T>, DBView<'b, T>)
    where F: Fn(&'b T) -> bool
{
    let mut new_view_a: DBView<'b, T> = DBView::new();
    for record in & view_a.entries {
        if predicate(record) {
            new_view_a.entries.push(record);
        }
    }
    let mut new_view_b: DBView<'b, T> = DBView::new();
    for record in & view_b.entries {
        if predicate(record) {
            new_view_b.entries.push(record);
        }
    }
    (new_view_a, new_view_b)
}

impl<T> DB<T> {
    /// Creates a DB from the given list of entries
    pub fn new(data: Vec<T>) -> DB<T> {
        DB{data: data}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<'a, F>(&'a self, predicate: F) -> DBView<'a, T>
        where F: Fn(&'a T) -> bool
    {
        let mut new_view: DBView<'a, T> = DBView::new();
        for record in & self.data {
            if predicate(record) {
                new_view.entries.push(record);
            }
        }

        new_view
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<'a, 'b, F>(&'a mut self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&'a T) -> bool
    {
        let mut new_view: DBViewMut<'a, T> = DBViewMut::new();

        for record in &mut self.data {
            if predicate(record) {
                new_view.entries.push(record);
            }
        }

        new_view
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view<'a>(&'a self) -> DBView<'a, T> {
        let mut new_view: DBView<'a, T> = DBView::new();
        for record in & self.data {
            new_view.entries.push(record);
        }

        new_view
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut<'a>(&'a mut self) -> DBViewMut<'a, T> {
        let mut new_view: DBViewMut<'a, T> = DBViewMut::new();

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
    pub fn new() -> Self {
        DBView{entries: vec!()}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where<F>(&'a self, predicate: F) -> DBView<'a, T>
        where F: Fn(&'a T) -> bool
    {
        let mut new_view: DBView<'a, T> = DBView::new();
        for record in & self.entries {
            if predicate(record) {
                new_view.entries.push(record);
            }
        }
        new_view
    }

    /// Returns the number of entries in the DBView
    pub fn len(&'a self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    pub fn new() -> Self {
        DBViewMut{entries: vec!()}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&'a T) -> bool
    {
        let mut new_view: DBViewMut<'a, T> = DBViewMut::new();
        for record in self.entries {
            if predicate(record) {
                new_view.entries.push(record);
            }
        }
        new_view
    }

    /// Returns the number of entries in the DBView
    pub fn len(&'a self) -> usize {
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
