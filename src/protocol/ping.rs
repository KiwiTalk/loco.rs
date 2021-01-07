/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoRequest};
use serde::{Deserialize, Serialize};

/// Signal server to keep connection
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoRequest)]
pub struct Ping;
