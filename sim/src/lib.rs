// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate control;
#[macro_use]
extern crate derivative;
extern crate dimensioned;
extern crate ezgui;
extern crate geom;
extern crate graphics;
extern crate map_model;
extern crate multimap;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate vecmath;

pub mod common;
mod straw_intersections;
pub mod straw_model;

pub use common::CarID;
