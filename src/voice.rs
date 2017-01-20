/*
 * Copyright (c) 2017 Sebastian Jastrzebski <sebby2k@gmail.com>. All rights reserved.
 * Portions (c) 2004 Dag Lem <resid@nimrod.no>
 *
 * This file is part of resid-rs.
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
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::cell::RefCell;
use std::rc::Rc;

use super::ChipModel;
use super::envelope::EnvelopeGenerator;
use super::wave::WaveformGenerator;

// The waveform output range is 0x000 to 0xfff, so the "zero"
// level should ideally have been 0x800. In the measured chip, the
// waveform output "zero" level was found to be 0x380 (i.e. $d41b
// = 0x38) at 5.94V.
const WAVE_ZERO: i32 = 0x0380;

// The envelope multiplying D/A converter introduces another DC
// offset. This is isolated by the following measurements:
//
// * The "zero" output level of the mixer at full volume is 5.44V.
// * Routing one voice to the mixer at full volume yields
//     6.75V at maximum voice output (wave = 0xfff, sustain = 0xf)
//     5.94V at "zero" voice output  (wave = any,   sustain = 0x0)
//     5.70V at minimum voice output (wave = 0x000, sustain = 0xf)
// * The DC offset of one voice is (5.94V - 5.44V) = 0.50V
// * The dynamic range of one voice is |6.75V - 5.70V| = 1.05V
// * The DC offset is thus 0.50V/1.05V ~ 1/2 of the dynamic range.
//
// Note that by removing the DC offset, we get the following ranges for
// one voice:
//     y > 0: (6.75V - 5.44V) - 0.50V =  0.81V
//     y < 0: (5.70V - 5.44V) - 0.50V = -0.24V
// The scaling of the voice amplitude is not symmetric about y = 0;
// this follows from the DC level in the waveform output.
const VOICE_DC: i32 = 0x800 * 0xff;

pub struct Voice {
    // Configuration
    wave_zero: i32,
    voice_dc: i32,
    // Generators
    pub envelope: EnvelopeGenerator,
    pub wave: Rc<RefCell<WaveformGenerator>>,
}

impl Voice {
    pub fn new(chip_model: ChipModel) -> Voice {
        Voice {
            wave_zero: WAVE_ZERO,
            voice_dc: VOICE_DC,
            envelope: EnvelopeGenerator::new(),
            wave: Rc::new(RefCell::new(
                WaveformGenerator::new(chip_model)
            )),
        }
    }

    pub fn get_wave(&self) -> Rc<RefCell<WaveformGenerator>> {
        self.wave.clone()
    }

    pub fn set_control(&mut self, value: u8) {
        self.envelope.set_control(value);
        self.wave.borrow_mut().set_control(value);
    }

    pub fn set_sync_source(&mut self, source: &mut Voice) {
        self.wave.borrow_mut().set_sync_source(source.get_wave());
        let source_wave = source.get_wave();
        source_wave.borrow_mut().set_sync_dest(self.get_wave());
    }

    /*
    Amplitude modulated 20-bit waveform output.
    Range [-2048*255, 2047*255].
    */
    pub fn output(&self) -> i32 {
        // Multiply oscillator output with envelope output.
        (self.wave.borrow().output() as i32 - self.wave_zero) * self.envelope.output() as i32
        + self.voice_dc
    }

    pub fn reset(&mut self) {
        self.envelope.reset();
        self.wave.borrow_mut().reset();
    }
}
