// This file is part of resid-rs.
// Copyright (c) 2017-2019 Sebastian Jastrzebski <sebby2k@gmail.com>. All rights reserved.
// Portions (c) 2004 Dag Lem <resid@nimrod.no>
// Licensed under the GPLv3. See LICENSE file in the project root for full license text.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate core;

mod data;
pub mod envelope;
pub mod external_filter;
pub mod filter;
#[cfg(not(feature = "std"))]
mod math;
pub mod sampler;
mod sid;
pub mod spline;
pub mod synth;
pub mod voice;
pub mod wave;

#[derive(Clone, Copy)]
pub enum ChipModel {
    Mos6581,
    Mos8580,
}

pub use self::sampler::SamplingMethod;
pub use self::sid::Sid;
