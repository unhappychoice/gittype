use gittype::domain::models::rank::{Rank, RankTier};

macro_rules! rank_range_tests {
    ($($test:ident => $index:expr, $min:expr, $max:expr),+ $(,)?) => {
        $(
            #[test]
            fn $test() {
                let rank = &Rank::all_ranks()[$index];
                assert_eq!(rank.min_score, $min);
                assert_eq!(rank.max_score, $max);
            }
        )+
    };
}

macro_rules! rank_tier_tests {
    ($($test:ident => $index:expr, $tier:expr),+ $(,)?) => {
        $(
            #[test]
            fn $test() {
                let rank = &Rank::all_ranks()[$index];
                assert_eq!(rank.tier(), &$tier);
            }
        )+
    };
}

rank_range_tests! {
    rank_range_00 => 0, 0, 800,
    rank_range_01 => 1, 801, 1200,
    rank_range_02 => 2, 1201, 1600,
    rank_range_03 => 3, 1601, 2000,
    rank_range_04 => 4, 2001, 2450,
    rank_range_05 => 5, 2451, 2900,
    rank_range_06 => 6, 2901, 3300,
    rank_range_07 => 7, 3301, 3700,
    rank_range_08 => 8, 3701, 4150,
    rank_range_09 => 9, 4151, 4550,
    rank_range_10 => 10, 4551, 5000,
    rank_range_11 => 11, 5001, 5600,
    rank_range_12 => 12, 5601, 5850,
    rank_range_13 => 13, 5851, 6000,
    rank_range_14 => 14, 6001, 6100,
    rank_range_15 => 15, 6101, 6250,
    rank_range_16 => 16, 6251, 6400,
    rank_range_17 => 17, 6401, 6550,
    rank_range_18 => 18, 6551, 6700,
    rank_range_19 => 19, 6701, 6850,
    rank_range_20 => 20, 6851, 7000,
    rank_range_21 => 21, 7001, 7100,
    rank_range_22 => 22, 7101, 7250,
    rank_range_23 => 23, 7251, 7500,
    rank_range_24 => 24, 7501, 7800,
    rank_range_25 => 25, 7801, 8000,
    rank_range_26 => 26, 8001, 8100,
    rank_range_27 => 27, 8101, 8250,
    rank_range_28 => 28, 8251, 8400,
    rank_range_29 => 29, 8401, 8500,
    rank_range_30 => 30, 8501, 8650,
    rank_range_31 => 31, 8651, 8800,
    rank_range_32 => 32, 8801, 8950,
    rank_range_33 => 33, 8951, 9100,
    rank_range_34 => 34, 9101, 9250,
    rank_range_35 => 35, 9251, 9500,
    rank_range_36 => 36, 9501, 9800,
    rank_range_37 => 37, 9801, 9950,
    rank_range_38 => 38, 9951, 10100,
    rank_range_39 => 39, 10101, 10200,
    rank_range_40 => 40, 10201, 10350,
    rank_range_41 => 41, 10351, 10500,
    rank_range_42 => 42, 10501, 10650,
    rank_range_43 => 43, 10651, 10800,
    rank_range_44 => 44, 10801, 10950,
    rank_range_45 => 45, 10951, 11100,
    rank_range_46 => 46, 11101, 11200,
    rank_range_47 => 47, 11201, 11400,
    rank_range_48 => 48, 11401, 11700,
    rank_range_49 => 49, 11701, 12250,
    rank_range_50 => 50, 12251, 12800,
    rank_range_51 => 51, 12801, 13400,
    rank_range_52 => 52, 13401, 13950,
    rank_range_53 => 53, 13951, 14500,
    rank_range_54 => 54, 14501, 15100,
    rank_range_55 => 55, 15101, 15650,
    rank_range_56 => 56, 15651, 16200,
    rank_range_57 => 57, 16201, 16800,
    rank_range_58 => 58, 16801, 17350,
    rank_range_59 => 59, 17351, 17900,
    rank_range_60 => 60, 17901, 18500,
    rank_range_61 => 61, 18501, 19100,
    rank_range_62 => 62, 19101, u32::MAX,
}

rank_tier_tests! {
    rank_tier_00 => 0, RankTier::Beginner,
    rank_tier_01 => 1, RankTier::Beginner,
    rank_tier_02 => 2, RankTier::Beginner,
    rank_tier_03 => 3, RankTier::Beginner,
    rank_tier_04 => 4, RankTier::Beginner,
    rank_tier_05 => 5, RankTier::Beginner,
    rank_tier_06 => 6, RankTier::Beginner,
    rank_tier_07 => 7, RankTier::Beginner,
    rank_tier_08 => 8, RankTier::Beginner,
    rank_tier_09 => 9, RankTier::Beginner,
    rank_tier_10 => 10, RankTier::Beginner,
    rank_tier_11 => 11, RankTier::Beginner,
    rank_tier_12 => 12, RankTier::Intermediate,
    rank_tier_13 => 13, RankTier::Intermediate,
    rank_tier_14 => 14, RankTier::Intermediate,
    rank_tier_15 => 15, RankTier::Intermediate,
    rank_tier_16 => 16, RankTier::Intermediate,
    rank_tier_17 => 17, RankTier::Intermediate,
    rank_tier_18 => 18, RankTier::Intermediate,
    rank_tier_19 => 19, RankTier::Intermediate,
    rank_tier_20 => 20, RankTier::Intermediate,
    rank_tier_21 => 21, RankTier::Intermediate,
    rank_tier_22 => 22, RankTier::Intermediate,
    rank_tier_23 => 23, RankTier::Intermediate,
    rank_tier_24 => 24, RankTier::Advanced,
    rank_tier_25 => 25, RankTier::Advanced,
    rank_tier_26 => 26, RankTier::Advanced,
    rank_tier_27 => 27, RankTier::Advanced,
    rank_tier_28 => 28, RankTier::Advanced,
    rank_tier_29 => 29, RankTier::Advanced,
    rank_tier_30 => 30, RankTier::Advanced,
    rank_tier_31 => 31, RankTier::Advanced,
    rank_tier_32 => 32, RankTier::Advanced,
    rank_tier_33 => 33, RankTier::Advanced,
    rank_tier_34 => 34, RankTier::Advanced,
    rank_tier_35 => 35, RankTier::Advanced,
    rank_tier_36 => 36, RankTier::Expert,
    rank_tier_37 => 37, RankTier::Expert,
    rank_tier_38 => 38, RankTier::Expert,
    rank_tier_39 => 39, RankTier::Expert,
    rank_tier_40 => 40, RankTier::Expert,
    rank_tier_41 => 41, RankTier::Expert,
    rank_tier_42 => 42, RankTier::Expert,
    rank_tier_43 => 43, RankTier::Expert,
    rank_tier_44 => 44, RankTier::Expert,
    rank_tier_45 => 45, RankTier::Expert,
    rank_tier_46 => 46, RankTier::Expert,
    rank_tier_47 => 47, RankTier::Expert,
    rank_tier_48 => 48, RankTier::Legendary,
    rank_tier_49 => 49, RankTier::Legendary,
    rank_tier_50 => 50, RankTier::Legendary,
    rank_tier_51 => 51, RankTier::Legendary,
    rank_tier_52 => 52, RankTier::Legendary,
    rank_tier_53 => 53, RankTier::Legendary,
    rank_tier_54 => 54, RankTier::Legendary,
    rank_tier_55 => 55, RankTier::Legendary,
    rank_tier_56 => 56, RankTier::Legendary,
    rank_tier_57 => 57, RankTier::Legendary,
    rank_tier_58 => 58, RankTier::Legendary,
    rank_tier_59 => 59, RankTier::Legendary,
    rank_tier_60 => 60, RankTier::Legendary,
    rank_tier_61 => 61, RankTier::Legendary,
    rank_tier_62 => 62, RankTier::Legendary,
}
