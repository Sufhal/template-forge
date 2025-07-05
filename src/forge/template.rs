use std::{marker::PhantomData, path::PathBuf};
use serde::Serialize;

pub struct ForgeTemplate<S: ToString, T: Serialize> {
    name: String,
    path: PathBuf,
    default_data: T,
    _phantom_s: PhantomData<S>,
}

impl<S: ToString, T: Serialize> ForgeTemplate<S, T> {
    pub fn new(
        name: S,
        path: PathBuf,
        default_data: T,
    ) -> Self
    {
        Self {
            name: name.to_string(),
            path,
            default_data,
            _phantom_s: PhantomData,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_default_data(&self) -> &T { &self.default_data }
}

