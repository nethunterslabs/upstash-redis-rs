use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::model::ReResponse;

use super::Result;

pub mod bitmap;
pub mod connection;
pub mod generic;
pub mod geo;
pub mod hash;
pub mod hyper_log_log;
pub mod json;
pub mod list;
pub mod pub_sub;
pub mod scripting;
pub mod server;
pub mod set;
pub mod sorted_set;
pub mod streams;
pub mod string;
pub mod transactions;

#[async_trait]
pub trait Command {
    type Output: DeserializeOwned;

    fn as_cmd(&self) -> &ReCmd;

    fn client(&self) -> &Client;

    fn url(&self) -> &Url;

    fn set_options<S: Serialize>(&mut self, opt: S) -> Result<&mut Self>;
    fn set_options_with_arg<S: Serialize>(&mut self, key: S, value: S) -> Result<&mut Self> {
        self.set_options(key)?.set_options(value)
    }

    async fn send(&self) -> Result<ReResponse<Self::Output>> {
        Ok(self
            .client()
            .post(self.url().as_ref())
            .json(self.as_cmd())
            .send()
            .await?
            .json()
            .await?)
    }

    fn json_to_output(&self, json: Value) -> Result<Self::Output> {
        Ok(serde_json::from_value(json)?)
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ReCmd {
    pub(crate) elems: Vec<Value>,
}

impl ReCmd {
    pub fn new<S: Serialize>(command: S) -> Self {
        let elems = vec![json!(command)];

        Self { elems }
    }

    pub fn add_arg<A: Serialize>(&mut self, arg: A) -> Result<&mut Self> {
        let v = serde_json::to_value(arg)?;
        self.elems.push(v);

        Ok(self)
    }

    pub fn add_pair<A: Serialize>(&mut self, k: A, v: A) -> Result<&mut Self> {
        let key = serde_json::to_value(k)?;
        let value = serde_json::to_value(v)?;

        self.elems.push(key);
        self.elems.push(value);

        Ok(self)
    }
}

#[macro_export]
macro_rules! cmd {
    ($(#[$outer:meta])* $c:ident,$ty:ty;$($i:ident),*) => {
        paste::paste! {
            $(#[$outer])*
            pub struct [<$c:camel Command>] {
                pub(crate) client: reqwest::Client,
                pub(crate) url: reqwest::Url,
                pub(crate) json: $crate::commands::ReCmd,
            }

            impl $crate::commands::Command for [<$c:camel Command>] {
                type Output = $ty;

                fn as_cmd(&self) -> &$crate::commands::ReCmd {
                    &self.json
                }

                fn set_options<S: serde::Serialize>(&mut self, opt: S) -> $crate::Result<&mut Self> {
                    self.json.add_arg(opt)?;
                    Ok(self)
                }

                fn client(&self) -> &reqwest::Client {
                    &self.client
                }

                fn url(&self) -> &reqwest::Url {
                    &self.url
                }
            }

            impl $crate::Redis {
                $(#[$outer])*
                pub fn [<$c:lower>]<$([<$i:upper>]),*>(&self, $($i: [<$i:upper>]),*) -> $crate::Result<[<$c:camel Command>]>
                where
                    $(
                    [<$i:upper>]: serde::Serialize,
                    )*
                {
                    [<$c:camel Command>]::new(self.client.clone(), self.url.clone(), $($i,)*)
                }
            }

            impl [<$c:camel Command>] {
                pub fn new<$([<$i:upper>]),*>(
                    client: reqwest::Client,
                    url: reqwest::Url,
                    $(
                    $i: [<$i:upper>],
                    )*
                ) -> $crate::Result<Self>
                where
                    $(
                    [<$i:upper>]: serde::Serialize,
                    )*
                {
                    let mut json = $crate::commands::ReCmd::new(stringify!($c));

                    $(
                    json.add_arg($i)?;
                    )*

                    Ok(Self { client, url, json })
                }
            }
        }
    };
}
