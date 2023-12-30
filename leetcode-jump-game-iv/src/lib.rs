pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut neighborhood_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (index, value) in arr.iter().enumerate() {
            neighborhood_map.entry(*value).or_default().push(index)
        }

        // Self::find_shorted_path(&arr, &mut neighborhood_map, vec![0])

        let mut hops = 0;
        let mut queue = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        queue.push_back(0);

        if arr.len() == 1 {
            return 0;
        }

        while !queue.is_empty() {
            let current_level_elements = queue.len();

            for _ in 0..current_level_elements {
                if let Some(current) = queue.pop_front() {
                    if current == arr.len() - 1 {
                        return hops;
                    }
                    visited.insert(current);

                    if let Some(jumps) = neighborhood_map.get_mut(&arr[current]) {
                        if current > 0 {
                            jumps.push(current - 1);
                        }
                        if current + 1 < arr.len() {
                            jumps.push(current + 1);
                        }

                        while let Some(item) = jumps.pop() {
                            if visited.insert(item) {
                                queue.push_back(item)
                            }
                        }
                    }
                }
            }
            hops += 1;
        }
        hops
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_01() {
        let input = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        let expected_result = 3;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_02() {
        let input = vec![7];
        let expected_result = 0;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_03() {
        let input = vec![7, 6, 9, 6, 9, 6, 9, 7];
        let expected_result = 1;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_04() {
        let input = vec![7, 6, 9, 6, 2, 7, 9, 6, 9, 8, 8, 3, 2];
        // explanation 7 -> 7 -> 2 -> 2
        let expected_result = 3;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_05() {
        let input = vec![6, 1, 9];
        // explanation 7 -> 7 -> 2 -> 2
        let expected_result = 2;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_06() {
        let input = vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 9];
        // explanation 7 -> 7 -> 2 -> 2
        let expected_result = 2;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_07() {
        let input = vec![
            7872, 8708, -5921, -2477, -4437, 3504, 494, 8363, 3266, -5718, -4400, 8284, 2673,
            -7216, -1520, 5056, -270, -390, 4510, 6907, 4885, 1642, 4335, 4176, -4502, -5924,
            -9026, -1857, -8747, -9315, -7045, -8549, 9978, -966, -143, -5319, -9896, -1247, 5306,
            -7483, -7808, -3024, 2290, -753, 6052, -2015, -6839, -577, 9365, -8705, 5060, 2966,
            1346, 3267, -9473, -8652, -6022, 6889, -1737, 8735, -3011, 3377, 1630, -9130, 2030,
            7754, 5001, 5124, 3753, -211, 2685, -3568, 1084, 4294, -9065, 8336, -2744, 3822, -4699,
            6895, -3698, -5011, 9309, -4239, 3499, -9557, 4735, -7566, 9866, 4506, 7253, 2266,
            3585, -9733, -9377, 2478, -589, -5089, -8898, 9154, -8402, -4222, -4080, 5013, 8304,
            1549, -4882, -3423, -70, 4254, -1765, -8795, -1639, -4688, -8124, 3014, 3721, 7273,
            -1973, 9258, -2037, -3496, 5888, 1207, 3694, -6569, 7275, -8546, 5389, -5547, 598,
            6086, 6958, 5446, -5781, -7410, -4451, -1595, 3153, 8497, 8950, -1374, 9568, 1804,
            7277, -2540, -9056, -6215, -3871, 3740, 3153, 0, 4386, 9742, 670, 5960, 79, 7398, 8447,
            -7702, -5374, 6893, 6138, 95, -801, 7474, 1621, -8632, 532, 2521, -9324, 3531, -3699,
            -723, 8927, 3334, -677, 1590, -3845, 9280, 7756, 5097, -773, 6232, 6579, 5770, 4527,
            -4667, -6164, -2210, -8204, 3646, -9688, 3979, 8484, -7198, 3916, -9788, 1412, -7760,
            9126, 9296, 5847, 9525, 5868, -8531, 1093, 7987, 6086, -6893, 9446, 5451, 3011, 4575,
            -1408, -9894, -3612, -1297, -6203, -7971, -4216, 6829, -3697, 1675, 8716, 1185, -3429,
            -5798, -3226, 9434, 7889, 871, 150, -9427, 7872, 7278, 6406, 701, -507, -4503, -823,
            -7702, 9869, 6847, -9175, 1226, -9598, 513, 3316, -7724, 4359, 5559, 2932, -7619, 5217,
            -2568, -9461, 5456, 5784, -4620, 6829, 4748, -1616, -8986, 4159, 7034, -9761, 3121,
            -8398, -3038, -3789, 5294, 6482, -5925, -2015, -9114, 5394, -7031, -8289, -8309, 3559,
            -3652, -3777, -3851, 4442, -6880, 9780, -7279, 9128, -942, -5132, -4598, -8399, 9287,
            -4086, 4649, 7351, 5571, -439, 8015, -1487, 2859, 5113, 8849, 6549, 722, -2430, -595,
            -178, 5223, -3741, -6105, -4568, 6768, 6585, -8431, -3751, -4215, -5896, -7844, -8405,
            -5582, 7022, 959, -3913, 838, 6859, 9143, 9899, 3446, -4788, 4502, 8132, -328, 1879,
            -1735, 4020, -6638, 2196, -8904, -7032, 2989, 4996, 2523, 6542, 8297, -8719, -1528,
            -1331, 7924, -6623, -1055, -6735, 3893, 5317, -1713, -3655, 7434, -4849, 9464, 4164,
            6363, 7547, -1325, 4797, -3646, 1804, 6115, 4069, 5023, -3423, 1346, -595, -8497,
            -4556, -3140, 6119, 5428, -9343, -4191, -9260, -9392, -1246, -9036, 8408, 489, -868,
            7534, 9903, 7783, 7704, 9603, 2219, 6615, -593, 4790, 1504, 7219, -884, 8140, -6256,
            1212, -2243, 5112, 1383, -8284, 2588, -8227, -4538, -592, -3720, -8927, 4322, 1126,
            -5578, 2969, -4441, -6888, -5319, -1371, 4092, -1207, -9963, -8111, 2199, -2801, -1121,
            -7455, 4504, 3200, 289, -3517, -261, -9149, -3924, -3791, 9084, 6336, 2027, 6690, 5922,
            2704, -7214, -6177, -8717, -6637, -1543, 8632, 4415, -1899, 8265, 3666, -7209, -3771,
            -6888, 1542, 3732, -2969, 1003, 7737, -7414, 2886, -5641, 1806, 9114, 480, 162, 2311,
            9738, -3000, -3748, 5947, 5101, 4994, 7496, 4743, 6466, 5263, 2265, 3267, 5291, -1231,
            3154, -8469, -4673, -3350, -6916, 1103, 1320, 9747, 8547, 3151, -6190, -8916, -6748,
            8998, -6049, -9726, -3430, 5459, 539, -7986, 2060, -9975, 7926, 9273, 4418, 577, -7451,
            5232, -7577, -8598, -7364, 128, -6726, 709, 7481, 8992, -4542, 421, 7599, 2744, -6073,
            -5180, -5611, -6340, -3742, -4946, 6936, 9563, -5904, 9690, -4751, 3088, -8352, 5193,
            6115, 4504, -9696, 6547, 6229, -6762, -7688, 536, -8474, 8490, 1509, -688, -3621, 4706,
            1509, -9198, 2083, 8775, 2291, 3660, 2785, 680, 7452, -2255, -9065, 8874, -3791, -7767,
            -5896, -640, 5130, 2934, 7808, 7416, 8998, -9471, -6585, -8762, 967, -6254, 5587, 5480,
            -2862, -6395, 6742, 8999, 6495, -5127, 4320, -5412, 1859, 916, 9116, -9334, -8868,
            -4445, 3675, -3569, 5087, -2023, 950, -3629, -2400, 3192, -4764, -976, -5635, 1036,
            9356, 7, 1893, 6931, -8958, -6465, -3229, 3253, 511, 4495, 5075, -3466, 3820, 7056,
            5974, -3518, -6588, -8365, -3436, 4533, -2811, 8262, 5037, 3893, -3433, 7734, -812,
            5079, 6265, -7187, 8108, 3278, -5234, -6452, 7565, -6333, -1116, 8045, 6033, -8654,
            -65, 3818, 391, -5512, 8349, -9966, -5692, 3377, 6291, -7308, -9036, 4931, 4121, -2883,
            6544, -8584, -7662, -7822, -5178, -517, 4337, 9946, -2969, 5233, -2308, 9187, -525,
            8516, 8243, 8468, 3193, 9175, -6052, 7204, 8893, 2802, -4008, 608, 4129, 9853, 4479,
            7335, 7814, -376, -9252, -5599, 2070, 1929, -2285, -6271, -4645, 1086, 7220, 4894,
            4715, -2733, -7148, -3633, 6680, 4283, -4558, -4686, 7919, -6701, 5504, -6741, -1978,
            128, -5594, -4927, -3708, 412, 6236, 6217, 1429, -3958, 6495, -4416, 8127, 9049, 2662,
            8217, -9071, 6495, -6180, -2040, 9420, 5955, 8043, 1827, 8615, 7279, -9557, 421, 7842,
            -5317, 1321, 7182, -7678, 4489, 6964, -3534, 748, 9354, 8011, 8202, 7933, 7887, -3482,
            5561, 6771, -6501, 554, 6217, -3038, -7577, -9014, 2217, 662, 4626, -3460, -7594, 1788,
            -7228, -5884, 9801, 6121, 5999, -3911, 5723, 6458, 3200, -9579, 3741, -6451, -5305,
            -1549, 6898, -1421, 3276, 1617, -8531, 9664, -927, 4453, -3816, 8883, -7822, 1396,
            8865, 5737, 4385, 2597, 9950, -5090, -8875, 2985, 1097, 9017, 8865, -9450, -4790, 2989,
            173, 2618, 3832, 6042, 5004, -1898, 2591, 5271, -6236, -8708, 9286, -8567, 2030, 9030,
            8616, 1201, -2594, 6060, -2057, 1504, 7196, 6669, -8800, -6146, 7690, -9212, 419, 396,
            -361, -6226, 3110, -2320, -867, -8933, 8532, -5202, -9239, 310, 1676, -6426, 2296,
            6570, 1936, -8916, -582, 729, 909, 4370, 8049, -7761, 6452, 6281, 5705, -9342, 9372,
            -3988, -2259, -4034, -1796, 5370, 138, -8952, -9038, 2672, 1831, -5093, -4924, -2857,
            7309, -8899, -5188, -7351, 7370, -4832, -299, 589, -363, 2780, 6162, 9104, -2227,
            -8212, -57, 722, 4643, -5447, 6052, -8004, -6103, 4020, -9874, 3530, -9906, 4162,
            -3611, -6745, 3663, -8903, -2560, -3937, -6225, -5759, 7974, 5508, 8353, -4088, -3789,
            7009, 1087, 3791, -2692, -8282, -5622, 3596, 6970, 7524, -7591, -7391, 1305, -1786,
            -8367, -5196, 6951, 3491, -8933, 5531, -4810, -8274, 7259, 7988, 5990, 1607, -1449,
            -5811, 6398, -3868, 3720, -9510, 4291, -1602, -1415, 477, 6858, -8265, -6349, 1547,
            7463, 1663, 3110, -8599, 1660, 5843, -5924, 8608, -7845, 8012, 3273, 4905, 7218, 7253,
            -2211, 2914, 5760, 3413, -5873, 9746, -3446, 3947, -311, 961, -8154, 8780, 915, 9524,
            -3449, 1562, 2892, -1974, 4048, 5070, -9847, 8991, -5089, 7790, -1363, -3988, 4993,
            9967, 8930, 371, 3408, -9471, -938, 7025, -8047, 3861, 3597, 5466, 201, -2823, -714,
            -6921, 2723, 3999, -5214, 7451, 7142, -2976, -7357, -7108, 2657, 7209, 1069, -2035,
            -1569, -8241, 1493, 6017, -6934, -8100, -3568, -9028, -4306, 8029, -7868, 7813, -1965,
            4956, -4665, -4891, -2951, 7535, 4583, 3625, -6839, 3981, -4184, -8381, -5369, -1601,
            -7622, 422, -7606, -2753, 4976, -799, -2674, 2998, 7061, -3350, 8360, 2825, -1507,
            -3512, 6275, -8747, -9372, -9135, 5572, -8048, 3228, 7372, 3710, -9647, 6759, 6620,
            7484, 2903, 9790, 8501, -7611, -6313, -4835, -7325, 1405, 4411, 8044, -304, 9417, 5334,
            4845, -8229, 5620, -9056, 8116, -4, 1682, 7959, -2518, -2199, -5278, 4886, -3042, 6744,
            -8590, -1095, 1949, -6123, -5274, 3192, -7561, 600, -1721, 1316, 2849, -4691, -3949,
            5244, -8172, 3954, 8083, -6747, 9333, 6674, -3398, 1981, -7702, 579, 8247, -1781,
            -7894, 8344, -3744, 6498, -4744, -9082, 9313, 1762, 6594, 5064, 617, -2430, -9352,
            3567, -538, -914, -4391, -4582, 8004, 9300, -7206, -4659, 3507, -4608, -59, -8219,
            2798, 7580, -4199, 6083, 5069, -3964, -3850, 3535, -7081, 1926, 4473, -7009, 6398, 193,
            8287, 5439, 7751, 5883, 1795, -6441, -6854, 7654, -3893, -2840, -3916, -1708, 1890,
            6673, 7385, -592, -3226, 154, 9217, -6140, 9128, -4961, -6511, 3316, 791, 7017, -5718,
            949, -9660, 6228, -3991, -4112, 9943, -8400, -2442, -2255, -3640, 5917, 4307, 2770,
            6953, -8282, 6645, 5526, -6934, 7532, -7233, 9332, 9015, -6916, 3844, 4009, -4545,
            1752, -3211, 2452, 3325, 3313, -2653, -6046, 1953, 6526, -4964, -2746, -7772, -784,
            -9634, -9678, 3546, 8894, -4329, 8183, 8837, -3202, -5654, -4812, 7605, -9409, -1024,
            -4381, -2592, -9116, 1315, 8783, 4621, -4308, 2653, -8349, -4549, 1730, 9326, -3072,
            6231, -4244, 3915, 6806, 8690, -7841, -812, 9575, 7109, -1180, 2048, 4397, -1545, -879,
            8691, 3504, 9082, -2596, -5033, -8508, -767, 8618, -8094, 9286, -2865, 9130, 5401,
            5284, -8273, -4988, -2938, 5613, -2484, 2959, -7571, 4067, 7547, 5673, 4765, -4032,
            1734, 338, 1781, 1201, -2975, -804, 2785, -8345, -9945, -6657, -5715, -1032, -3976,
            7204, 6976, 6282, -2445, -2201, -8698, -16, -4397, -1566, -4571, -9398, 6964, 5618,
            -5783, -5587, -8040, 5699, -7890, 9079, -9472, -4049, -8556, 3838, 9493, 1293, 1952,
            6017, 215, -443, 8410, 6029, 9856, -3848, -8154, 8627, -9643, 5958, -5968, -2209, 4742,
            8510, -5272, 2568, 391, -5210, 8762, 3363, 5866, -4500, -2779, -1522, -6203, 677, 6954,
            -693, 2168, -4534, 8833, 1652, 7036, 2864, 7167, 5363, -7816, 8127, 3019, -2475, 8562,
            4522, -4558, 773, 2452, -2867, 4192, -3230, 3439, 102, -8905, -7054, 3590, -6641,
            -5921, -6351, -7734, 4711, 4447, 747, -1612, 7486, 6455, 9496, -265, 9903, -7953,
            -1609, 8933, -6449, -1612, -3769, -1668, -1631, -7975, 1244, 7010, -3990, 9787, 3644,
            7639, -9874, -6581, -4448, -2623, -2847, 7177, -5938, -8421, -7135, -2096, -3227, 4965,
            -3861, -1155, 4348, -3443, -4367, -6694, -61, 9868, -2408, 9021, 1292, 82, -2404, 1584,
            834, -5625, 3673, -1892, 383, -5549, -7817, -5909, -4211, -1108, 950, -5544, -5028,
            -567, 3446, -1312, -2752, 9601, 1763, 233, 3419, -4268, -2134, 7987, -5371, 2341, 8388,
            2668, 5602, -5855, 2196, 450, -7577, 9866, -402, -2485, -1047, 5013, -9200, 7182, 9277,
            -8806, -4450, -3697, -4194, -5679, 4795, -2829, -3989, -8053, -4970, -8360, -1591,
            -9942, -1717, 5634, 4950, 6480, 9059, 9315, 8343, 582, 1766, 8273, 3069, 8253, -5984,
            -4046, -3048, 6978, 8118, 7370, 3826, -2693, 4215, 3807, 5459, -7229, 7378, 3424,
            -8668, -3123, -3495, -4859, -1890, -61, -5512, 154, -8733, -5017, 2059, 1949, -3612,
            -2559, -2966, -1120, 4748, -5910, -8096, -3234, 8256, -8300, 9065, -5248, -3512, -6110,
            2241, -586, 7742, 1289, 6217, -3399, -7923, 326, -8297, -8552, -8757, 4903, -5660,
            4380, 8148, -5722, -3722, -2196, 4493, -8732, -9380, 8251, -4960, -4427, -4615, -5100,
            8950, -517, 4799, -5749, 1226, 5473, 6603, 4294, -2391, 1365, -7292, -1099, -6113,
            -8850, 2584, 4107, -9056, 3646, -7753, -7779, 5228, 9395, 6572, 3611, -550, 9531, 9720,
            8271, 3367, -3781, 716, 2727, -5196, -2823, -2515, -9242, -4373, 1318, 43, -2123, 8227,
            -6824, -1398, 4339, -3669, -476, -294, -6835, 7550, -3492, 7963, 7506, -9260, 893,
            -247, -683, -2069, -5896, 3338, 8793, -8864, 5317, -1514, 7814, -3898, -6987, 4600,
            5177, 3167, 2896, -3118, 2487, 7162, 364, 8218, -9575, -5590, -8146, -1489, 1741,
            -7009, -1772, -8472, -1989, -6749, 3428, 1885, 6922, 1388, -2877, 1999, 3165, -6032,
            7978, 5492, 5679, -3997, -2320, 9079, -9610, -2392, 5941, -9143, -2226, -5888, -3444,
            8632, -3850, -826, -2670, 1508, -9248, -6530, 3367, -9675, -4837, -1448, -6108, 8675,
            4658, 4331, -6583, 1993, -2171, -9964, -3818, 8550, -2621, 3524, -9625, 7623, -9757,
            -8556, -4249, 8932, -440, -9153, 771, 5965, 3896, -447, -2863, 2507, 600, -7849, 7751,
            -5027, -5423, 7910, 6029, 257, -3464, -5663, 6363, -9424, -4027, 3291, 5561, 7019,
            3740, -2341, 3531, 5734, 6376, 122, -4561, 9710, 6072, -4409, 2382, -9242, -8212, -478,
            -1203, 6015, 8393, -872, 6467, 624, -4530, -3092, 8360, 7048, 8336, 449, -495, -2671,
            5750, 4082, 1496, 6276, 450, 1597, 3400, -3193, -1490, 8841, 6356, -1163, -1370, 5615,
            3069, -6012, 6573, -3632, -9169, 5057, -4675, -1841, 4919, 905, 4739, 148, -3701, 269,
            4809, -7434, -7464, -107, 4554, -1448, -4507, 7145, -4246, -4348, -7610, -2256, -5587,
            2340, -745, -3660, 2811, -8228, -3860, 2025, 8363, -8755, -3389, 2992, 5938, -4359,
            -9128, 3425, -9571, 9755, -4084, -4263, 1459, 2918, 9601, -3027, -7490, 6825, -9610,
            -2172, 3653, -7653, -4506, 6863, -8386, -8698, 6878, -2937, -5239, -4323, -3434, 6859,
            6087, -5800, 1072, 934, 6319, -5597, -9472, 9727, -2811, -5633, -8555, -9887, -5162,
            5002, 8380, -7281, 4082, -388, -926, 1818, 8511, -4886, 4767, 2927, 3617, -4307, -3326,
            -3380, 9387, -7445, -2753, 6259, 5585, 7168, -3922, -8025, -1301, -8030, 1015, -3357,
            402, -1445, 2033, -8205, -8515, -6632, -1031, -607, 7789, -7845, 2927, 3177, 9143,
            -2797, 2139, 4151, 122, -8094, 3159, -480, 3687, -1320, 5223, -4924, -1485, 1944, 4719,
            5475, 2808, 5798, -4038, 3753, -485, 1890, 4441, 3268, 5079, 7405, 470, 8306, -8795,
            8255, -6570, 3969, -1776, 1979, 5149, 5112, -6740, 2672, -4522, 9709, 3316, 1660,
            -5530, -764, 2668, 9237, 2277, -5677, -9681, 6669, 1607, 6592, 2377, 176, -6288, 5681,
            463, 1486, 481, 2285, -1589, 2175, 6592, -5188, 6121, 1456, 8962, -3592, -5249, 6690,
            -641, 599, 8604, 3997, 336, -7018, -3173, -6342, 5635, -8190, 4967, 9782, 893, 453,
            -5356, -7900, -8836, 1967, 6693, 2315, -4311, -8839, -9381, -9876, 7252, -7509, -1265,
            -5165, 351, 8300, -5599, -1594, -4644, -2780, -3807, 8886, 3444, -6360, -5451, -5783,
            -3865, 508, 5323, 8388, -6313, 9099, -9948, 539, -446, -8337, -9034, -1752, -3655,
            8423, -55, -2400, 1849, 2261, 1979, 3625, -1038, 4498, 4887, -2938, -5896, -2951, 5957,
            -1806, -1111, 5664, -6283, 6096, 9313, -5660, -2702, 2765, 8150, -5292, -2492, -3223,
            -9714, 7847, -3363, 680, 1045, 8315, 9866, 2356, -8592, -849, 4039, 742, -7741, -2402,
            6531, 5341, -5234, -4202, -6202, 3624, 9279, -4784, 2819, -7099, 2603, 9897, 7919,
            -7365, -4915, -3631, 6350, 7711, 8621, -8375, 8679, -1244, 6258, -1961, -8534, 44,
            6350, 3381, -4498, 8686, -1751, 3143, -5613, 3280, -5210, -2419, 89, -438, 756, -9975,
            5270, 2802, -9963, 6600, -2118, 1658, 3838, 2716, 5931, 3381, 6999, 2065, -7182, 1790,
            8491, -360, 8708, -2261, 8375, 7099, 9104, -206, 8665, 6121, -7379, -3126, -9242, 7458,
            -4037, -6716, 3320, 872, -7958, 4062, -7771, -4539, -3834, 9545, 8571, 1993, 316, 3335,
            4554, -6896, 7993, 4045, 950, 9038, 9017, -2931, -8089, -1273, -5802, -8770, -1331,
            -466, -4968, -8708, -3744, -3865, -3503, 6540, 7081, -9418, -9625, 8686, 9833, 7593,
            4234, -6003, 6057, 1535, 592, 7504, -1616, 4307, -324, 8616, 2603, -1708, 811, -346,
            3045, 5905, -9656, -2329, 9267, 2843, 5518, 8388, 1199, 5013, -5925, -4485, -1106,
            9227, 7793, 5475, 7028, -5229, 317, -3074, -1954, -6703, -3767, -9320, 3950, -1969,
            9850, 5131, -390, 7406, -8918, 7504, -9257, -5661, -6251, -5160, -72, -773, 7828, 3248,
            -8793, -9709, 100, -6637, -9071, -381, -8872, 3361, -4246, -4532, -5789, 648, 9426,
            -9647, 810, -3497, -6110, 5248, -2763, 2741, 9239, 8547, 1458, 930, -1532, -5189,
            -1222, 4007, 1895, 5525, -7261, -1052, 9857, 6519, -4831, -9051, -2124, 463, 3559,
            5489, 7807, -4473, 3428, -7644, -1577, 8754, 274, 4118, 6294, -1996, -8197, 1762, 4611,
            1344, -1504, -7246, 2741, 4359, -6373, 3408, -2938, -3233, -8880, -6081, 295, 8614,
            -24, -897, 7990, 9365, 9056, -8547, -494, -2455, -9761, 7659, 6865, -1410, 2802, -9779,
            4056, 9477, 8487, 5812, 8793, 9495, -2322, 7541, 1959, -4, 4682, 3639, -4437, -7767,
            -897, 7896, 9466, -8697, 2666, -8042, 3377, 6422, -8219, -1206, -7653, 5145, 2026,
            1486, -4968, 7434, -2133, 9647, 1873, 6999, 982, 8232, -3931, 9828, -83, -5600, 9021,
            5723, 958, 3444, -6499, 4742, 9843, 3453, 4583, 6862, 5934, 1316, 7304, -7748, 7032,
            5033, 6296, 6964, 5655, -2682, 2918, 5572, -3147, -5556, 3474, 1987, -6898, -391,
            -5033, 3575, 7532, -2372, -2847, 2497, 4614, -1423, -6486, 8415, -3833, 47, 3575,
            -1316, -1155, -3495, -4038, -6426, 8590, -7778, 2922, 1060, 2706, 7244, 6631, -1291,
            -183, -598, -648, 7955, 5717, -9839, 812, 7039, -6940, -5176, -7914, 1535, -1702,
            -9964, 3625, -2803, 4343, 9545, 3430, -3609, -5027, -3989, 6445, -4127, 100, 6383,
            -3589, -14, 4184, -3647, -581, 3222, 5815, 122, 8406, 6230, 1388, 7984, -7963, -5746,
            -5342, -2860, -8022, 5723, 4910, -8183, -601, 2319, 5719, 5507, 3471, -7806, -9053,
            9932, 4143, 2187, 3110, -200, -8074, 5961, 5363, 9593, -4602, -4195, -5424, 7479, 835,
            6319, 3704, -9989, -6265, 463, -8589, -6454, 3645, -3592, 3779, 6669, -609, -659,
            -5183, 7841, -1349, -6019, 4135, -1910, 5520, 4645, 3129, -9624, -3701, 406, 1495,
            -9966, -9777, -6340, -6443, 4320, -5787, -6359, 750, 6379, -371, -6335, -236, -8267,
            -2554, -6353, -3058, -2967, -5892, 7370, -9798, 3887, 5749, 2637, 4894, -1760, 1452,
            -9270, -6359, 3882, 4592, -3365, -2668, -5840, 9472, -709, 1438, 9903, -6785, -4138,
            -8409, 4162, -7904, 5567, -8952, -8993, 5033, 760, -4027, 1433, 3933, 6483, -8452,
            -1594, 2302, -6507, -9847, -8491, -1532, -6236, -4817, 2490, -3242, -6215, 2825, 3535,
            -6620, 3633, 814, -2633, 7760, 5270, 4640, 4304, 44, 2588, -4801, -2741, 2909, -2104,
            -6741, 2021, 6824, -3571, -4784, 7798, -618, -1201, 4253, -3482, 6415, -4270, 4220,
            2927, 2843, 5842, -6833, 937, 8815, 7709, -9829, 4034, 5072, -2133, 8290, -476, -5788,
            -4724, 2989, -6921, 984, -4855, -406, -9572, 7230, -7672, 1069, -6504, -8927, -2532,
            8280, 7427, 2547, 2926, 2832, -8730, -9894, -5404, 5483, -4751, 5935, 4956, -5401,
            -6888, -1603, -9073, 89, 5430, -6105, 4280, 6339, 990, -2498, -2733, -7813, -8724,
            9794, 6651, 4217, -7365, -4782, 9885, 2072, -1925, 6061, -5159, 8124, -5972, -643,
            1751, -784, 6009, -527, -1981, 3055, 7591, -4118, -6916, 2595, 8738, -1438, -5142,
            7723, 2773, -7558, -4591, 8356, 8102, -2596, 9546, 3669, -9810, 9174, 5679, -1686,
            6697, -5686, -2599, 3163, -4866, -8080, -4291, -5655, 1160, 6379, -6356, 3417, -9538,
            -3496, 3472, 871, 2538, 3695, -5547, -5095, -216, -9643, -3323, 1171, 8995, -9136,
            -5979, 8234, -4849, -8960, -1843, -2267, 3361, -3061, -9584, -4947, -5907, -7075, 1039,
            5510, 2340, 5776, 3201, 4719, 1848, 8525, 6528, -414, 5200, 3497, -988, 1279, 5285,
            4685, -5115, 3273, -7943, -3744, 8481, 66, -2572, -9238, -5100, -992, -9527, 3764,
            -2495, 5200, 7347, -9585, 4486, 634, -8645, -6353, 6492, 3935, 543, 7924, -7922, -1690,
            -9005, 1686, 1949, -8218, 8939, -9061, 5710, -5420, -5596, 7790, -1868, 4160, -5793,
            -673, 2812, 1724, 2384, -6195, 147, 6402, 6146, -3473, -5364, 9436, -7211, 4199, -5704,
            986, -906, 1496, 2627, -8063, 383, -7910, 8497, 3982, -3256, 8585, -371, -2206, 79,
            -9749, 2645, 1076, -1316, -6067, 9115, -2998, -150, -8412, -1954, 9713, 1486, 5707,
            -780, -6278, -7561, -2683, -2637, 6756, 9721, -6465, 1722, 3743, 8840, 6859, -3493,
            5069, 4533, -1250, -6029, 2172, -9829, 8591, -1857, 9569, -9099, 4703, 6909, 2538,
            -8393, -4504, -4692, -7666, -2722, -6813, 7378, -8586, 3428, -9989, 7534, 3538, -6972,
            5416, 4109, 7133, 3219, 1860, -267, -5974, -1312, -8094, -9961, -4747, -7307, 6402,
            6017, -7948, 7548, -2882, 2798, 3966, -3478, 8504, -5660, 54, 2647, -1545, 6029, 8940,
            -9112, -5015, -5432, -7738, 298, 6348, 1272, -2201, -6293, 4411, -9709, -8539, 4866,
            -2386, 5615, 3711, 1150, -1428, 2173, -3577, 1873, -879, 7025, -9101, 4062, 9997,
            -8335, -462, -3495, 3307, -8463, 3751, 7258, -1952, 6220, -5149, -9743, -5479, -1882,
            -8169, 6315, 5018, -8109, -1180, 9524, 5304, 7981, 709, 1532, -8077, -4207, 1611,
            -6247, 9904, -1776, -5624, 1843, 7330, -1, -7315, -8561, 6687, 3430, 8833, 8735, 6742,
            3608, 2423, 9149, -3071, -2602, -4825, -580, 5679, 7042, 2709, 1883, 9146, 1996, 3173,
            -2887, -1459, 5428, -1079, -694, 4273, 6433, 3244, 2737, -2272, 3007, -6826, 8104,
            7922, 2060, -8231, -4599, 9911, 2196, -5217, -6950, 3907, 5124, -3060, 6416, 824,
            -8859, 470, 5303, 2785, 1449, -8644, -7525, -1744, 229, 1368, 8921, -1966, -9070,
            -9641, -1512, 791, 294, 842, 9021, -8661, 8580, 8436, 683, -5635, 7296, 5403, -7955,
            2217, -5165, -4112, -309, -5105, -3767, 8202, -1494, 6381, 39, 8616, -5472, 6981, 5023,
            1019, -5090, -241, 8556, -3497, -6252, -8672, 6718, 4397, -897, -3601, -4326, 5711,
            5689, 5209, 227, -7219, -9787, 1093, -4235, 8318, 3337, -9926, 6574, -1415, 8657, 8532,
            5765, -2278, 1256, -1154, 1449, -3717, 3375, 5379, -4052, 2687, -1294, 4699, -244, 617,
            2945, -3336, 6958, 4169, 3744, -5656, -737, -5430, -8061, 589, -725, 6204, -4282, 7582,
            -3013, -6204, 5776, -5017, -402, -4220, 3357, -6312, -7730, 4748, 5441, 3880, 3560, 14,
            -6582, -4900, -3741, 3258, 4204, -4400, -1605, 814, 8253, 9670, 6494, -1708, 9973,
            -3960, -6436, 2447, -1039, 9584, 3098, 308, 1093, 4355, -9352, 5088, -4979, 6874,
            -4991, 763, -1301, -2264, 7168, 1544, 3136, 2843, 5336, 8202, 9134, 7372, 3339, 8092,
            1639, -7216, 2158, 9569, -7910, -8200, -1630, 1838, -581, 8874, 4634, -9852, 6219,
            5945, -15, -4471, -1381, -2161, 7190, -8209, -6934, -8499, -1067, -8407, -2440, 5679,
            7545, 6776, -1118, 9313, 4941, -5238, 8645, -7760, 7919, 5963, 9418, 4915, 1750, -9530,
            102, 5760, -4667, 6445, 7475, -7094, 2487, -5927, 6812, 8909, -4373, 7050, -9107,
            -7108, -7604, 1822, 996, -2739, -7845, 8343, 260, -8710, 6198, -4253, 786, -165, 1031,
            6591, -2824, 1962, 6444, -8074, 5440, 1572, 6098, 482, -6353, -1121, -8795, -5664,
            -8035, 9987, -6508, 4149, -3889, 3871, -2609, -1978, 8577, 4795, 1086, -6864, -3702,
            1632, -3456, -41, 7328, 9758, 9375, -8298, 6291, -9421, -8053, 9977, 4224, -6457,
            -3953, 5911, 6281, -7490, -216, 7713, -7932, -5311, -4295, 4225, -7182, 2011, 4993,
            -416, -5578, -3779, -3624, -3999, 3339, 4594, -9918, -328, -6508, -1187, 4077, -6904,
            -8850, -2553, -3781, -466, -6527, 278, 7498, -7633, -8037, -3736, -4714, -1831, -7155,
            -2341, -1246, 3333, 8625, 9315, 238, -9530, -4023, 5992, 8504, 6756, 5258, 4083, -8504,
            881, -4311, 8106, -6698, -6190, -5093, 2295, 6089, -2602, -444, -2308, -4532, -4471,
            976, 8732, -7139, -1011, -4810, -3530, 9147, -6915, 2476, -4849, 9690, 1565, 3339,
            4296, 9005, 2449, -5141, 8708, -8886, -9510, 8730, -1732, 5968, 3656, -8816, -5598,
            4643, -6950, 3749, -1670, 5947, -1120, 5510, 3656, -6113, -776, -8614, -2226, -9934,
            8933, 1780, 464, -7153, 7884, 8886, 3703, 8176, 9312, -9582, -7407, -8760, 3, 7409,
            -8053, -7512, -4828, 8728, -2388, 7768, 3554, 8629, 762, 1728, 5359, 1726, 7008, 1358,
            7808, 5931, -5677, -8337, 9904, 2302, 8674, -7125, -4908, 6615, 617, -7639, 3695, 225,
            -6313, 1860, -264, 3732, -7139, -3132, 5406, -5820, -5671, 1623, -4240, 3727, -9033,
            6895, 3336, 2393, 702, -3368, -9954, 849, -6690, 3793, -57, -6199, -2878, -6722, 4192,
            -6190, -8402, -1732, -8887, 7701, -4478, -2887, 6262, -2325, -4835, 5282, 8230, 730,
            9423, 2495, -8880, 670, -4219, 453, 191, 6232, 6903, 7150, 3162, -2791, -1077, -3842,
            7859, -5303, -3448, -518, -5836, 3071, -4270, 7704, 193, 1297, 8510, -4320, 3311,
            -1639, -8899, -1085, -2518, -4996, 6535, -490, 149, 326, 3771, -109, -4644, 5625, 9260,
            6435, 6103, 8447, -7301, 4335, 4740, 450, 4629, 6978, -6437, -5306, 702, -1218, 4348,
            1548, 4791, -9194, -9231, -2693, -4567, -1776, -5677, 4530, 5328, -7904, 116, -8508,
            1368, 8017, 3188, -4666, 6559, 5750, 4306, -7139, 4294, 7849, 138, -3087, 7220, 5002,
            -3569, 2145, 8360, -3035, 8910, -5093, -2217, 7585, 2768, 3313, 1334, -8342, 4086,
            -1899, 1298, 4246, 791, 6755, 1381, 4962, -4972, -5484, -4744, -517, -6110, 278, 3593,
            3853, 6071, 6642, -7610, -4297, -9228, 6043, 6931, -8772, 717, 3679, 6154, -9259,
            -4317, 5411, 6395, 1174,
        ];
        // explanation 7 -> 7 -> 2 -> 2
        let expected_result = 30;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }
}
