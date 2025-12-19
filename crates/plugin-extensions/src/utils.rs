use crate::PluginState;

pub trait AsyncTryFromWithState<T>: Sized {
    async fn async_try_from_with_state(value: T, state: &PluginState) -> anyhow::Result<Self>;
}

pub trait AyncTryIntoWithState<T>: Sized {
    async fn async_try_into_with_state(self, state: &PluginState) -> anyhow::Result<T>;
}

impl<T, U> AyncTryIntoWithState<U> for T
where
    U: AsyncTryFromWithState<T>,
{
    async fn async_try_into_with_state(self, state: &PluginState) -> anyhow::Result<U> {
        U::async_try_from_with_state(self, state).await
    }
}
