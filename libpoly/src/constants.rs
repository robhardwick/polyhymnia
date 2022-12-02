use core::ops::RangeInclusive;

use crate::synth::Signal;

pub const SEQUENCE_MUTATE: RangeInclusive<usize> = 441_000..=882_000;
pub const SYNTH_MUTATE: RangeInclusive<usize> = 2_205_000..=4_410_000;

pub const TEMPOS: RangeInclusive<f32> = 80.0..=120.0;
pub const METRES: [usize; 5] = [3, 4, 5, 7, 8];
pub const LENGTH: usize = 8;

pub const OPERATORS: [(Signal, RangeInclusive<f32>); 5] = [
    (Signal::Square, 1.0..=1.0),
    (Signal::Saw, 1.0..=1.0),
    (Signal::Square, 2.0..=4.0),
    (Signal::Sine, 0.2..=0.4),
    (Signal::Square, 0.2..=0.4),
];

pub const ATTACK: RangeInclusive<f32> = 0.01..=0.6;
pub const DECAY: RangeInclusive<f32> = 0.01..=0.1;
pub const SUSTAIN: RangeInclusive<f32> = 0.4..=0.95;
pub const RELEASE: RangeInclusive<f32> = 0.01..=0.05;

pub const CUTOFF: RangeInclusive<f32> = 400.0..=600.0;
pub const Q: RangeInclusive<f32> = 0.2..=0.5;

pub type Scale = [f32; 8];
pub const SCALES: [Scale; 21] = [
    [
        739.9800001503532,
        678.3149999421656,
        616.6500001254908,
        924.9750002601583,
        462.4875001300791,
        847.8937499939062,
        1017.4724999923628,
        508.736249996181,
    ],
    [
        832.4774998177487,
        739.9800001503532,
        678.3149999421656,
        554.9850001559179,
        1109.970000311835,
        1017.4724999923628,
        763.1043750536072,
        508.736249996181,
    ],
    [
        739.9800001503532,
        485.61187500483805,
        647.482499956106,
        616.6500001254908,
        554.9850001559179,
        462.4875001300791,
        693.731249848345,
        539.5687499635937,
    ],
    [
        739.9800001503532,
        647.482499956106,
        616.6500001254908,
        462.4875001300791,
        501.02812487931885,
        601.2337498549912,
        539.5687499635937,
        701.4393749913753,
    ],
    [
        832.4774998177487,
        801.6450002073708,
        450.92531242630474,
        739.9800001503532,
        901.8506248526095,
        554.9850001559179,
        1109.970000311835,
        601.2337498549912,
    ],
    [
        801.6450002073708,
        739.9800001503532,
        616.6500001254908,
        1002.0562497586384,
        462.4875001300791,
        501.02812487931885,
        601.2337498549912,
        924.9750002601583,
    ],
    [
        739.9800001503532,
        581.4128572960421,
        654.0894641819485,
        687.1242857347563,
        472.39794634311426,
        601.2337498549912,
        475.70142859618926,
        508.736249996181,
    ],
    [
        450.92531242630474,
        739.9800001503532,
        616.6500001254908,
        554.9850001559179,
        462.4875001300791,
        501.02812487931885,
        693.731249848345,
        601.2337498549912,
    ],
    [
        801.6450002073708,
        739.9800001503532,
        647.482499956106,
        1294.9649999122114,
        1202.4674997099817,
        601.2337498549912,
        701.4393749913753,
        863.3099998743477,
    ],
    [
        739.9800001503532,
        1187.0512502627944,
        678.3149999421656,
        647.482499956106,
        593.5256251313969,
        1017.4724999923628,
        508.736249996181,
        863.3099998743477,
    ],
    [
        739.9800001503532,
        678.3149999421656,
        616.6500001254908,
        554.9850001559179,
        462.4875001300791,
        693.731249848345,
        763.1043750536072,
        508.736249996181,
    ],
    [
        801.6450002073708,
        739.9800001503532,
        678.3149999421656,
        551.1309375264486,
        1102.2618750528968,
        601.2337498549912,
        508.736249996181,
        1017.4724999923628,
    ],
    [
        739.9800001503532,
        517.9859999244433,
        582.7342500056201,
        647.482499956106,
        712.230750157449,
        457.86262503201795,
        665.9820001868889,
        508.736249996181,
    ],
    [
        739.9800001503532,
        647.482499956106,
        616.6500001254908,
        462.4875001300791,
        1079.137499927188,
        539.5687499635937,
        924.9750002601583,
        863.3099998743477,
    ],
    [
        832.4774998177487,
        739.9800001503532,
        616.6500001254908,
        554.9850001559179,
        462.4875001300791,
        693.731249848345,
        1109.970000311835,
        924.9750002601583,
    ],
    [
        739.9800001503532,
        485.61187500483805,
        678.3149999421656,
        647.482499956106,
        554.9850001559179,
        593.5256251313969,
        763.1043750536072,
        508.736249996181,
    ],
    [
        450.92531242630474,
        739.9800001503532,
        678.3149999421656,
        551.1309375264486,
        554.9850001559179,
        601.2337498549912,
        763.1043750536072,
        508.736249996181,
    ],
    [
        832.4774998177487,
        739.9800001503532,
        485.61187500483805,
        647.482499956106,
        554.9850001559179,
        971.2237500096761,
        1109.970000311835,
        863.3099998743477,
    ],
    [
        739.9800001503532,
        485.61187500483805,
        647.482499956106,
        616.6500001254908,
        554.9850001559179,
        462.4875001300791,
        693.731249848345,
        539.5687499635937,
    ],
    [
        739.9800001503532,
        678.3149999421656,
        551.1309375264486,
        616.6500001254908,
        462.4875001300791,
        501.02812487931885,
        601.2337498549912,
        508.736249996181,
    ],
    [
        549.4136869822032,
        457.84473915198214,
        523.2511306011972,
        575.5762434951098,
        470.9260175775441,
        503.629213212657,
        539.6027283607516,
        479.64686957941154,
    ],
];