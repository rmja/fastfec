use super::qpp::QppInterleaver;

pub const fn create(length: usize) -> Option<QppInterleaver> {
    match length {
        // 8 bit step
        40 => Some(QppInterleaver::new(40, 3, 10)),
        48 => Some(QppInterleaver::new(48, 7, 12)),
        56 => Some(QppInterleaver::new(56, 19, 42)),
        64 => Some(QppInterleaver::new(64, 7, 16)),
        72 => Some(QppInterleaver::new(72, 7, 18)),
        80 => Some(QppInterleaver::new(80, 11, 20)),
        88 => Some(QppInterleaver::new(88, 5, 22)),
        96 => Some(QppInterleaver::new(96, 11, 24)),
        104 => Some(QppInterleaver::new(104, 7, 26)),
        112 => Some(QppInterleaver::new(112, 41, 84)),
        120 => Some(QppInterleaver::new(120, 103, 90)),
        128 => Some(QppInterleaver::new(128, 15, 32)),
        136 => Some(QppInterleaver::new(136, 9, 34)),
        144 => Some(QppInterleaver::new(144, 17, 108)),
        152 => Some(QppInterleaver::new(152, 9, 38)),
        160 => Some(QppInterleaver::new(160, 21, 120)),
        168 => Some(QppInterleaver::new(168, 101, 84)),
        176 => Some(QppInterleaver::new(176, 21, 44)),
        184 => Some(QppInterleaver::new(184, 57, 46)),
        192 => Some(QppInterleaver::new(192, 23, 48)),
        200 => Some(QppInterleaver::new(200, 13, 50)),
        208 => Some(QppInterleaver::new(208, 27, 52)),
        216 => Some(QppInterleaver::new(216, 11, 36)),
        224 => Some(QppInterleaver::new(224, 27, 56)),
        232 => Some(QppInterleaver::new(232, 85, 58)),
        240 => Some(QppInterleaver::new(240, 29, 60)),
        248 => Some(QppInterleaver::new(248, 33, 62)),
        256 => Some(QppInterleaver::new(256, 15, 32)),
        264 => Some(QppInterleaver::new(264, 17, 198)),
        272 => Some(QppInterleaver::new(272, 33, 68)),
        280 => Some(QppInterleaver::new(280, 103, 210)),
        288 => Some(QppInterleaver::new(288, 19, 36)),
        296 => Some(QppInterleaver::new(296, 19, 74)),
        304 => Some(QppInterleaver::new(304, 37, 76)),
        312 => Some(QppInterleaver::new(312, 19, 78)),
        320 => Some(QppInterleaver::new(320, 21, 120)),
        328 => Some(QppInterleaver::new(328, 21, 82)),
        336 => Some(QppInterleaver::new(336, 115, 84)),
        344 => Some(QppInterleaver::new(344, 193, 86)),
        352 => Some(QppInterleaver::new(352, 21, 44)),
        360 => Some(QppInterleaver::new(360, 133, 90)),
        368 => Some(QppInterleaver::new(368, 81, 46)),
        376 => Some(QppInterleaver::new(376, 45, 94)),
        384 => Some(QppInterleaver::new(384, 23, 48)),
        392 => Some(QppInterleaver::new(392, 243, 98)),
        400 => Some(QppInterleaver::new(400, 151, 40)),
        408 => Some(QppInterleaver::new(408, 155, 102)),
        416 => Some(QppInterleaver::new(416, 25, 52)),
        424 => Some(QppInterleaver::new(424, 51, 106)),
        432 => Some(QppInterleaver::new(432, 47, 72)),
        440 => Some(QppInterleaver::new(440, 91, 110)),
        448 => Some(QppInterleaver::new(448, 29, 168)),
        456 => Some(QppInterleaver::new(456, 29, 114)),
        464 => Some(QppInterleaver::new(464, 247, 58)),
        472 => Some(QppInterleaver::new(472, 29, 118)),
        480 => Some(QppInterleaver::new(480, 89, 180)),
        488 => Some(QppInterleaver::new(488, 91, 122)),
        496 => Some(QppInterleaver::new(496, 157, 62)),
        504 => Some(QppInterleaver::new(504, 55, 84)),

        // 16 bit step
        512 => Some(QppInterleaver::new(512, 31, 64)),
        528 => Some(QppInterleaver::new(528, 17, 66)),
        544 => Some(QppInterleaver::new(544, 35, 68)),
        560 => Some(QppInterleaver::new(560, 227, 420)),
        576 => Some(QppInterleaver::new(576, 65, 96)),
        592 => Some(QppInterleaver::new(592, 19, 74)),
        608 => Some(QppInterleaver::new(608, 37, 76)),
        624 => Some(QppInterleaver::new(624, 41, 234)),
        640 => Some(QppInterleaver::new(640, 39, 80)),
        656 => Some(QppInterleaver::new(656, 185, 82)),
        672 => Some(QppInterleaver::new(672, 43, 252)),
        688 => Some(QppInterleaver::new(688, 21, 86)),
        704 => Some(QppInterleaver::new(704, 155, 44)),
        720 => Some(QppInterleaver::new(720, 79, 120)),
        736 => Some(QppInterleaver::new(736, 139, 92)),
        752 => Some(QppInterleaver::new(752, 23, 94)),
        768 => Some(QppInterleaver::new(768, 217, 48)),
        784 => Some(QppInterleaver::new(784, 25, 98)),
        800 => Some(QppInterleaver::new(800, 17, 80)),
        816 => Some(QppInterleaver::new(816, 127, 102)),
        832 => Some(QppInterleaver::new(832, 25, 52)),
        848 => Some(QppInterleaver::new(848, 239, 106)),
        864 => Some(QppInterleaver::new(864, 17, 48)),
        880 => Some(QppInterleaver::new(880, 137, 110)),
        896 => Some(QppInterleaver::new(896, 215, 112)),
        912 => Some(QppInterleaver::new(912, 29, 114)),
        928 => Some(QppInterleaver::new(928, 15, 58)),
        944 => Some(QppInterleaver::new(944, 147, 118)),
        960 => Some(QppInterleaver::new(960, 29, 60)),
        976 => Some(QppInterleaver::new(976, 59, 122)),
        992 => Some(QppInterleaver::new(992, 65, 124)),
        1008 => Some(QppInterleaver::new(1008, 55, 84)),

        // 32 bit step
        1024 => Some(QppInterleaver::new(1024, 31, 64)),
        1056 => Some(QppInterleaver::new(1056, 17, 66)),
        1088 => Some(QppInterleaver::new(1088, 171, 204)),
        1120 => Some(QppInterleaver::new(1120, 67, 140)),
        1152 => Some(QppInterleaver::new(1152, 35, 72)),
        1184 => Some(QppInterleaver::new(1184, 19, 74)),
        1216 => Some(QppInterleaver::new(1216, 39, 76)),
        1248 => Some(QppInterleaver::new(1248, 19, 78)),
        1280 => Some(QppInterleaver::new(1280, 199, 240)),
        1312 => Some(QppInterleaver::new(1312, 21, 82)),
        1344 => Some(QppInterleaver::new(1344, 211, 252)),
        1376 => Some(QppInterleaver::new(1376, 21, 86)),
        1408 => Some(QppInterleaver::new(1408, 43, 88)),
        1440 => Some(QppInterleaver::new(1440, 149, 60)),
        1472 => Some(QppInterleaver::new(1472, 45, 92)),
        1504 => Some(QppInterleaver::new(1504, 49, 846)),
        1536 => Some(QppInterleaver::new(1536, 71, 48)),
        1568 => Some(QppInterleaver::new(1568, 13, 28)),
        1600 => Some(QppInterleaver::new(1600, 17, 80)),
        1632 => Some(QppInterleaver::new(1632, 25, 102)),
        1664 => Some(QppInterleaver::new(1664, 183, 104)),
        1696 => Some(QppInterleaver::new(1696, 55, 954)),
        1728 => Some(QppInterleaver::new(1728, 127, 96)),
        1760 => Some(QppInterleaver::new(1760, 27, 110)),
        1792 => Some(QppInterleaver::new(1792, 29, 112)),
        1824 => Some(QppInterleaver::new(1824, 29, 114)),
        1856 => Some(QppInterleaver::new(1856, 57, 116)),
        1888 => Some(QppInterleaver::new(1888, 45, 354)),
        1920 => Some(QppInterleaver::new(1920, 31, 120)),
        1952 => Some(QppInterleaver::new(1952, 59, 610)),
        1984 => Some(QppInterleaver::new(1984, 185, 124)),
        2016 => Some(QppInterleaver::new(2016, 113, 420)),

        // 64 bit step
        2048 => Some(QppInterleaver::new(2048, 31, 64)),
        2112 => Some(QppInterleaver::new(2112, 17, 66)),
        2176 => Some(QppInterleaver::new(2176, 171, 136)),
        2240 => Some(QppInterleaver::new(2240, 209, 420)),
        2304 => Some(QppInterleaver::new(2304, 253, 216)),
        2368 => Some(QppInterleaver::new(2368, 367, 444)),
        2432 => Some(QppInterleaver::new(2432, 265, 456)),
        2496 => Some(QppInterleaver::new(2496, 181, 468)),
        2560 => Some(QppInterleaver::new(2560, 39, 80)),
        2624 => Some(QppInterleaver::new(2624, 27, 164)),
        2688 => Some(QppInterleaver::new(2688, 127, 504)),
        2752 => Some(QppInterleaver::new(2752, 143, 172)),
        2816 => Some(QppInterleaver::new(2816, 43, 88)),
        2880 => Some(QppInterleaver::new(2880, 29, 300)),
        2944 => Some(QppInterleaver::new(2944, 45, 92)),
        3008 => Some(QppInterleaver::new(3008, 157, 188)),
        3072 => Some(QppInterleaver::new(3072, 47, 96)),
        3136 => Some(QppInterleaver::new(3136, 13, 28)),
        3200 => Some(QppInterleaver::new(3200, 111, 240)),
        3264 => Some(QppInterleaver::new(3264, 443, 204)),
        3328 => Some(QppInterleaver::new(3328, 51, 104)),
        3392 => Some(QppInterleaver::new(3392, 51, 212)),
        3456 => Some(QppInterleaver::new(3456, 451, 192)),
        3520 => Some(QppInterleaver::new(3520, 257, 220)),
        3584 => Some(QppInterleaver::new(3584, 57, 336)),
        3648 => Some(QppInterleaver::new(3648, 313, 228)),
        3712 => Some(QppInterleaver::new(3712, 271, 232)),
        3776 => Some(QppInterleaver::new(3776, 179, 236)),
        3840 => Some(QppInterleaver::new(3840, 331, 120)),
        3904 => Some(QppInterleaver::new(3904, 363, 244)),
        3968 => Some(QppInterleaver::new(3968, 375, 248)),
        4032 => Some(QppInterleaver::new(4032, 127, 168)),
        4096 => Some(QppInterleaver::new(4096, 31, 64)),
        4160 => Some(QppInterleaver::new(4160, 33, 130)),
        4224 => Some(QppInterleaver::new(4224, 43, 264)),
        4288 => Some(QppInterleaver::new(4288, 33, 134)),
        4352 => Some(QppInterleaver::new(4352, 477, 408)),
        4416 => Some(QppInterleaver::new(4416, 35, 138)),
        4480 => Some(QppInterleaver::new(4480, 233, 280)),
        4544 => Some(QppInterleaver::new(4544, 357, 142)),
        4608 => Some(QppInterleaver::new(4608, 337, 480)),
        4672 => Some(QppInterleaver::new(4672, 37, 146)),
        4736 => Some(QppInterleaver::new(4736, 71, 444)),
        4800 => Some(QppInterleaver::new(4800, 71, 120)),
        4864 => Some(QppInterleaver::new(4864, 37, 152)),
        4928 => Some(QppInterleaver::new(4928, 39, 462)),
        4992 => Some(QppInterleaver::new(4992, 127, 234)),
        5056 => Some(QppInterleaver::new(5056, 39, 158)),
        5120 => Some(QppInterleaver::new(5120, 39, 80)),
        5184 => Some(QppInterleaver::new(5184, 31, 96)),
        5248 => Some(QppInterleaver::new(5248, 113, 902)),
        5312 => Some(QppInterleaver::new(5312, 41, 166)),
        5376 => Some(QppInterleaver::new(5376, 251, 336)),
        5440 => Some(QppInterleaver::new(5440, 43, 170)),
        5504 => Some(QppInterleaver::new(5504, 21, 86)),
        5568 => Some(QppInterleaver::new(5568, 43, 174)),
        5632 => Some(QppInterleaver::new(5632, 45, 176)),
        5696 => Some(QppInterleaver::new(5696, 45, 178)),
        5760 => Some(QppInterleaver::new(5760, 161, 120)),
        5824 => Some(QppInterleaver::new(5824, 89, 182)),
        5888 => Some(QppInterleaver::new(5888, 323, 184)),
        5952 => Some(QppInterleaver::new(5952, 47, 186)),
        6016 => Some(QppInterleaver::new(6016, 23, 94)),
        6080 => Some(QppInterleaver::new(6080, 47, 190)),
        6144 => Some(QppInterleaver::new(6144, 263, 480)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::interleaver::Interleaver;

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn can_interleave_iterator() {
        for length in 0..6144 {
            if let Some(interleaver) = create(length) {
                can_interleave_iterator_case(interleaver);
            }
        }
    }

    fn can_interleave_iterator_case(interleaver: QppInterleaver) {
        // Given
        let mut interleaved = HashSet::with_capacity(interleaver.len());

        // When
        for i in interleaver.iter() {
            interleaved.insert(*i);
            assert_eq!(interleaver.get(i.0), i.1);
        }

        // Then
        assert_eq!(interleaver.len(), interleaved.len());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn can_interleave() {
        // Given
        let interleaver = QppInterleaver::new(16, 1, 4);
        let mut buffer: Vec<i8> = (0..16).collect();

        // When
        interleaver.interleave(&mut buffer);

        // Then
        assert_eq!(
            [0, 5, 2, 7, 4, 9, 6, 11, 8, 13, 10, 15, 12, 1, 14, 3].to_vec(),
            buffer,
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn can_deinterleave() {
        // Given
        let interleaver = QppInterleaver::new(16, 1, 4);
        let mut buffer: Vec<i8> = (0..16).collect();

        // When
        interleaver.deinterleave(&mut buffer);

        // Then
        assert_eq!(
            [0, 13, 2, 15, 4, 1, 6, 3, 8, 5, 10, 7, 12, 9, 14, 11].to_vec(),
            buffer,
        );
    }
}
