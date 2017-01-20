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
fn waveform_0() {
    let expected: Vec<i16> = vec![
        4111, 4271, 4264, 4254, 4245, 4236, 4226, 4217, 4208, 4198, 4189, 4180, 4171,
        4162, 4152, 4143, 4134, 4125, 4116, 4107, 4098, 4089, 4079, 4071, 4062, 4053,
        4044, 4035, 4026, 4017, 4009, 4000, 3991, 3982, 3973, 3965, 3956, 3947, 3939,
        3930, 3921, 3913, 3904, 3895, 3887, 3878, 3870, 3861, 3853, 3844, 3836, 3828,
        3819, 3811, 3802, 3794, 3785, 3777, 3769, 3760, 3752, 3744, 3736, 3728, 3719,
        3711, 3703, 3695, 3687, 3679, 3671, 3663, 3654, 3646, 3638, 3630, 3622, 3614,
        3606, 3599, 3591, 3583, 3575, 3567, 3559, 3551, 3544, 3536, 3528, 3520, 3512,
        3505, 3497, 3489, 3482, 3474, 3466, 3459, 3451, 3444, 3436, 3429, 3421, 3413,
        3406, 3398, 3391, 3383, 3376, 3369, 3361, 3354, 3347, 3339, 3332, 3325, 3317,
        3310, 3303, 3295, 3288, 3281, 3274, 3267, 3260, 3252, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.reset();
    sid.write(0x18, 4);
    let res = dump(&mut sid, "waveform_0", SAMPLE_COUNT);
}

#[test]
fn waveform_1() {
    let expected: Vec<i16> = vec![
        -21179, -21185, -21188, -21190, -21193, -21193, -21193, -21192, -21190, -21187, -21183, -21177, -21171, -21164, -21156, -21147, -21136, -21125, -21112, -21099, -21084, -21069, -21052, -21036, -21016, -20998, -20976, -20956, -20932, -20911, -20884, -20860, -20836, -20806, -20780, -20748, -20720, -20686, -20656, -20619, -20588, -20549, -20515, -20475, -20439, -20396, -20359, -20314, -20274, -20227, -20186, -20135, -20093, -20041, -19996, -19942, -19896, -19893, -19904, -19916, -19923, -19936, -19944, -19961, -19970, -19989, -20001, -20021, -20036, -20057, -20073, -20098, -20116, -20141, -20164, -20191, -20213, -20243, -20268, -20299, -20328, -20360, -20389, -20425, -20457, -20493, -20529, -20566, -20605, -20644, -20684, -20724, -20768, -20809, -20853, -20899, -20945, -20991, -21041, -21089, -21138, -21191, -21243, -21294, -21348, -21398, -21452, -21503, -21554, -21608, -21659, -21710, -21763, -21815, -21781, -21728, -21677, -21626, -21572, -21521, -21470, -21416, -21365, -21314, -21261, -21209, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 1, 0x19b1, 0x0200, 4);
    let res = dump(&mut sid, "waveform_1", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_2() {
    let expected: Vec<i16> = vec![
        -21179, -21186, -21190, -21194, -21199, -21201, -21205, -21207, -21210, -21211, -21213, -21213, -21213, -21212, -21212, -21210, -21208, -21206, -21203, -21199, -21196, -21191, -21186, -21180, -21174, -21168, -21161, -21153, -21145, -21137, -21127, -21118, -21108, -21097, -21087, -21075, -21063, -21050, -21037, -21023, -21010, -20994, -20980, -20963, -20948, -20930, -20914, -20896, -20878, -20859, -20841, -20819, -20800, -20779, -20758, -20735, -20715, -20689, -20668, -20647, -20620, -20597, -20570, -20546, -20518, -20493, -20464, -20438, -20407, -20381, -20349, -20321, -20289, -20260, -20226, -20197, -20163, -20132, -20095, -20065, -20027, -19995, -19957, -19923, -19884, -19851, -19809, -19775, -19739, -19696, -19661, -19618, -19580, -19536, -19498, -19452, -19413, -19367, -19327, -19279, -19238, -19188, -19147, -19121, -19095, -19069, -19042, -19017, -18992, -18965, -18939, -18913, -18887, -18861, -21803, -21776, -21750, -21725, -21698, -21673, -21647, -21620, -21595, -21569, -21543, -21516, 0, 0    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 2, 0x19b1, 0x0200, 4);
    let res = dump(&mut sid, "waveform_2", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_3() {
    let expected: Vec<i16> = vec![
        -21179, -21187, -21192, -21197, -21205, -21210, -21217, -21223, -21230, -21235, -21243, -21248, -21256, -21261, -21268, -21273, -21281, -21286, -21294, -21299, -21307, -21312, -21319, -21324, -21332, -21337, -21345, -21350, -21357, -21363, -21370, -21375, -21380, -21388, -21393, -21401, -21406, -21413, -21419, -21426, -21431, -21439, -21444, -21452, -21457, -21464, -21469, -21477, -21482, -21490, -21495, -21503, -21508, -21515, -21520, -21528, -20719, -21541, -21546, -21551, -21559, -21564, -21571, -21576, -21584, -21589, -21597, -21602, -21609, -21615, -21622, -21627, -21635, -21640, -21648, -21653, -21660, -21665, -21673, -21678, -21686, -21691, -21699, -21704, -21711, -21716, -21724, -21729, -21734, -21742, -21747, -21755, -21760, -21767, -21772, -21780, -21785, -21793, -21798, -21805, -21811, -21818, -21823, -21823, -21823, -21823, -21742, -21823, -21823, -21823, -21823, -21823, -21823, -21105, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 3, 0x19b1, 0x0200, 4);
    let res = dump(&mut sid, "waveform_3", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_4() {
    let expected: Vec<i16> = vec![
        -21179, -21187, -21192, -21197, -21205, -21210, -21217, -21223, -21230, -21235, -21243, -21248, -21256, -21261, -20838, -20820, -20793, -20775, -20747, -20729, -20702, -20684, -20656, -20638, -20611, -20593, -20565, -20547, -20520, -20502, -20475, -20456, -20438, -20411, -20393, -20365, -20347, -20320, -20302, -20275, -20256, -20229, -20211, -20184, -20165, -20138, -20120, -20093, -20075, -20047, -20029, -20002, -19984, -19957, -19939, -19911, -19893, -19866, -19848, -19829, -19802, -19784, -19757, -19739, -19711, -19693, -19666, -19648, -19620, -19602, -19575, -19557, -19529, -19511, -19484, -19466, -19439, -19420, -19393, -19375, -19348, -19329, -19302, -19284, -19257, -19239, -19211, -19193, -19175, -19148, -19129, -19102, -19084, -19057, -19039, -19011, -18993, -18966, -18948, -18920, -18902, -18875, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -18857, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, -21823, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    let res = dump(&mut sid, "waveform_4", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_8() {
    let expected: Vec<i16> = vec![
        -21156, -21129, -21111, -21093, -21066, -21048, -21021, -21003, -20976, -20958, -20934, -20916, -20889, -20871, -20845, -20827, -20800, -20782, -20756, -20738, -20711, -20693, -20667, -20649, -20622, -20604, -20577, -20560, -20533, -20515, -20488, -20471, -20467, -20441, -20424, -20397, -20380, -20354, -20336, -20310, -20293, -20267, -20249, -20223, -20205, -20179, -20162, -20136, -20118, -20092, -20075, -20048, -20031, -20005, -19987, -19961, -19944, -19917, -19900, -19883, -19911, -19895, -19869, -19853, -19828, -19811, -19786, -19769, -19744, -19727, -19702, -19685, -19660, -19644, -19754, -19739, -19716, -19700, -19677, -19662, -19639, -19624, -19601, -19586, -19563, -19548, -19525, -19509, -19494, -19471, -19456, -19433, -19418, -19395, -19380, -19357, -19691, -19672, -19660, -19641, -19629, -19610, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -19598, -20328, -20328, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 8, 0x19b1, 0x0200, 4);
    let res = dump(&mut sid, "waveform_8", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_440() {
    let expected: Vec<i16> = vec![
        -21184, -21200, -21210, -21220, -21096, -21083, -21063, -21050, -21031, -21017, -20998, -20985, -20965, -20952, -20502, -20466, -20411, -20375, -20320, -20284, -20229, -20193, -20139, -20102, -20048, -20011, -19957, -19920, -19866, -19829, -19775, -19739, -19702, -19648, -19611, -19557, -19520, -19466, -19429, -19375, -19339, -19284, -19248, -19193, -19157, -19102, -19066, -19011, -18975, -18920, -18884, -18830, -18794, -18739, -18703, -18648, -18612, -18557, -18521, -18485, -18430, -18394, -18339, -18303, -18248, -18212, -18157, -18121, -18067, -18030, -20023, -20010, -19990, -19977, -17794, -17757, -17703, -17667, -17612, -17576, -17521, -17485, -17430, -17394, -17339, -17303, -17248, -17212, -17176, -17121, -17085, -17030, -16994, -16939, -16903, -16848, -16812, -16757, -16721, -16667, -16630, -16576, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    let res = dump(&mut sid, "waveform_440", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_404() {
    let expected: Vec<i16> = vec![
        -21184, -21142, -21129, -21116, -21096, -21083, -21063, -21050, -21031, -21017, -20998, -20985, -20965, -20952, -20502, -20466, -20411, -20375, -20320, -20284, -20229, -20193, -20139, -20102, -20048, -20011, -19957, -19920, -19866, -19829, -19775, -19739, -19702, -19648, -19611, -19557, -19520, -19466, -19429, -19375, -19339, -19284, -19248, -19193, -19157, -19102, -19066, -19011, -18975, -18920, -20350, -18830, -18794, -18739, -18703, -18648, -18612, -18557, -18521, -18485, -18430, -18394, -18339, -18303, -18248, -18212, -18157, -18121, -18067, -18030, -17976, -17939, -17885, -17848, -17794, -17757, -17703, -17667, -17612, -17576, -17521, -17485, -17430, -17394, -17339, -17303, -17248, -17212, -17176, -17121, -17085, -17030, -16994, -16939, -16903, -16848, -16812, -16757, -16721, -16667, -16630, -19519, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -16539, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, -19506, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    let res = dump(&mut sid, "waveform_404", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_444() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    let res = dump(&mut sid, "waveform_444", SAMPLE_COUNT);
    assert_eq!(res, expected);
}

#[test]
fn waveform_444_2048() {
    let expected: Vec<i16> = vec![
        -21189, -21155, -21147, -21139, -20987, -20956, -20909, -20878, -20831, -20800, -20753, -20721, -20675, -20643, -20166, -20112, -20030, -19975, -19893, -19839, -19757, -19703, -19621, -19566, -19484, -19430, -19348, -19293, -19212, -19157, -19075, -19021, -18966, -18884, -18830, -18748, -18693, -18612, -18557, -18475, -18421, -18339, -18284, -18203, -18148, -18066, -18012, -17930, -17875, -17793, -19205, -17658, -17604, -17522, -17467, -17385, -17331, -17249, -17195, -17140, -17058, -17004, -16922, -16867, -16785, -16731, -16649, -16595, -16513, -16458, -18424, -18392, -18345, -18314, -16104, -16049, -15967, -15913, -15831, -15776, -15695, -15640, -15558, -15504, -15422, -15367, -15285, -15231, -15176, -15095, -15040, -14958, -14904, -14822, -14767, -14685, -14631, -14549, -14495, -14413, -14358, -17220, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -17188, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -14222, -17188, -17188, -17188, -17188, -17188, -14222, -14249, -14249, -14249, -14249, -14249, -14249, -17204, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14249, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -17220, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -17220, -17220, -17220, -17220, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -14276, -17220, -17220, -17220, -17220, -17220, -17235, -17235, -17235, -17235, -17235, -17235, -17235, -17235, -17235, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -17235, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14304, -14331, -14331, -14331, -14331, -17251, -17251, -17251, -17251, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -17251, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14331, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -17267, -17267, -17267, -17267, -17267, -17267, -17267, -17267, -17267, -20175, -20175, -20175, -20175, -23083, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14358, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -14385, -17282, -14385, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -17298, -17298, -17298, -17298, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14413, -14440, -14440, -14440, -14440, -17313, -20187, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -17313, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -14440, -17313, -17313, -17313, -17313, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -17329, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14467, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -14495, -17345, -14495, -14495, -17345, -17345, -17345, -17345, -17345, -14495, -14495, -14495, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17345, -17360, -17360, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -17360, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14522, -14549, -14549, -14549, -14549, -14549, -14549, -17376, -17376, -17376, -17376, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -17376, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14549, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -14576, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -17392, -14576, -14576, -14576, -14576, -17392, -17392, -17392, -17392, -14576, -14576, -14576, -14576, -17392, -14576, -14576, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14604, -14631, -14631, -14631, -14631, -17423, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -17423, -17423, -17423, -17423, -17423, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -14631, -17423, -17423, -17423, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -17439, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -14658, -17439, -17439, -17454, -17454, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -17454, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14685, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -14713, -17470, -20227, -22984, -20227, -20227, -20227, -17470, -17470, -17470, -17470, -17470, -17470, -17470, -17470, -14713, -14713, -14713, -14713, -14713, -14713, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -17485, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14740, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -17501, -17501, -17501, -17501, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -14767, -17501, -14767, -14767, -14767, -14767, -14795, -14795, -14795, -14795, -14795, -14795, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -17517, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -14795, -17517, -17517, -17517, -17517, -14795, -14795, -14795, -14795, -14795, -14822, -14822, -14822, -17532, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14822, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -17548, -14849, -14849, -14849, -14849, -14849, -14849, -17548, -17548, -17548, -17548, -17548, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -14849, -17548, -17548, -17548, -17548, -17548, -17548, -17548, -17548, -17548, -17548, -17548, -17564, -17564, -17564, -17564, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -17564, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14876, -14904, -17579, -17579, -17579, -17579, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -17579, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14904, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -17595, -17595, -17595, -17595, -17595, -17595, -17595, -17595, -17595, -17595, -17595, -20259, -20259, -20259, -20259, -14931, -17595, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14931, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -14958, -17611, -14958, -14958, -14958, -14958, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -17626, -17626, -17626, -17626, -17626, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -14985, -17642, -17642, -20271, -17642, -17642, -17642, -17642, -17642, -17642, -17642, -17642, -17642, -17642, -17642, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -15013, -17642, -17642, -17642, -17642, -15013, -15013, -15013, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -17657, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15040, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -15067, -17673, -15067, -15067, -17673, -17673, -17673, -17673, -15067, -15067, -15067, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -17673, -15067, -15067, -15067, -15067, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -17689, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15095, -15122, -15122, -17704, -17704, -17704, -17704, -17704, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -17704, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15122, -15149, -15149, -15149, -15149, -15149, -15149, -15149, -15149, -15149, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -17720, -15149, -15149, -15149, -15149, -15149, -15149, -17720, -17720, -17720, -17720, -15149, -15149, -15149, -15149, -15149, -17720, -15149, -15149, -15149, -15149, -15149, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -15176, -17751, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -17751, -17751, -17751, -17751, -17751, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -15204, -17751, -17751, -17751, -17751, -17751, -17751, -17751, -17751, -17767, -17767, -17767, -17767, -17767, -17767, -15231, -15231, -17767, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -15231, -17767, -17767, -17767, -17767, -15231, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -17783, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15258, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -17798, -17798, -17798, -20311, -22824, -20311, -20311, -17798, -17798, -17798, -17798, -17798, -17798, -17798, -17798, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15285, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -17814, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15313, -15340, -15340, -15340, -15340, -17829, -17829, -17829, -17829, -17829, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -15340, -17829, -15340, -15340, -15340, -15340, -15340, -15340, -15367, -15367, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -17845, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -17845, -17845, -17845, -17845, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -15367, -17861, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, -15395, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    let mut sid = Sid::new(ChipModel::Mos6581);
    sid.enable_external_filter(false);
    sid.enable_filter(false);
    setup(&mut sid, 0, 4, 0x19b1, 0x0200, 4);
    setup(&mut sid, 1, 4, 0x29b1, 0x0100, 4);
    setup(&mut sid, 2, 4, 0x39b1, 0x0050, 4);
    let res = dump(&mut sid, "waveform_444_2048", 2048);
    assert_eq!(res, expected);
}