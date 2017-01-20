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

extern crate resid;

use resid::{ChipModel, Sid};

const CPU_FREQ: u32 = 985248;
const SAMPLE_FREQ: u32 = 44100;
const SAMPLE_COUNT: usize = 128;
const CYCLES_PER_SAMPLE: u32 = CPU_FREQ / SAMPLE_FREQ;

fn dump(sid: &mut Sid, name: &str, samples: usize) -> Vec<i16> {
    let mut buffer = vec![0; samples];
    let (read, delta) = sid.sample(samples as u32 * CYCLES_PER_SAMPLE,
                                   &mut buffer,
                                   samples,
                                   1);
    buffer
}

fn setup(sid: &mut Sid, voice: u8, waveform: u8, freq: u16, pw: u16, vol: u8) {
    let offset = voice * 7;
    let control = (waveform << 4) | 1;
    sid.write(offset + 0x00, (freq & 0x00ff) as u8); // FREQ_LO
    sid.write(offset + 0x01, (freq >> 8) as u8); // FREQ_HI
    sid.write(offset + 0x02, (pw & 0x00ff) as u8); // PW_LO
    sid.write(offset + 0x03, (pw >> 8) as u8); // PW_HI
    sid.write(offset + 0x04, control); // CONTROL
    sid.write(offset + 0x05, 9); // ATTACK_DECAY
    sid.write(offset + 0x06, 0); // SUSTAIN_RELEASE
    sid.write(0x18, vol); // MODE_VOL
}

#[test]
fn filter_off() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    let res = dump(&mut sid, "filter_off", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_on() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    let res = dump(&mut sid, "filter_on", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_vol_8() {
    let expected: Vec<i16> = vec![
        -16919, -16849, -16833, -16817, -16515, -16452, -16358, -16296, -16202, -16139, -16045, -15983, -15889, -15827, -14872, -14763, -14600, -14491, -14327, -14218, -14054, -13945, -13781, -13672, -13509, -13400, -13236, -13127, -12963, -12854, -12691, -12581, -12472, -12309, -12200, -12036, -11927, -11763, -11654, -11491, -11381, -11218, -11109, -10945, -10836, -10672, -10563, -10400, -10291, -10127, -12949, -9856, -9747, -9584, -9475, -9311, -9202, -9038, -8929, -8820, -8656, -8547, -8384, -8275, -8111, -8002, -7838, -7729, -7565, -7456, -11387, -11325, -11231, -11168, -6747, -6638, -6475, -6365, -6202, -6093, -5929, -5820, -5656, -5547, -5384, -5275, -5111, -5002, -4893, -4729, -4620, -4456, -4347, -4184, -4075, -3911, -3802, -3638, -3529, -3365, -3256, -8979, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -2984, -8917, -8917, -8917, -8917, -8917, -8917, -8917, -8917, -8917, -8917, -8917, -8917, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x18, 8); // MODE_VOL
    let res = dump(&mut sid, "filter_vol_8", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_voice3_off() {
    let expected: Vec<i16> = vec![
        -22668, -22683, -22693, -22704, -22580, -22567, -22547, -22534, -22514, -22501, -22481, -22468, -22449, -22436, -21986, -21949, -21895, -21859, -21804, -21768, -21713, -21677, -21622, -21586, -21531, -21495, -21440, -21404, -21349, -21313, -21259, -21222, -21186, -21131, -21095, -21040, -21004, -20949, -20913, -20859, -20822, -20768, -20731, -20677, -20640, -20586, -20549, -20495, -20459, -20404, -20368, -20314, -20277, -20223, -20187, -20132, -20096, -20041, -20005, -19968, -19914, -19877, -19823, -19787, -19732, -19696, -19641, -19605, -19550, -19514, -21507, -21493, -21474, -21461, -19277, -19241, -19187, -19150, -19096, -19059, -19005, -18968, -18914, -18877, -18823, -18787, -18732, -18696, -18659, -18605, -18568, -18514, -18477, -18423, -18387, -18332, -18296, -18241, -18205, -18150, -18114, -18059, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -18023, -20989, -20989, -20989, -20989, -20989, -20989, -20989, -20989, -20989, -20989, -20989, -20989, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x18, (0x08 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_voice3_off", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_hp_lp_lp() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x18, (0x07 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_hp_lp_lp", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_filt_15() {
    let expected: Vec<i16> = vec![
        -29856, -29561, -29267, -28987, -28842, -28599, -28381, -28146, -27950, -27748, -27560, -27377, -27217, -27042, -27311, -27155, -27024, -26884, -26777, -26643, -26551, -26431, -26353, -26255, -26183, -26099, -26047, -25969, -25930, -25869, -25835, -25784, -25741, -25726, -25693, -25692, -25665, -25674, -25659, -25673, -25668, -25694, -25694, -25728, -25739, -25779, -25796, -25845, -25869, -25924, -24557, -26116, -26150, -26212, -26250, -26319, -26363, -26436, -26484, -26536, -26616, -26673, -26758, -26818, -26907, -26971, -27063, -27129, -27225, -27295, -25440, -25640, -25846, -26033, -28295, -28346, -28425, -28479, -28562, -28619, -28704, -28764, -28852, -28915, -29004, -29068, -29160, -29226, -29293, -29389, -29458, -29554, -29625, -29723, -29793, -29892, -29965, -30065, -30139, -30240, -30313, -27617, -30705, -30719, -30739, -30763, -30792, -30826, -30864, -30907, -30952, -31001, -31056, -31112, -28342, -28625, -28888, -29144, -29407, -29651, -29888, -30130, -30355, -30574, -30796, -31003, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x17, 0x0f); // RESFILT
    sid.write(0x18, (0x07 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_filt_15", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_res_15() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x17, 0xf0); // RESFILT
    sid.write(0x18, (0x07 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_res_15", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_fc_255() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x15, 0xff); // FCLO
    sid.write(0x18, (0x07 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_fc_255", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn filter_fc_res_filt() {
    let expected: Vec<i16> = vec![
        -29925, -29739, -29539, -29342, -29279, -29106, -28952, -28775, -28627, -28465, -28315, -28161, -28027, -27872, -28161, -28025, -27909, -27779, -27679, -27547, -27452, -27327, -27241, -27131, -27045, -26944, -26873, -26773, -26711, -26625, -26564, -26488, -26416, -26370, -26307, -26276, -26216, -26194, -26149, -26132, -26096, -26092, -26061, -26065, -26048, -26058, -26050, -26073, -26071, -26103, -24691, -26219, -26235, -26284, -26308, -26364, -26397, -26461, -26503, -26549, -26625, -26679, -26764, -26826, -26918, -26988, -27087, -27163, -27269, -27352, -25479, -25648, -25828, -25996, -28280, -28371, -28492, -28590, -28716, -28817, -28949, -29054, -29188, -29299, -29437, -29549, -29693, -29809, -29926, -30075, -30195, -30343, -30468, -30619, -30742, -30898, -31023, -31176, -31307, -31461, -31589, -28901, -32022, -32096, -32176, -32256, -32343, -32429, -32518, -32614, -32708, -32768, -32768, -32768, -30234, -30490, -30733, -30975, -31226, -31465, -31702, -31948, -32181, -32411, -32650, -32768, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(true);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    sid.write(0x15, 0xff); // FCLO
    sid.write(0x17, 0xff); // RESFILT
    sid.write(0x18, (0x07 << 4) | 0x04); // MODE_VOL
    let res = dump(&mut sid, "filter_fc_res_filt", SAMPLE_COUNT);
    assert_eq!(res, expected);
}
