use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref IPHONE_NAME_TO_PPI: HashMap<Vec<&'static str>, i32> = {
            [
            (
                vec![
                // iPhone 12 mini
                "iPhone13,1"
                ],
                476
            ),
                (
                    vec![
                        // iPhone 12
                        "iPhone13,2",
                        // iPhone 12 Pro
                        "iPhone13,3"
                    ],
                    460
                ),
                (
                    vec![
                    // iPhone 12 Pro Max
                    "iPhone13,4",
                    // iPhone 11 Pro
                    "iPhone12,3",
                    // iPhone 11 Pro Max
                    "iPhone12,5",
                    // iPhone XS
                    "iPhone11,2",
                    // iPhone XS Max
                    "iPhone11,4", "iPhone11,6",
                    // iPhone X
                    "iPhone10,3", "iPhone10,6"
                ],
                458
                ),
                (
                    vec![
                        // iPhone 8 Plus
                        "iPhone10,2", "iPhone10,5",
                        // iPhone 7 Plus
                        "iPhone9,2", "iPhone9,4",
                        // iPhone 6S Plus
                        "iPhone8,2",
                        // iPhone 6 Plus
                        "iPhone7,1"
                    ],
                    401
                ),
                (
                    vec![
                // iPhone 11
                "iPhone12,1",
                // iPhone XR
                "iPhone11,8",

                // iPhone SE (2nd generation)
                "iPhone12,8",
                // iPhone 8
                "iPhone10,1", "iPhone10,4",
                // iPhone 7
                "iPhone9,1", "iPhone9,3",
                // iPhone 6S
                "iPhone8,1",
                // iPhone 6
                "iPhone7,2",

                // iPhone SE
                "iPhone8,4",
                // iPhone 5S
                "iPhone6,1", "iPhone6,2",
                // iPhone 5C
                "iPhone5,3", "iPhone5,4",
                // iPhone 5
                "iPhone5,1", "iPhone5,2",
                // iPod touch (7th generation)
                "iPod9,1",
                // iPod touch (6th generation)
                "iPod7,1",
                // iPod touch (5th generation)
                "iPod5,1",

                // iPhone 4S
                "iPhone4,1",

                // iPad mini (5th generation)
                "iPad11,1", "iPad11,2",
                // iPad mini 4
                "iPad5,1", "iPad5,2",
                // iPad mini 3
                "iPad4,7", "iPad4,8", "iPad4,9",
                // iPad mini 2
                "iPad4,4", "iPad4,5", "iPad4,6"
            ],
            326
                ),
                (
                    vec![
                        // iPad Air (4th generation)
                        "iPad13,1", "iPad13,2",
                        // iPad (8th generation)
                        "iPad11,6", "iPad11,7",
                        // iPad Pro (12.9″, 4th generation)
                        "iPad8,11", "iPad8,12",
                        // iPad Pro (11″, 2nd generation)
                        "iPad8,9", "iPad8,10",
                        // iPad (7th generation)
                        "iPad7,11", "iPad7,12",
                        // iPad Air (3rd generation)
                        "iPad11,3", "iPad11,4",
                        // iPad Pro (12.9″, 3rd generation)
                        "iPad8,5", "iPad8,6", "iPad8,7", "iPad8,8",
                        // iPad Pro (11″)
                        "iPad8,1", "iPad8,2", "iPad8,3", "iPad8,4",
                        // iPad (6th generation)
                        "iPad7,5", "iPad7,6",
                        // iPad Pro (10.5″)
                        "iPad7,3", "iPad7,4",
                        // iPad Pro (12.9″, 2nd generation)
                        "iPad7,1", "iPad7,2",
                        // iPad (5th generation)
                        "iPad6,11", "iPad6,12",
                        // iPad Pro (12.9″)
                        "iPad6,7", "iPad6,8",
                        // iPad Pro (9.7″)
                        "iPad6,3", "iPad6,4",
                        // iPad Air 2
                        "iPad5,3", "iPad5,4",
                        // iPad Air
                        "iPad4,1", "iPad4,2", "iPad4,3",
                        // iPad (4th generation)
                        "iPad3,4", "iPad3,5", "iPad3,6",
                        // iPad (3rd generation)
                        "iPad3,1", "iPad3,2", "iPad3,3"
                    ],
                    264
                ),
                (
                    vec![
                        // iPad mini
                        "iPad2,5", "iPad2,6", "iPad2,7"
                    ],
                    163
                ),
                (
                    vec![
                        // iPad 2
                        "iPad2,1", "iPad2,2", "iPad2,3", "iPad2,4"
                    ],
                    132
                )

            ].iter().cloned().collect()
    };
}
