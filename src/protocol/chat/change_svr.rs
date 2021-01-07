/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoResponse};
use serde::{Deserialize, Serialize};

/// If received, Client must change server or client will get disconencted soon.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct ChangeSvr;
