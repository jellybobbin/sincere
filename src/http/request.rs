use std::collections::HashMap;
use std::net::SocketAddr;

use serde::de::DeserializeOwned;
use serde_json;

use super::http_method::Method;
use error::Result;

pub struct Request {
    method: Method,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    remote_addr: SocketAddr,
    data: Vec<u8>,
}

impl Request {
    pub fn new(method: Method, path: String, version: String, headers: HashMap<String, String>, remote_addr: SocketAddr, data: Vec<u8>) -> Request {
        Request {
            method: method,
            path: path,
            version: version,
            headers: headers,
            params: HashMap::new(),
            remote_addr: remote_addr,
            data: data,
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn headers(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn get_header<'a, S>(&self, key: S) -> Option<String>
        where S: Into<&'a str>
    {
        self.headers.get(key.into()).map(|v| v.to_string())
    }

    pub fn remote_addr(&self) -> &SocketAddr {
        &self.remote_addr
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    pub fn data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn params(&mut self) -> &mut HashMap<String, String> {
        &mut self.params
    }

    pub fn get_param<'a, S>(&self, key: S) -> Option<String>
        where S: Into<&'a str>
    {
        self.params.get(key.into()).map(|v| v.to_string())
    }

    pub fn bind_json<D: DeserializeOwned>(&self) -> Result<D> {
        Ok(serde_json::from_slice(&self.data)?)
    }
}
