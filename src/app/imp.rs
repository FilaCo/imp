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
use cfg_if::cfg_if;
use iced::*;
use imp_message::*;

#[derive(Debug, Copy, Clone)]
pub enum Imp {
    Loading,
    Loaded(State),
}

impl Imp {
    pub fn new() -> (Self, Task<Message>) {
        todo!()
    }

    pub fn update(&mut self, msg: Message) {
        match msg {}
    }

    cfg_if! {
        if #[cfg(feature = "gui")] {
            pub fn view(&self) -> Element<Message> {
                todo!()
            }

            pub fn title(&self) -> String {
                todo!()
            }
        } else if #[cfg(feature = "tui")] {
            pub fn view(&self) {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct State {}
