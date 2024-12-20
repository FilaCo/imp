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
use std::num::NonZeroU128;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(NonZeroU128);

impl Version {
    pub fn new() -> Self {
        Self(NonZeroU128::MIN)
    }

    pub fn increment(&mut self) -> Option<&mut Self> {
        self.0 = self.0.checked_add(1)?;

        Some(self)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl From<NonZeroU128> for Version {
    fn from(value: NonZeroU128) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn increment_works() {
        // arrange
        let test_suits = [
            (
                NonZeroU128::new(5),
                Some(Version::from(NonZeroU128::new(6).unwrap())),
            ),
            (Some(NonZeroU128::MAX), None),
            (None, Some(Version::from(NonZeroU128::new(2).unwrap()))),
        ];

        // act
        let result = test_suits.map(|suit| {
            let mut sut = if let Some(n) = suit.0 {
                n.into()
            } else {
                Version::default()
            };

            sut.increment().map(|v| *v)
        });

        // assert
        for i in 0..result.len() {
            assert_eq!(test_suits[i].1, result[i]);
        }
    }
}
