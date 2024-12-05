/*
 * IMP, the serverless peer-to-peer instant messaging protocol.
 * Copyright (C) 2024  FilaCo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{Message, Model};
use iced::{Element, Task};

#[derive(Debug)]
pub struct Imp {
    model: Model,
}

impl Imp {
    pub fn new() -> (Self, Task<Message>) {
        todo!()
    }

    pub fn title(&self) -> String {
        "Imp".to_string()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        self.model
            .update(msg)
            .map(Task::done)
            .unwrap_or(Task::none())
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}
