use std::ops::ControlFlow;

#[derive(PartialEq, Eq)]
pub enum Method {
    Visit,
    Leave,
}

pub trait TreeItem {
    type Id: PartialEq + Eq;

    fn id(&self) -> Self::Id;
}

pub trait TreeIterator {
    type Item: TreeItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item, Method) -> ControlFlow<R>;

    fn search<T>(
        &self,
        key: <Self::Item as TreeItem>::Id,
        action: impl FnOnce(&Self::Item) -> T,
    ) -> Option<T> {
        Self::search_key(&self, |node| node.id() == key, action)
    }
    fn search_key<T>(
        &self,
        mut key: impl FnMut(&Self::Item) -> bool,
        action: impl FnOnce(&Self::Item) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk(&mut |node: &Self::Item, method: Method| {
            if method == Method::Visit && key(&node) {
                ControlFlow::Break(action.take().map(|action| (action)(node)))
            } else {
                ControlFlow::Continue(())
            }
        });
        match output {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(x) => x,
        }
    }
}

pub trait TreeIteratorMut {
    type Item: TreeItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item, Method) -> ControlFlow<R>;

    fn search_mut<T>(
        &mut self,
        key: <Self::Item as TreeItem>::Id,
        action: impl FnOnce(&mut Self::Item) -> T,
    ) -> Option<T> {
        Self::search_key_mut(self, |node| node.id() == key, action)
    }

    fn search_key_mut<T>(
        &mut self,
        mut key: impl FnMut(&Self::Item) -> bool,
        action: impl FnOnce(&mut Self::Item) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk_mut(&mut |node: &mut Self::Item, method: Method| {
            if method == Method::Visit && key(&node) {
                ControlFlow::Break(action.take().map(|action| (action)(node)))
            } else {
                ControlFlow::Continue(())
            }
        });
        match output {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(x) => x,
        }
    }
}
