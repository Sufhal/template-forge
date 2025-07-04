use std::{marker::PhantomData, path::PathBuf};

pub struct ForgeTemplate<S: ToString> {
    name: String,
    path: PathBuf,
    _phantom_s: PhantomData<S>,
}

impl<S: ToString> ForgeTemplate<S> {
    pub fn new(
        name: S,
        path: PathBuf,
    ) -> Self
    {
        Self {
            name: name.to_string(),
            path,
            _phantom_s: PhantomData,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

