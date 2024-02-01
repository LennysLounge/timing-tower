use std::ops::ControlFlow;

use uuid::Uuid;

#[derive(PartialEq, Eq)]
pub enum Method {
    Visit,
    Leave,
}

pub trait TreeItem {
    fn id(&self) -> Uuid;
}

pub trait TreeIterator {
    type Item<'item>: TreeItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>;

    fn search<T>(&self, node_id: Uuid, action: impl FnOnce(&Self::Item<'_>) -> T) -> Option<T> {
        Self::search_key(&self, |node| node.id() == node_id, action)
    }
    fn search_key<T>(
        &self,
        mut key: impl FnMut(&Self::Item<'_>) -> bool,
        action: impl FnOnce(&Self::Item<'_>) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk(&mut |node: &Self::Item<'_>, method: Method| {
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
    type Item<'item>: TreeItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, Method) -> ControlFlow<R>;

    fn search_mut<T>(
        &mut self,
        node_id: Uuid,
        action: impl FnOnce(&mut Self::Item<'_>) -> T,
    ) -> Option<T> {
        Self::search_key_mut(self, |node| node.id() == node_id, action)
    }

    fn search_key_mut<T>(
        &mut self,
        mut key: impl FnMut(&Self::Item<'_>) -> bool,
        action: impl FnOnce(&mut Self::Item<'_>) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk_mut(&mut |node: &mut Self::Item<'_>, method: Method| {
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
