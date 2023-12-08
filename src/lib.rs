use reqwest::{header, Client, Url};

pub mod commands;
mod error;
mod model;

pub use commands::{Command, ReCmd};
pub use error::Result;
pub use model::ReResponse;

/// The main struct for interacting with the Upstash Redis REST API.
#[derive(Clone)]
pub struct Redis {
    client: Client,
    url: Url,
}

impl Redis {
    /// Create a new Upstash Redis REST API client.
    pub fn new<RS>(url: RS, token: RS) -> Result<Self>
    where
        RS: AsRef<str>,
    {
        let auth = format!("Bearer {}", token.as_ref());

        let mut value = header::HeaderValue::from_str(auth.as_str())?;
        value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.append(header::AUTHORIZATION, value);

        let client = Client::builder().default_headers(headers).build()?;

        let url = Url::parse(url.as_ref())?;

        Ok(Self { client, url })
    }

    /// Create a new pipeline builder.
    pub fn pipeline(&self) -> Pipeline {
        Pipeline::new(self)
    }

    /// Create a new transaction builder.
    pub fn transaction(&self) -> Transaction {
        Transaction::new(self)
    }
}

/// Send multiple commands in a single request through [pipelining](https://upstash.com/docs/redis/features/restapi#pipelining).
/// This is useful for reducing the network latency of sending multiple commands.
#[derive(Debug, Clone)]
pub struct Pipeline {
    client: Client,
    url: Url,
    cmds: Vec<ReCmd>,
}

impl Pipeline {
    /// Create a new pipeline builder.
    pub fn new(redis: &Redis) -> Self {
        let mut url = redis.url.clone();
        url.set_path("/pipeline");

        Self {
            client: redis.client.clone(),
            url,
            cmds: Vec::new(),
        }
    }

    /// Add a command to the pipeline.
    pub fn add_cmd(&mut self, cmd: ReCmd) -> &mut Self {
        self.cmds.push(cmd);
        self
    }

    /// Add multiple commands to the pipeline.
    pub fn add_cmds(&mut self, cmds: Vec<ReCmd>) -> &mut Self {
        self.cmds.extend(cmds);
        self
    }

    /// Send the pipelined commands.
    pub async fn send(&self) -> Result<Vec<ReResponse<serde_json::Value>>> {
        Ok(self
            .client
            .post(self.url.as_ref())
            .json(&self.cmds)
            .send()
            .await?
            .json()
            .await?)
    }
}

/// Redis transactions are atomic operations that are executed sequentially and uninterruptedly.
#[derive(Debug, Clone)]
pub struct Transaction {
    client: Client,
    url: Url,
    cmds: Vec<ReCmd>,
}

impl Transaction {
    /// Create a new transaction builder.
    pub fn new(redis: &Redis) -> Self {
        let mut url = redis.url.clone();
        url.set_path("/multi-exec");

        Self {
            client: redis.client.clone(),
            url,
            cmds: Vec::new(),
        }
    }

    /// Add a command to the transaction.
    pub fn add_cmd(&mut self, cmd: ReCmd) -> &mut Self {
        self.cmds.push(cmd);
        self
    }

    /// Add multiple commands to the transaction.
    pub fn add_cmds(&mut self, cmds: Vec<ReCmd>) -> &mut Self {
        self.cmds.extend(cmds);
        self
    }

    /// Send the transaction commands.
    pub async fn send(&self) -> Result<ReResponse<Vec<ReResponse<serde_json::Value>>>> {
        Ok(self
            .client
            .post(self.url.as_ref())
            .json(&self.cmds)
            .send()
            .await?
            .json()
            .await?)
    }
}
