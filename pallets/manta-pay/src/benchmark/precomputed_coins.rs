// Copyright 2019-2022 Manta Network.
// This file is part of pallet-manta-pay.
//
// pallet-manta-pay is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pallet-manta-pay is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pallet-manta-pay.  If not, see <http://www.gnu.org/licenses/>.

//! Precomputed Coins
//!
//! THIS FILE IS AUTOMATICALLY GENERATED by `src/bin/precompute_coins.rs`. DO NOT EDIT.

pub(crate) const MINT: &[u8] = &[
	1, 8, 0, 0, 0, 4, 160, 134, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 74, 107, 21, 231,
	190, 12, 35, 250, 44, 22, 179, 148, 164, 173, 15, 184, 117, 43, 59, 123, 232, 187, 80, 154, 54,
	248, 51, 84, 47, 247, 55, 41, 3, 193, 58, 135, 164, 161, 62, 217, 142, 241, 215, 18, 146, 251,
	200, 181, 7, 245, 119, 159, 236, 255, 101, 121, 150, 76, 122, 152, 247, 116, 38, 118, 143, 112,
	57, 95, 248, 80, 254, 5, 93, 159, 18, 200, 152, 7, 18, 10, 222, 23, 110, 73, 17, 183, 19, 205,
	203, 228, 53, 227, 167, 148, 11, 88, 217, 25, 187, 26, 0, 213, 254, 139, 66, 154, 240, 164,
	198, 205, 28, 14, 79, 218, 181, 112, 24, 3, 19, 60, 37, 216, 205, 15, 200, 47, 169, 161, 72,
	135, 229, 45, 198, 44, 190, 111, 139, 95, 228, 21, 10, 90, 20, 17, 45, 57, 194, 45, 151, 208,
	47, 48, 111, 46, 134, 216, 91, 10, 204, 114, 177, 239, 37, 246, 233, 114, 179, 73, 101, 252,
	148, 241, 189, 217, 10, 161, 100, 209, 141, 221, 5, 207, 199, 160, 76, 168, 170, 190, 213, 73,
	18, 207, 132, 101, 184, 128, 23, 246, 250, 86, 26, 95, 38, 186, 225, 162, 37, 239, 31, 206,
	196, 4, 46, 208, 83, 42, 212, 210, 109, 162, 124, 107, 8, 254, 137, 92, 162, 72, 84, 201, 125,
	196, 28, 83, 131, 253, 112, 8, 122, 250, 153, 155, 200, 131, 24, 198, 71, 64, 29, 231, 19, 200,
	2, 65, 230, 185, 20, 15, 50, 176, 210, 217, 196, 89, 174, 162, 171, 22, 253, 120, 229, 22, 3,
	24, 56, 88, 247, 20, 26, 104, 246, 79, 216, 75, 137, 126, 93, 5, 115, 255, 173, 207, 137,
];

pub(crate) const PRIVATE_TRANSFER_INPUT: &[&[u8]] = &[
	&[
		1, 8, 0, 0, 0, 4, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 20, 6, 81, 61,
		120, 248, 96, 193, 17, 117, 21, 250, 202, 11, 128, 41, 122, 110, 5, 241, 210, 244, 159,
		226, 22, 7, 34, 125, 75, 192, 241, 72, 220, 204, 4, 119, 85, 71, 131, 84, 114, 58, 216, 18,
		168, 163, 168, 176, 21, 69, 105, 255, 69, 137, 65, 219, 168, 101, 123, 173, 51, 212, 199,
		255, 41, 243, 245, 7, 126, 172, 252, 189, 59, 186, 64, 237, 152, 58, 219, 85, 69, 95, 214,
		111, 61, 137, 179, 130, 50, 108, 209, 234, 114, 167, 70, 94, 194, 113, 44, 74, 0, 180, 22,
		136, 92, 72, 234, 221, 135, 163, 97, 229, 112, 105, 116, 44, 84, 130, 245, 67, 222, 65,
		177, 181, 43, 124, 228, 221, 93, 144, 157, 55, 151, 96, 18, 92, 70, 171, 142, 54, 100, 152,
		6, 136, 93, 162, 130, 31, 152, 78, 165, 88, 31, 129, 150, 203, 161, 11, 125, 22, 31, 23,
		80, 177, 4, 185, 136, 101, 190, 112, 20, 22, 196, 23, 201, 175, 0, 175, 251, 173, 209, 7,
		48, 5, 147, 18, 103, 84, 92, 130, 7, 169, 220, 243, 90, 170, 15, 194, 59, 104, 47, 66, 172,
		219, 17, 75, 19, 77, 64, 59, 3, 26, 44, 98, 44, 143, 85, 248, 137, 231, 61, 148, 180, 243,
		12, 115, 219, 254, 52, 97, 102, 12, 24, 128, 230, 99, 167, 129, 136, 222, 61, 43, 253, 219,
		14, 139, 128, 172, 80, 211, 173, 197, 11, 9, 237, 4, 73, 47, 59, 227, 109, 217, 84, 143,
		164, 118, 180, 91, 227, 69, 45, 99, 232, 227, 118, 24, 34, 109, 172, 154, 28, 204, 215, 38,
		183, 167, 98, 174, 74, 91, 254, 112, 152,
	],
	&[
		1, 8, 0, 0, 0, 4, 32, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 33, 196, 16, 32,
		203, 198, 154, 168, 148, 43, 11, 41, 189, 68, 164, 61, 49, 213, 60, 167, 61, 238, 196, 54,
		193, 215, 218, 178, 99, 97, 161, 21, 122, 122, 134, 60, 205, 171, 19, 126, 23, 224, 119,
		97, 80, 122, 186, 157, 200, 54, 3, 173, 210, 233, 125, 80, 126, 215, 204, 225, 240, 201,
		73, 247, 67, 5, 210, 39, 47, 133, 44, 60, 77, 245, 141, 72, 211, 198, 191, 237, 5, 67, 75,
		135, 137, 113, 212, 46, 107, 228, 212, 63, 82, 189, 205, 54, 131, 249, 204, 156, 0, 21,
		130, 162, 20, 206, 65, 141, 176, 219, 126, 244, 150, 117, 108, 173, 89, 100, 190, 243, 40,
		145, 77, 16, 214, 159, 91, 235, 39, 47, 249, 56, 118, 42, 176, 100, 167, 221, 174, 34, 233,
		177, 113, 251, 120, 178, 186, 190, 21, 180, 242, 227, 221, 181, 209, 207, 132, 175, 66, 95,
		245, 118, 180, 254, 71, 7, 62, 161, 220, 249, 178, 202, 94, 235, 195, 4, 136, 124, 81, 143,
		149, 110, 61, 243, 1, 92, 181, 103, 132, 100, 236, 95, 118, 84, 95, 42, 6, 46, 206, 163,
		182, 99, 179, 234, 144, 111, 85, 133, 103, 49, 43, 137, 85, 33, 73, 97, 110, 61, 90, 93,
		164, 85, 7, 239, 83, 210, 155, 39, 93, 202, 145, 39, 16, 210, 201, 24, 115, 37, 241, 187,
		111, 25, 19, 40, 146, 92, 32, 76, 197, 173, 227, 31, 53, 130, 14, 204, 186, 50, 64, 0, 110,
		147, 234, 19, 96, 243, 180, 145, 148, 62, 127, 126, 13, 177, 8, 113, 189, 202, 125, 99,
		121, 94, 26, 135, 212, 221, 165, 180, 244, 115, 44, 147, 150,
	],
];

pub(crate) const PRIVATE_TRANSFER: &[u8] = &[
	0, 0, 8, 13, 139, 46, 132, 205, 5, 70, 42, 29, 3, 4, 40, 86, 85, 136, 208, 53, 154, 150, 99,
	139, 174, 97, 124, 23, 210, 255, 202, 78, 186, 181, 53, 188, 85, 21, 31, 88, 42, 231, 54, 212,
	147, 124, 158, 241, 176, 100, 230, 151, 51, 69, 65, 74, 211, 221, 61, 224, 24, 81, 88, 245,
	106, 95, 69, 52, 172, 66, 52, 211, 169, 72, 135, 188, 5, 245, 81, 217, 166, 108, 14, 113, 230,
	68, 108, 236, 240, 204, 149, 61, 63, 44, 104, 181, 219, 79, 78, 94, 87, 222, 82, 238, 169, 78,
	169, 205, 123, 157, 66, 120, 136, 116, 223, 35, 103, 68, 146, 115, 109, 61, 192, 84, 151, 253,
	31, 91, 6, 212, 33, 8, 130, 26, 248, 67, 242, 80, 16, 194, 74, 16, 17, 233, 52, 193, 83, 111,
	166, 19, 188, 105, 78, 243, 155, 63, 239, 154, 71, 214, 29, 68, 110, 94, 197, 36, 5, 4, 124,
	138, 123, 201, 55, 170, 25, 247, 139, 211, 254, 54, 145, 63, 249, 43, 221, 51, 59, 119, 48,
	219, 172, 136, 89, 43, 129, 243, 203, 98, 53, 23, 101, 110, 168, 100, 180, 121, 216, 33, 85,
	156, 34, 20, 183, 96, 87, 182, 219, 142, 213, 16, 35, 222, 153, 30, 193, 74, 84, 241, 195, 49,
	213, 154, 233, 173, 54, 203, 145, 47, 95, 132, 143, 18, 227, 54, 218, 63, 76, 140, 91, 49, 185,
	161, 12, 120, 117, 124, 84, 191, 101, 165, 127, 19, 114, 53, 20, 239, 201, 32, 25, 186, 60, 16,
	19, 25, 235, 165, 132, 242, 243, 208, 124, 23, 99, 219, 118, 87, 23, 122, 229, 66, 232, 99,
	140, 63, 186, 114, 179, 120, 38, 77, 167, 113, 52, 83, 137, 226, 189, 253, 254, 119, 140, 197,
	215, 145, 99, 110, 154, 146, 193, 130, 62, 41, 14, 32, 47, 232, 234, 98, 94, 79, 184, 140, 0,
	174, 10, 25, 193, 170, 177, 167, 124, 192, 227, 18, 207, 20, 129, 252, 89, 163, 114, 161, 172,
	49, 134, 255, 174, 186, 38, 76, 165, 134, 17, 4, 132, 205, 165, 228, 62, 43, 18, 49, 15, 5, 84,
	217, 196, 197, 157, 61, 135, 77, 40, 101, 116, 13, 20, 38, 233, 137, 114, 132, 156, 176, 72,
	76, 208, 122, 72, 67, 108, 16, 188, 45, 238, 28, 184, 131, 63, 97, 93, 243, 239, 195, 196, 20,
	169, 51, 210, 127, 227, 196, 202, 103, 84, 182, 62, 18, 17, 27, 135, 231, 172, 38, 6, 97, 20,
	191, 89, 15, 26, 202, 19, 88, 141, 140, 89, 144, 167, 255, 108, 45, 109, 52, 28, 255, 46, 84,
	40, 42, 182, 182, 23, 125, 244, 211, 143, 69, 204, 182, 179, 133, 179, 40, 228, 70, 137, 211,
	183, 46, 213, 22, 186, 70, 150, 92, 21, 126, 20, 50, 30, 3, 245, 5, 167, 117, 146, 51, 187,
	124, 193, 185, 203, 104, 255, 109, 129, 59, 140, 83, 11, 157, 238, 39, 10, 143, 207, 238, 146,
	239, 156, 57, 215, 98, 7,
];

pub(crate) const RECLAIM_INPUT: &[&[u8]] = &[
	&[
		1, 8, 0, 0, 0, 4, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 233, 109, 90,
		102, 96, 156, 86, 27, 202, 210, 35, 113, 133, 183, 210, 195, 252, 117, 173, 255, 17, 85,
		87, 204, 135, 100, 97, 160, 222, 114, 217, 2, 186, 76, 204, 110, 220, 153, 176, 134, 97,
		195, 114, 80, 111, 112, 94, 142, 119, 64, 201, 231, 199, 9, 147, 228, 206, 177, 157, 142,
		161, 224, 244, 13, 71, 134, 253, 50, 46, 7, 131, 231, 236, 106, 199, 102, 4, 253, 66, 70,
		37, 14, 232, 225, 4, 64, 180, 88, 132, 179, 177, 196, 218, 11, 42, 129, 60, 9, 248, 39, 0,
		193, 209, 7, 217, 164, 162, 171, 192, 76, 168, 88, 213, 31, 246, 223, 118, 177, 44, 139,
		64, 64, 109, 83, 204, 124, 99, 134, 59, 142, 115, 146, 200, 137, 179, 0, 124, 144, 198,
		206, 66, 243, 109, 145, 169, 91, 196, 50, 134, 82, 231, 159, 204, 14, 176, 190, 119, 29,
		235, 33, 204, 222, 87, 110, 34, 58, 222, 218, 156, 195, 226, 225, 223, 26, 123, 253, 201,
		92, 55, 131, 36, 215, 212, 132, 128, 4, 93, 161, 38, 5, 243, 236, 92, 113, 88, 9, 8, 72,
		193, 194, 244, 217, 47, 83, 180, 128, 58, 226, 244, 43, 142, 211, 162, 28, 194, 143, 23,
		113, 162, 44, 168, 221, 110, 45, 247, 166, 159, 243, 253, 67, 225, 100, 216, 240, 219, 133,
		120, 192, 41, 103, 228, 0, 117, 49, 19, 180, 20, 123, 42, 240, 35, 78, 91, 112, 95, 151,
		162, 108, 207, 59, 55, 173, 70, 126, 13, 43, 173, 20, 14, 98, 42, 106, 138, 9, 123, 164,
		29, 49, 161, 190, 24, 80, 64, 49, 9, 64, 170, 162, 113, 251, 17, 185, 128,
	],
	&[
		1, 8, 0, 0, 0, 4, 32, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 47, 78, 188, 215,
		159, 112, 40, 219, 240, 133, 16, 121, 255, 188, 224, 172, 69, 38, 149, 161, 161, 5, 198,
		33, 64, 255, 0, 28, 182, 46, 120, 96, 20, 207, 115, 38, 45, 24, 97, 120, 88, 95, 117, 89,
		49, 139, 230, 254, 54, 71, 230, 134, 23, 179, 138, 176, 143, 231, 58, 151, 8, 55, 204, 62,
		23, 39, 245, 105, 108, 177, 172, 121, 232, 134, 9, 17, 247, 46, 22, 206, 154, 107, 194, 47,
		177, 68, 142, 219, 73, 239, 74, 145, 104, 210, 15, 16, 157, 94, 128, 132, 0, 26, 6, 41,
		168, 216, 128, 35, 102, 87, 77, 178, 180, 236, 167, 182, 254, 236, 96, 162, 12, 152, 70, 0,
		60, 122, 44, 201, 13, 70, 60, 185, 245, 123, 29, 62, 54, 190, 96, 4, 152, 71, 212, 86, 21,
		147, 182, 231, 6, 220, 97, 139, 248, 36, 128, 205, 77, 218, 48, 92, 75, 121, 91, 38, 213,
		250, 210, 55, 155, 89, 85, 51, 112, 243, 254, 18, 38, 202, 10, 81, 87, 42, 205, 164, 52,
		25, 16, 78, 125, 142, 179, 42, 229, 198, 55, 139, 19, 6, 171, 243, 220, 153, 231, 50, 133,
		46, 0, 16, 192, 74, 19, 53, 84, 79, 22, 170, 203, 251, 163, 32, 244, 244, 230, 54, 10, 239,
		181, 81, 221, 181, 191, 51, 36, 4, 215, 109, 192, 62, 148, 40, 199, 147, 14, 32, 10, 26,
		48, 139, 26, 7, 243, 86, 55, 116, 84, 42, 20, 46, 53, 156, 98, 202, 56, 207, 157, 245, 236,
		43, 83, 227, 232, 152, 219, 214, 38, 26, 129, 181, 42, 61, 128, 127, 153, 227, 33, 166, 9,
		64, 243, 71, 22, 49, 147,
	],
];

pub(crate) const RECLAIM: &[u8] = &[
	1, 8, 0, 0, 0, 0, 8, 26, 231, 209, 123, 75, 35, 238, 255, 11, 131, 161, 227, 162, 232, 15, 108,
	141, 174, 251, 128, 219, 142, 210, 113, 148, 219, 123, 216, 105, 115, 60, 43, 204, 145, 255,
	139, 215, 216, 235, 189, 154, 124, 81, 172, 165, 126, 3, 100, 54, 43, 73, 94, 180, 160, 229,
	239, 91, 75, 151, 172, 167, 34, 167, 28, 117, 91, 204, 239, 94, 215, 143, 213, 121, 14, 88,
	120, 250, 143, 106, 100, 29, 207, 93, 233, 6, 25, 163, 44, 33, 0, 216, 12, 134, 116, 172, 62,
	124, 228, 122, 161, 33, 62, 163, 88, 166, 210, 70, 38, 233, 173, 118, 93, 127, 38, 112, 225,
	105, 245, 238, 62, 52, 31, 53, 98, 63, 176, 184, 83, 4, 7, 31, 44, 255, 214, 63, 123, 181, 58,
	195, 185, 140, 21, 221, 55, 84, 23, 128, 226, 109, 12, 207, 141, 71, 216, 23, 241, 94, 112,
	168, 120, 39, 110, 251, 136, 201, 60, 192, 3, 104, 216, 61, 29, 52, 54, 84, 35, 184, 16, 191,
	248, 83, 8, 243, 230, 31, 219, 45, 76, 108, 186, 173, 245, 234, 134, 35, 233, 42, 193, 123,
	135, 35, 247, 212, 40, 25, 71, 237, 211, 109, 127, 43, 67, 90, 135, 10, 120, 232, 155, 74, 28,
	104, 133, 181, 127, 94, 7, 191, 22, 26, 4, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	106, 215, 19, 93, 200, 168, 244, 203, 23, 109, 135, 210, 244, 82, 227, 217, 212, 146, 105, 199,
	119, 194, 190, 152, 110, 82, 211, 214, 103, 152, 141, 120, 209, 32, 215, 87, 69, 177, 245, 66,
	226, 20, 224, 219, 110, 114, 90, 130, 221, 84, 49, 27, 240, 92, 198, 80, 165, 69, 174, 232, 72,
	90, 121, 198, 92, 240, 166, 90, 48, 214, 199, 20, 89, 235, 123, 239, 80, 172, 141, 91, 69, 219,
	178, 150, 250, 47, 17, 193, 196, 217, 142, 144, 125, 150, 204, 5, 221, 22, 26, 95, 107, 225,
	145, 233, 59, 9, 93, 104, 155, 18, 136, 6, 199, 113, 111, 110, 14, 213, 216, 243, 54, 66, 113,
	230, 26, 39, 150, 144, 247, 144, 101, 170, 64, 244, 101, 230, 94, 166, 160, 94, 218, 198, 38,
	140, 144, 20, 39, 184, 58, 9, 253, 3, 103, 9, 125, 205, 123, 4, 105, 15, 157, 190, 126, 160,
	67, 178, 55, 9, 81, 136, 170, 36, 162, 104, 28, 149, 243, 194, 152, 184, 101, 191, 48, 22, 225,
	108, 136, 241, 231, 107, 137, 5,
];