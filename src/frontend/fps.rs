// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use std::collections::VecDeque;
use std::time::Instant;

const HISTORY_SIZE: usize = 128;

pub struct FpsCounter {
  history: VecDeque<f64>,
  last_time: Instant
}

impl FpsCounter {
  pub fn new() -> FpsCounter {
    FpsCounter {
      history: VecDeque::with_capacity(HISTORY_SIZE),
      last_time: Instant::now()
    }
  }
  pub fn update(&mut self, current_time: Instant) {
    let delta = current_time - self.last_time;
    let delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;

    self.make_room_for_new_element();
    self.history.push_front(delta_s);

    self.last_time = current_time;
  }
  pub fn get_fps(&self) -> f64 {
    let sum = self.history.iter().fold(0.0, |acc, &item| acc + item);
    self.history.len() as f64 / sum
  }
  fn make_room_for_new_element(&mut self) {
    if self.history.len() >= HISTORY_SIZE {
      let _ = self.history.pop_back();
    }
  }
}
