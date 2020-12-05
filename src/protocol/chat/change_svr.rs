/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use loco_derive::{LocoResponse, BsonData};

/// If received, Client must change server or client will get disconencted soon.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct ChangeSvr;