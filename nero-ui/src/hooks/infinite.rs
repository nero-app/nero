use std::future::Future;

use futures::future::{FutureExt, LocalBoxFuture};
use nero_extensions::{
    types::{Episode, Page, Series},
    Extension as ExtensionTrait,
};
use sycamore::{
    futures::create_suspense_task,
    prelude::{batch, create_effect, create_signal, ReadSignal, Signal},
};
use wasm_bindgen::UnwrapThrowExt;

use crate::extensions::Extension;

pub struct InfinitePage<T: 'static> {
    items: Signal<Vec<T>>,
    current_page: Signal<u16>,
    is_loading: Signal<bool>,
    has_next_page: Signal<bool>,
    #[allow(clippy::type_complexity)]
    refetch: Signal<Box<dyn FnMut(u16) -> LocalBoxFuture<'static, Page<T>>>>,
}

impl<T> InfinitePage<T> {
    fn new<F, Fut>(mut refetch: F) -> Self
    where
        F: FnMut(u16) -> Fut + 'static,
        Fut: Future<Output = Page<T>> + 'static,
    {
        let current_page = create_signal(1);
        Self {
            items: create_signal(vec![]),
            current_page,
            is_loading: create_signal(true),
            has_next_page: create_signal(false),
            refetch: create_signal(Box::new(move |_| refetch(current_page.get()).boxed_local())),
        }
    }

    fn always_refetch(self) -> Self {
        create_effect(move || {
            self.is_loading.set(true);
            let fut = self.refetch.update_silent(|f| f(self.current_page.get()));

            create_suspense_task(async move {
                let value = fut.await;
                batch(move || {
                    self.items.update(|prev| prev.extend(value.items));
                    self.has_next_page.set(value.has_next_page);
                    self.is_loading.set(false);
                });
            });
        });

        self
    }

    pub fn load_more(&self) {
        if self.has_next_page.get() {
            self.current_page.update(|v| *v += 1);
        }
    }

    pub fn items(&self) -> ReadSignal<Vec<T>> {
        *self.items
    }
}

// TODO: Handle errors

// TODO: Filters
pub fn use_infinite_search(query: String) -> InfinitePage<Series> {
    InfinitePage::new(move |page| {
        let query = query.clone();
        async move {
            Extension
                .search(&query, Some(page), vec![])
                .await
                .unwrap_throw()
        }
    })
    .always_refetch()
}

pub fn use_infinite_episodes(series_id: String) -> InfinitePage<Episode> {
    InfinitePage::new(move |page| {
        let series_id = series_id.clone();
        async move {
            Extension
                .get_series_episodes(&series_id, Some(page))
                .await
                .unwrap_throw()
        }
    })
    .always_refetch()
}
