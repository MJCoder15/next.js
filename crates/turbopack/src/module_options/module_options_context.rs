#[turbo_tasks::value(shared)]
#[derive(Default)]
pub struct ModuleOptionsContext {
    pub enable_react_refresh: bool,
    pub enable_styled_jsx: bool,
    pub enable_typescript_transform: bool,
    pub placeholder_for_future_extensions: (),
}

#[turbo_tasks::value_impl]
impl ModuleOptionsContextVc {
    #[turbo_tasks::function]
    pub fn default() -> Self {
        Self::cell(Default::default())
    }
}

impl Default for ModuleOptionsContextVc {
    fn default() -> Self {
        Self::default()
    }
}
