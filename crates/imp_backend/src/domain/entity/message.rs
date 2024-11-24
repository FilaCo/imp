/*
 * IMP, the serverless peer-to-peer instant messaging protocol.
 * Copyright (C) 2024 FilaCo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::domain::entity::Peer;
use crate::domain::vo::message::*;
use chrono::{DateTime, Utc};
use imp_ddd::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Message {
    id: Id,
    text: String,
    author: Arc<Peer>,

    version: Version,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Message {}

#[derive(Debug, Copy, Clone)]
pub enum Event {}
