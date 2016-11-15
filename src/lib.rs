#[derive(Debug, PartialEq, Eq)]
pub struct DB<T> {
    data: Vec<T>,
}

/// An immutably borrowed subset of a DB
///
/// NB: (nota bene, or "take special note"): You will need to be explcit about the lifetimes in this
/// struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBView<'a, T: 'a> {
    entries: Vec<&'a T>,
}

/// An mutably borrowed subset of a DB
///
/// NB: You will need to be explcit about the liftimes in this struct
#[derive(Debug, PartialEq, Eq)]
pub struct DBViewMut<'a, T: 'a> {
    entries: Vec<&'a mut T>,
}

/// Filters a DBView using the the given predicate.
pub fn filter_one<'a, 'b, T, F>(view: &'a DBView<'b, T>, predicate: F) -> DBView<'b, T>
    where F: Fn(&T) -> bool
{
    view.select_where(predicate)
}

/// Filters two DBView structs using the same predicate, producing two separate results. This is
/// the moral equivalent of doing the two filters separately.
pub fn filter_two<'a, 'b, 'c, 'd, T, F>(view_a: &'a DBView<'b, T>,
                        view_b: &'c DBView<'d, T>,
                        predicate: F)
                        -> (DBView<'b, T>, DBView<'d, T>)
    where F: Fn(&T) -> bool
{
	(DBView{entries: view_a.entries.iter().cloned().filter(|x| predicate(x)).collect()},
		 DBView{entries: view_b.entries.iter().cloned().filter(|x| predicate(x)).collect()})
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
        DBView{entries: self.data.iter().filter(|x| predicate(x)).collect()}
    }

    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(&mut self, predicate: F) -> DBViewMut<T>
        where F: Fn(&T) -> bool
    {
    	DBViewMut{entries: self.data.iter_mut().filter(|x| predicate(x)).collect()}	
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view(&self) -> DBView<T> {
        DBView{entries: self.data.iter().collect()}
    }

    /// Returns a DBView consisting on the entirety of `self`
    pub fn as_view_mut(&mut self) -> DBViewMut<T> {
        DBViewMut{entries: self.data.iter_mut().collect()}
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
        DBView{entries: self.entries.iter().cloned().filter(|x| predicate(x)).collect()}
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, T> DBViewMut<'a, T> {
    /// Creates a new DBView containing all entries in `self` which satisfy `predicate`
    pub fn select_where_mut<F>(mut self, predicate: F) -> DBViewMut<'a, T>
        where F: Fn(&T) -> bool
    {
        self.entries.retain(|x| predicate(x));
        self
    }

    /// Returns the number of entries in the DBView
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

 //Bonus A

 impl<T> IntoIterator for DB<T> {
     type Item = T;
     type IntoIter = ::std::vec::IntoIter<T>;

     fn into_iter(self) -> Self::IntoIter
     {
     	self.data.into_iter()
     }

 }

 impl<'a, T> IntoIterator for &'a DB<T> {
     type Item = &'a T;
     type IntoIter = ::std::slice::Iter<'a, T>;

     fn into_iter(self) -> Self::IntoIter
     {
     	(&self.data).into_iter()
     }
 }

 impl<'a, T> IntoIterator for &'a mut DB<T> {
     type Item = &'a mut T;
     type IntoIter = ::std::slice::IterMut<'a, T>;

  	fn into_iter(self) -> Self::IntoIter
    {
    	(&mut self.data).into_iter()
    }
 }

 impl<'a, T> IntoIterator for DBView<'a, T> {
     type Item = &'a T;
     type IntoIter = ::std::vec::IntoIter<&'a T>;

	fn into_iter(self) -> Self::IntoIter
    {
    	self.entries.into_iter()
    }
 }

 impl<'a, T> IntoIterator for DBViewMut<'a, T> {
     type Item = &'a mut T;
     type IntoIter = ::std::vec::IntoIter<&'a mut T>;

     fn into_iter(self) -> Self::IntoIter
     {
     	self.entries.into_iter()
     }
 }
