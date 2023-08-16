// MIT License
// Copyright (c) 2023 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;
use self::records::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

pub struct KeyPropertiesRepository {
    properties_repository: PropertiesRespository,
    properties_index: BTreeIndex,
}

impl KeyPropertiesRepository {
    pub fn new(init_ctx: init::InitContext) -> Self {
        KeyPropertiesRepository { 
            properties_repository: PropertiesRespository::new(&init_ctx.get_key_properties_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap()),
            properties_index: BTreeIndex::new(&init_ctx.get_key_properties_index_path().unwrap())
        }
    }

    pub fn put(&mut self, key: &str, properties: &mut [Property]) -> Option<()> {
        let props_id = self.properties_repository.create_list(properties)?;
        self.properties_index.insert(key, props_id)
    }

    pub fn get(&mut self, key: &str) -> Option<Vec<Vec<Property>>> {
        let props_ids = self.properties_index.search(key)?;
        let mut res = vec![];
        for pid in props_ids {
            let props = self.properties_repository.retrieve_list(pid)?;
            res.push(props);
        }
        Some(res)
    }
    pub fn sync(&mut self) {
        self.properties_repository.sync();
        self.properties_index.sync();
        
    }
    pub fn clear(&mut self) {
        self.properties_repository.clear();
        self.properties_index.clear();
    }
}