#[inline(never)] pub fn fuzzable_function(input: &[u8; 128], covered_code_blocks: &mut [u64; 560]) -> bool {
    let mut new_block_reached = false;
    if covered_code_blocks[0] == 0 { new_block_reached = true; }
    covered_code_blocks[0] = 1;
        if input[122] & 0b00111100 == 0b00011100 {
        if covered_code_blocks[1] == 0 { new_block_reached = true; }
        covered_code_blocks[1] = 1;
            if input[62] & 0b11100000 == 0b00100000 {
            if covered_code_blocks[2] == 0 { new_block_reached = true; }
            covered_code_blocks[2] = 1;
        }
    }
        if input[47] & 0b11111110 == 0b01101100 {
        if covered_code_blocks[3] == 0 { new_block_reached = true; }
        covered_code_blocks[3] = 1;
    }
        if input[121] & 0b00010000 == 0b00010000 {
        if covered_code_blocks[4] == 0 { new_block_reached = true; }
        covered_code_blocks[4] = 1;
            if input[41] & 0b00011111 == 0b00010000 {
            if covered_code_blocks[5] == 0 { new_block_reached = true; }
            covered_code_blocks[5] = 1;
                if input[95] & 0b01111110 == 0b00111100 {
                if covered_code_blocks[6] == 0 { new_block_reached = true; }
                covered_code_blocks[6] = 1;
                    if input[18] & 0b00111111 == 0b00100010 {
                    if covered_code_blocks[7] == 0 { new_block_reached = true; }
                    covered_code_blocks[7] = 1;
                }
                    if input[41] & 0b00100000 == 0b00100000 {
                    if covered_code_blocks[8] == 0 { new_block_reached = true; }
                    covered_code_blocks[8] = 1;
                }
            }
                if input[52] & 0b11110000 == 0b10110000 {
                if covered_code_blocks[9] == 0 { new_block_reached = true; }
                covered_code_blocks[9] = 1;
                    if input[100] & 0b01100000 == 0b01000000 {
                    if covered_code_blocks[10] == 0 { new_block_reached = true; }
                    covered_code_blocks[10] = 1;
                }
                    if input[28] & 0b00011100 == 0b00010100 {
                    if covered_code_blocks[11] == 0 { new_block_reached = true; }
                    covered_code_blocks[11] = 1;
                }
                    if input[63] & 0b00111000 == 0b00100000 {
                    if covered_code_blocks[12] == 0 { new_block_reached = true; }
                    covered_code_blocks[12] = 1;
                        if input[51] & 0b00000010 == 0b00000000 {
                        if covered_code_blocks[13] == 0 { new_block_reached = true; }
                        covered_code_blocks[13] = 1;
                    }
                        if input[74] & 0b00000110 == 0b00000010 {
                        if covered_code_blocks[14] == 0 { new_block_reached = true; }
                        covered_code_blocks[14] = 1;
                            if input[77] & 0b11110000 == 0b10110000 {
                            if covered_code_blocks[15] == 0 { new_block_reached = true; }
                            covered_code_blocks[15] = 1;
                        }
                            if input[30] & 0b11111111 == 0b10100101 {
                            if covered_code_blocks[16] == 0 { new_block_reached = true; }
                            covered_code_blocks[16] = 1;
                        }
                    }
                        if input[120] & 0b00011100 == 0b00000100 {
                        if covered_code_blocks[17] == 0 { new_block_reached = true; }
                        covered_code_blocks[17] = 1;
                            if input[56] & 0b01110000 == 0b00110000 {
                            if covered_code_blocks[18] == 0 { new_block_reached = true; }
                            covered_code_blocks[18] = 1;
                        }
                            if input[118] & 0b00011000 == 0b00000000 {
                            if covered_code_blocks[19] == 0 { new_block_reached = true; }
                            covered_code_blocks[19] = 1;
                                if input[113] & 0b11000000 == 0b10000000 {
                                if covered_code_blocks[20] == 0 { new_block_reached = true; }
                                covered_code_blocks[20] = 1;
                            }
                                if input[75] & 0b00010000 == 0b00010000 {
                                if covered_code_blocks[21] == 0 { new_block_reached = true; }
                                covered_code_blocks[21] = 1;
                                    if input[16] & 0b00111110 == 0b00101000 {
                                    if covered_code_blocks[22] == 0 { new_block_reached = true; }
                                    covered_code_blocks[22] = 1;
                                        if input[96] & 0b00011100 == 0b00001000 {
                                        if covered_code_blocks[23] == 0 { new_block_reached = true; }
                                        covered_code_blocks[23] = 1;
                                    }
                                        if input[56] & 0b00001110 == 0b00000100 {
                                        if covered_code_blocks[24] == 0 { new_block_reached = true; }
                                        covered_code_blocks[24] = 1;
                                    }
                                        if input[10] & 0b00001110 == 0b00001110 {
                                        if covered_code_blocks[25] == 0 { new_block_reached = true; }
                                        covered_code_blocks[25] = 1;
                                            if input[2] & 0b11111111 == 0b10101101 {
                                            if covered_code_blocks[26] == 0 { new_block_reached = true; }
                                            covered_code_blocks[26] = 1;
                                        }
                                            if input[83] & 0b01111110 == 0b01010010 {
                                            if covered_code_blocks[27] == 0 { new_block_reached = true; }
                                            covered_code_blocks[27] = 1;
                                        }
                                            if input[22] & 0b11111110 == 0b11010100 {
                                            if covered_code_blocks[28] == 0 { new_block_reached = true; }
                                            covered_code_blocks[28] = 1;
                                        }
                                            if input[17] & 0b00111111 == 0b00001011 {
                                            if covered_code_blocks[29] == 0 { new_block_reached = true; }
                                            covered_code_blocks[29] = 1;
                                                if input[55] & 0b11110000 == 0b00010000 {
                                                if covered_code_blocks[30] == 0 { new_block_reached = true; }
                                                covered_code_blocks[30] = 1;
                                                    if input[104] & 0b11000000 == 0b01000000 {
                                                    if covered_code_blocks[31] == 0 { new_block_reached = true; }
                                                    covered_code_blocks[31] = 1;
                                                        if input[33] & 0b00001111 == 0b00000001 {
                                                        if covered_code_blocks[32] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[32] = 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                    if input[106] & 0b11000000 == 0b01000000 {
                                    if covered_code_blocks[33] == 0 { new_block_reached = true; }
                                    covered_code_blocks[33] = 1;
                                        if input[66] & 0b01100000 == 0b00100000 {
                                        if covered_code_blocks[34] == 0 { new_block_reached = true; }
                                        covered_code_blocks[34] = 1;
                                    }
                                }
                            }
                                if input[67] & 0b00011000 == 0b00001000 {
                                if covered_code_blocks[35] == 0 { new_block_reached = true; }
                                covered_code_blocks[35] = 1;
                            }
                                if input[124] & 0b00001111 == 0b00001100 {
                                if covered_code_blocks[36] == 0 { new_block_reached = true; }
                                covered_code_blocks[36] = 1;
                            }
                                if input[25] & 0b00100000 == 0b00000000 {
                                if covered_code_blocks[37] == 0 { new_block_reached = true; }
                                covered_code_blocks[37] = 1;
                            }
                        }
                    }
                }
            }
        }
            if input[35] & 0b00001110 == 0b00000000 {
            if covered_code_blocks[38] == 0 { new_block_reached = true; }
            covered_code_blocks[38] = 1;
                if input[93] & 0b00000110 == 0b00000110 {
                if covered_code_blocks[39] == 0 { new_block_reached = true; }
                covered_code_blocks[39] = 1;
                    if input[92] & 0b01000000 == 0b00000000 {
                    if covered_code_blocks[40] == 0 { new_block_reached = true; }
                    covered_code_blocks[40] = 1;
                }
                    if input[21] & 0b00001110 == 0b00000010 {
                    if covered_code_blocks[41] == 0 { new_block_reached = true; }
                    covered_code_blocks[41] = 1;
                }
            }
                if input[101] & 0b01111111 == 0b00001001 {
                if covered_code_blocks[42] == 0 { new_block_reached = true; }
                covered_code_blocks[42] = 1;
                    if input[54] & 0b11111111 == 0b00011101 {
                    if covered_code_blocks[43] == 0 { new_block_reached = true; }
                    covered_code_blocks[43] = 1;
                }
            }
                if input[80] & 0b00001100 == 0b00000000 {
                if covered_code_blocks[44] == 0 { new_block_reached = true; }
                covered_code_blocks[44] = 1;
                    if input[103] & 0b00000100 == 0b00000000 {
                    if covered_code_blocks[45] == 0 { new_block_reached = true; }
                    covered_code_blocks[45] = 1;
                        if input[116] & 0b00001111 == 0b00001110 {
                        if covered_code_blocks[46] == 0 { new_block_reached = true; }
                        covered_code_blocks[46] = 1;
                    }
                        if input[15] & 0b00100000 == 0b00100000 {
                        if covered_code_blocks[47] == 0 { new_block_reached = true; }
                        covered_code_blocks[47] = 1;
                            if input[13] & 0b00000111 == 0b00000111 {
                            if covered_code_blocks[48] == 0 { new_block_reached = true; }
                            covered_code_blocks[48] = 1;
                                if input[71] & 0b11110000 == 0b10000000 {
                                if covered_code_blocks[49] == 0 { new_block_reached = true; }
                                covered_code_blocks[49] = 1;
                            }
                                if input[84] & 0b11111100 == 0b01010100 {
                                if covered_code_blocks[50] == 0 { new_block_reached = true; }
                                covered_code_blocks[50] = 1;
                                    if input[20] & 0b00011000 == 0b00000000 {
                                    if covered_code_blocks[51] == 0 { new_block_reached = true; }
                                    covered_code_blocks[51] = 1;
                                        if input[90] & 0b11000000 == 0b01000000 {
                                        if covered_code_blocks[52] == 0 { new_block_reached = true; }
                                        covered_code_blocks[52] = 1;
                                            if input[127] & 0b11111100 == 0b10100000 {
                                            if covered_code_blocks[53] == 0 { new_block_reached = true; }
                                            covered_code_blocks[53] = 1;
                                        }
                                            if input[111] & 0b01000000 == 0b01000000 {
                                            if covered_code_blocks[54] == 0 { new_block_reached = true; }
                                            covered_code_blocks[54] = 1;
                                        }
                                            if input[33] & 0b01110000 == 0b00100000 {
                                            if covered_code_blocks[55] == 0 { new_block_reached = true; }
                                            covered_code_blocks[55] = 1;
                                        }
                                            if input[23] & 0b00011100 == 0b00011100 {
                                            if covered_code_blocks[56] == 0 { new_block_reached = true; }
                                            covered_code_blocks[56] = 1;
                                        }
                                            if input[37] & 0b10000000 == 0b00000000 {
                                            if covered_code_blocks[57] == 0 { new_block_reached = true; }
                                            covered_code_blocks[57] = 1;
                                        }
                                    }
                                }
                                    if input[81] & 0b00001111 == 0b00000111 {
                                    if covered_code_blocks[58] == 0 { new_block_reached = true; }
                                    covered_code_blocks[58] = 1;
                                }
                            }
                                if input[86] & 0b00001000 == 0b00001000 {
                                if covered_code_blocks[59] == 0 { new_block_reached = true; }
                                covered_code_blocks[59] = 1;
                            }
                        }
                            if input[75] & 0b00001110 == 0b00000010 {
                            if covered_code_blocks[60] == 0 { new_block_reached = true; }
                            covered_code_blocks[60] = 1;
                        }
                    }
                        if input[92] & 0b00100000 == 0b00000000 {
                        if covered_code_blocks[61] == 0 { new_block_reached = true; }
                        covered_code_blocks[61] = 1;
                    }
                        if input[29] & 0b00001110 == 0b00000100 {
                        if covered_code_blocks[62] == 0 { new_block_reached = true; }
                        covered_code_blocks[62] = 1;
                    }
                        if input[113] & 0b00110000 == 0b00110000 {
                        if covered_code_blocks[63] == 0 { new_block_reached = true; }
                        covered_code_blocks[63] = 1;
                    }
                        if input[26] & 0b11000000 == 0b11000000 {
                        if covered_code_blocks[64] == 0 { new_block_reached = true; }
                        covered_code_blocks[64] = 1;
                    }
                        if input[127] & 0b00000001 == 0b00000000 {
                        if covered_code_blocks[65] == 0 { new_block_reached = true; }
                        covered_code_blocks[65] = 1;
                    }
                        if input[125] & 0b00000110 == 0b00000110 {
                        if covered_code_blocks[66] == 0 { new_block_reached = true; }
                        covered_code_blocks[66] = 1;
                            if input[1] & 0b11111110 == 0b01100110 {
                            if covered_code_blocks[67] == 0 { new_block_reached = true; }
                            covered_code_blocks[67] = 1;
                                if input[59] & 0b00011111 == 0b00000011 {
                                if covered_code_blocks[68] == 0 { new_block_reached = true; }
                                covered_code_blocks[68] = 1;
                            }
                                if input[44] & 0b00000100 == 0b00000000 {
                                if covered_code_blocks[69] == 0 { new_block_reached = true; }
                                covered_code_blocks[69] = 1;
                                    if input[108] & 0b10000000 == 0b10000000 {
                                    if covered_code_blocks[70] == 0 { new_block_reached = true; }
                                    covered_code_blocks[70] = 1;
                                }
                            }
                                if input[82] & 0b00000100 == 0b00000000 {
                                if covered_code_blocks[71] == 0 { new_block_reached = true; }
                                covered_code_blocks[71] = 1;
                                    if input[29] & 0b00010000 == 0b00010000 {
                                    if covered_code_blocks[72] == 0 { new_block_reached = true; }
                                    covered_code_blocks[72] = 1;
                                }
                            }
                                if input[17] & 0b01000000 == 0b00000000 {
                                if covered_code_blocks[73] == 0 { new_block_reached = true; }
                                covered_code_blocks[73] = 1;
                            }
                                if input[64] & 0b11111100 == 0b00110100 {
                                if covered_code_blocks[74] == 0 { new_block_reached = true; }
                                covered_code_blocks[74] = 1;
                            }
                                if input[45] & 0b00111110 == 0b00000010 {
                                if covered_code_blocks[75] == 0 { new_block_reached = true; }
                                covered_code_blocks[75] = 1;
                            }
                        }
                    }
                        if input[53] & 0b00010000 == 0b00000000 {
                        if covered_code_blocks[76] == 0 { new_block_reached = true; }
                        covered_code_blocks[76] = 1;
                            if input[11] & 0b01111111 == 0b01001000 {
                            if covered_code_blocks[77] == 0 { new_block_reached = true; }
                            covered_code_blocks[77] = 1;
                        }
                            if input[43] & 0b00100000 == 0b00100000 {
                            if covered_code_blocks[78] == 0 { new_block_reached = true; }
                            covered_code_blocks[78] = 1;
                                if input[31] & 0b00111000 == 0b00011000 {
                                if covered_code_blocks[79] == 0 { new_block_reached = true; }
                                covered_code_blocks[79] = 1;
                                    if input[102] & 0b00110000 == 0b00010000 {
                                    if covered_code_blocks[80] == 0 { new_block_reached = true; }
                                    covered_code_blocks[80] = 1;
                                        if input[110] & 0b00011110 == 0b00001110 {
                                        if covered_code_blocks[81] == 0 { new_block_reached = true; }
                                        covered_code_blocks[81] = 1;
                                            if input[105] & 0b00001000 == 0b00001000 {
                                            if covered_code_blocks[82] == 0 { new_block_reached = true; }
                                            covered_code_blocks[82] = 1;
                                        }
                                            if input[59] & 0b00100000 == 0b00100000 {
                                            if covered_code_blocks[83] == 0 { new_block_reached = true; }
                                            covered_code_blocks[83] = 1;
                                                if input[126] & 0b01111000 == 0b00110000 {
                                                if covered_code_blocks[84] == 0 { new_block_reached = true; }
                                                covered_code_blocks[84] = 1;
                                            }
                                        }
                                            if input[0] & 0b00000001 == 0b00000000 {
                                            if covered_code_blocks[85] == 0 { new_block_reached = true; }
                                            covered_code_blocks[85] = 1;
                                        }
                                            if input[88] & 0b11100000 == 0b11100000 {
                                            if covered_code_blocks[86] == 0 { new_block_reached = true; }
                                            covered_code_blocks[86] = 1;
                                                if input[90] & 0b00111100 == 0b00011000 {
                                                if covered_code_blocks[87] == 0 { new_block_reached = true; }
                                                covered_code_blocks[87] = 1;
                                            }
                                        }
                                    }
                                        if input[38] & 0b00000111 == 0b00000011 {
                                        if covered_code_blocks[88] == 0 { new_block_reached = true; }
                                        covered_code_blocks[88] = 1;
                                            if input[14] & 0b00100000 == 0b00000000 {
                                            if covered_code_blocks[89] == 0 { new_block_reached = true; }
                                            covered_code_blocks[89] = 1;
                                                if input[44] & 0b00000001 == 0b00000001 {
                                                if covered_code_blocks[90] == 0 { new_block_reached = true; }
                                                covered_code_blocks[90] = 1;
                                            }
                                        }
                                            if input[20] & 0b00000111 == 0b00000010 {
                                            if covered_code_blocks[91] == 0 { new_block_reached = true; }
                                            covered_code_blocks[91] = 1;
                                                if input[39] & 0b01111110 == 0b01111110 {
                                                if covered_code_blocks[92] == 0 { new_block_reached = true; }
                                                covered_code_blocks[92] = 1;
                                            }
                                                if input[12] & 0b11111000 == 0b00000000 {
                                                if covered_code_blocks[93] == 0 { new_block_reached = true; }
                                                covered_code_blocks[93] = 1;
                                                    if input[99] & 0b00111110 == 0b00001110 {
                                                    if covered_code_blocks[94] == 0 { new_block_reached = true; }
                                                    covered_code_blocks[94] = 1;
                                                        if input[48] & 0b11100000 == 0b01100000 {
                                                        if covered_code_blocks[95] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[95] = 1;
                                                    }
                                                        if input[0] & 0b00010000 == 0b00000000 {
                                                        if covered_code_blocks[96] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[96] = 1;
                                                            if input[89] & 0b00001110 == 0b00001010 {
                                                            if covered_code_blocks[97] == 0 { new_block_reached = true; }
                                                            covered_code_blocks[97] = 1;
                                                                if input[55] & 0b00001100 == 0b00000100 {
                                                                if covered_code_blocks[98] == 0 { new_block_reached = true; }
                                                                covered_code_blocks[98] = 1;
                                                            }
                                                        }
                                                            if input[97] & 0b11110000 == 0b00000000 {
                                                            if covered_code_blocks[99] == 0 { new_block_reached = true; }
                                                            covered_code_blocks[99] = 1;
                                                                if input[61] & 0b00111100 == 0b00010100 {
                                                                if covered_code_blocks[100] == 0 { new_block_reached = true; }
                                                                covered_code_blocks[100] = 1;
                                                                    if input[31] & 0b00000011 == 0b00000001 {
                                                                    if covered_code_blocks[101] == 0 { new_block_reached = true; }
                                                                    covered_code_blocks[101] = 1;
                                                                }
                                                                    if input[36] & 0b11111100 == 0b11100100 {
                                                                    if covered_code_blocks[102] == 0 { new_block_reached = true; }
                                                                    covered_code_blocks[102] = 1;
                                                                }
                                                                    if input[65] & 0b01111100 == 0b01000100 {
                                                                    if covered_code_blocks[103] == 0 { new_block_reached = true; }
                                                                    covered_code_blocks[103] = 1;
                                                                        if input[107] & 0b01111110 == 0b01101000 {
                                                                        if covered_code_blocks[104] == 0 { new_block_reached = true; }
                                                                        covered_code_blocks[104] = 1;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                            if input[69] & 0b00000110 == 0b00000010 {
                                                            if covered_code_blocks[105] == 0 { new_block_reached = true; }
                                                            covered_code_blocks[105] = 1;
                                                        }
                                                    }
                                                        if input[113] & 0b00000011 == 0b00000001 {
                                                        if covered_code_blocks[106] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[106] = 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                    if input[114] & 0b11100000 == 0b01100000 {
                                    if covered_code_blocks[107] == 0 { new_block_reached = true; }
                                    covered_code_blocks[107] = 1;
                                }
                            }
                                if input[121] & 0b00000010 == 0b00000010 {
                                if covered_code_blocks[108] == 0 { new_block_reached = true; }
                                covered_code_blocks[108] = 1;
                                    if input[3] & 0b00001100 == 0b00001100 {
                                    if covered_code_blocks[109] == 0 { new_block_reached = true; }
                                    covered_code_blocks[109] = 1;
                                }
                            }
                                if input[8] & 0b00000110 == 0b00000110 {
                                if covered_code_blocks[110] == 0 { new_block_reached = true; }
                                covered_code_blocks[110] = 1;
                                    if input[57] & 0b00001100 == 0b00001000 {
                                    if covered_code_blocks[111] == 0 { new_block_reached = true; }
                                    covered_code_blocks[111] = 1;
                                        if input[91] & 0b00110000 == 0b00010000 {
                                        if covered_code_blocks[112] == 0 { new_block_reached = true; }
                                        covered_code_blocks[112] = 1;
                                            if input[74] & 0b11100000 == 0b00000000 {
                                            if covered_code_blocks[113] == 0 { new_block_reached = true; }
                                            covered_code_blocks[113] = 1;
                                        }
                                    }
                                        if input[92] & 0b00001111 == 0b00001011 {
                                        if covered_code_blocks[114] == 0 { new_block_reached = true; }
                                        covered_code_blocks[114] = 1;
                                    }
                                }
                                    if input[6] & 0b01000000 == 0b00000000 {
                                    if covered_code_blocks[115] == 0 { new_block_reached = true; }
                                    covered_code_blocks[115] = 1;
                                        if input[34] & 0b00011000 == 0b00010000 {
                                        if covered_code_blocks[116] == 0 { new_block_reached = true; }
                                        covered_code_blocks[116] = 1;
                                    }
                                        if input[73] & 0b01111111 == 0b01000111 {
                                        if covered_code_blocks[117] == 0 { new_block_reached = true; }
                                        covered_code_blocks[117] = 1;
                                            if input[51] & 0b01111100 == 0b01110100 {
                                            if covered_code_blocks[118] == 0 { new_block_reached = true; }
                                            covered_code_blocks[118] = 1;
                                        }
                                    }
                                        if input[121] & 0b00100000 == 0b00000000 {
                                        if covered_code_blocks[119] == 0 { new_block_reached = true; }
                                        covered_code_blocks[119] = 1;
                                    }
                                        if input[7] & 0b01110000 == 0b01110000 {
                                        if covered_code_blocks[120] == 0 { new_block_reached = true; }
                                        covered_code_blocks[120] = 1;
                                    }
                                        if input[19] & 0b01000000 == 0b01000000 {
                                        if covered_code_blocks[121] == 0 { new_block_reached = true; }
                                        covered_code_blocks[121] = 1;
                                            if input[5] & 0b11111000 == 0b10110000 {
                                            if covered_code_blocks[122] == 0 { new_block_reached = true; }
                                            covered_code_blocks[122] = 1;
                                        }
                                    }
                                }
                                    if input[53] & 0b10000000 == 0b10000000 {
                                    if covered_code_blocks[123] == 0 { new_block_reached = true; }
                                    covered_code_blocks[123] = 1;
                                }
                                    if input[124] & 0b01000000 == 0b00000000 {
                                    if covered_code_blocks[124] == 0 { new_block_reached = true; }
                                    covered_code_blocks[124] = 1;
                                }
                            }
                                if input[50] & 0b00111111 == 0b00100011 {
                                if covered_code_blocks[125] == 0 { new_block_reached = true; }
                                covered_code_blocks[125] = 1;
                                    if input[0] & 0b00000010 == 0b00000000 {
                                    if covered_code_blocks[126] == 0 { new_block_reached = true; }
                                    covered_code_blocks[126] = 1;
                                }
                            }
                        }
                    }
                }
                    if input[76] & 0b00001110 == 0b00000000 {
                    if covered_code_blocks[127] == 0 { new_block_reached = true; }
                    covered_code_blocks[127] = 1;
                }
            }
                if input[105] & 0b01100000 == 0b00100000 {
                if covered_code_blocks[128] == 0 { new_block_reached = true; }
                covered_code_blocks[128] = 1;
            }
        }
    }
        if input[110] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[129] == 0 { new_block_reached = true; }
        covered_code_blocks[129] = 1;
            if input[4] & 0b00000100 == 0b00000100 {
            if covered_code_blocks[130] == 0 { new_block_reached = true; }
            covered_code_blocks[130] = 1;
        }
            if input[44] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[131] == 0 { new_block_reached = true; }
            covered_code_blocks[131] = 1;
        }
    }
        if input[55] & 0b00000010 == 0b00000000 {
        if covered_code_blocks[132] == 0 { new_block_reached = true; }
        covered_code_blocks[132] = 1;
            if input[3] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[133] == 0 { new_block_reached = true; }
            covered_code_blocks[133] = 1;
        }
    }
        if input[50] & 0b01000000 == 0b00000000 {
        if covered_code_blocks[134] == 0 { new_block_reached = true; }
        covered_code_blocks[134] = 1;
    }
        if input[58] & 0b00001111 == 0b00001100 {
        if covered_code_blocks[135] == 0 { new_block_reached = true; }
        covered_code_blocks[135] = 1;
    }
        if input[93] & 0b11111000 == 0b00111000 {
        if covered_code_blocks[136] == 0 { new_block_reached = true; }
        covered_code_blocks[136] = 1;
            if input[40] & 0b00000100 == 0b00000000 {
            if covered_code_blocks[137] == 0 { new_block_reached = true; }
            covered_code_blocks[137] = 1;
                if input[91] & 0b10000000 == 0b00000000 {
                if covered_code_blocks[138] == 0 { new_block_reached = true; }
                covered_code_blocks[138] = 1;
                    if input[111] & 0b00001110 == 0b00001100 {
                    if covered_code_blocks[139] == 0 { new_block_reached = true; }
                    covered_code_blocks[139] = 1;
                }
            }
        }
    }
        if input[42] & 0b00001110 == 0b00001110 {
        if covered_code_blocks[140] == 0 { new_block_reached = true; }
        covered_code_blocks[140] = 1;
            if input[102] & 0b00000100 == 0b00000000 {
            if covered_code_blocks[141] == 0 { new_block_reached = true; }
            covered_code_blocks[141] = 1;
                if input[82] & 0b00000011 == 0b00000010 {
                if covered_code_blocks[142] == 0 { new_block_reached = true; }
                covered_code_blocks[142] = 1;
                    if input[12] & 0b00000001 == 0b00000001 {
                    if covered_code_blocks[143] == 0 { new_block_reached = true; }
                    covered_code_blocks[143] = 1;
                }
                    if input[86] & 0b00000111 == 0b00000010 {
                    if covered_code_blocks[144] == 0 { new_block_reached = true; }
                    covered_code_blocks[144] = 1;
                        if input[119] & 0b00000110 == 0b00000110 {
                        if covered_code_blocks[145] == 0 { new_block_reached = true; }
                        covered_code_blocks[145] = 1;
                    }
                        if input[115] & 0b01110000 == 0b01100000 {
                        if covered_code_blocks[146] == 0 { new_block_reached = true; }
                        covered_code_blocks[146] = 1;
                            if input[108] & 0b01111100 == 0b00011100 {
                            if covered_code_blocks[147] == 0 { new_block_reached = true; }
                            covered_code_blocks[147] = 1;
                                if input[88] & 0b00000111 == 0b00000011 {
                                if covered_code_blocks[148] == 0 { new_block_reached = true; }
                                covered_code_blocks[148] = 1;
                            }
                        }
                    }
                }
            }
                if input[9] & 0b01111111 == 0b00010111 {
                if covered_code_blocks[149] == 0 { new_block_reached = true; }
                covered_code_blocks[149] = 1;
                    if input[122] & 0b00000001 == 0b00000001 {
                    if covered_code_blocks[150] == 0 { new_block_reached = true; }
                    covered_code_blocks[150] = 1;
                        if input[117] & 0b00000110 == 0b00000000 {
                        if covered_code_blocks[151] == 0 { new_block_reached = true; }
                        covered_code_blocks[151] = 1;
                    }
                }
            }
        }
            if input[70] & 0b00100000 == 0b00000000 {
            if covered_code_blocks[152] == 0 { new_block_reached = true; }
            covered_code_blocks[152] = 1;
        }
    }
        if input[98] & 0b00000001 == 0b00000000 {
        if covered_code_blocks[153] == 0 { new_block_reached = true; }
        covered_code_blocks[153] = 1;
            if input[78] & 0b00010000 == 0b00010000 {
            if covered_code_blocks[154] == 0 { new_block_reached = true; }
            covered_code_blocks[154] = 1;
        }
    }
        if input[79] & 0b01111000 == 0b01000000 {
        if covered_code_blocks[155] == 0 { new_block_reached = true; }
        covered_code_blocks[155] = 1;
    }
        if input[68] & 0b00000001 == 0b00000001 {
        if covered_code_blocks[156] == 0 { new_block_reached = true; }
        covered_code_blocks[156] = 1;
    }
        if input[80] & 0b01110000 == 0b01000000 {
        if covered_code_blocks[157] == 0 { new_block_reached = true; }
        covered_code_blocks[157] = 1;
    }
        if input[37] & 0b00100000 == 0b00000000 {
        if covered_code_blocks[158] == 0 { new_block_reached = true; }
        covered_code_blocks[158] = 1;
            if input[89] & 0b01110000 == 0b00000000 {
            if covered_code_blocks[159] == 0 { new_block_reached = true; }
            covered_code_blocks[159] = 1;
        }
    }
        if input[123] & 0b00000010 == 0b00000000 {
        if covered_code_blocks[160] == 0 { new_block_reached = true; }
        covered_code_blocks[160] = 1;
            if input[115] & 0b00000001 == 0b00000001 {
            if covered_code_blocks[161] == 0 { new_block_reached = true; }
            covered_code_blocks[161] = 1;
        }
            if input[60] & 0b00100000 == 0b00000000 {
            if covered_code_blocks[162] == 0 { new_block_reached = true; }
            covered_code_blocks[162] = 1;
                if input[67] & 0b10000000 == 0b00000000 {
                if covered_code_blocks[163] == 0 { new_block_reached = true; }
                covered_code_blocks[163] = 1;
            }
                if input[49] & 0b01111000 == 0b00000000 {
                if covered_code_blocks[164] == 0 { new_block_reached = true; }
                covered_code_blocks[164] = 1;
                    if input[46] & 0b00100000 == 0b00000000 {
                    if covered_code_blocks[165] == 0 { new_block_reached = true; }
                    covered_code_blocks[165] = 1;
                        if input[4] & 0b00110000 == 0b00100000 {
                        if covered_code_blocks[166] == 0 { new_block_reached = true; }
                        covered_code_blocks[166] = 1;
                    }
                }
                    if input[87] & 0b00111100 == 0b00111100 {
                    if covered_code_blocks[167] == 0 { new_block_reached = true; }
                    covered_code_blocks[167] = 1;
                        if input[114] & 0b00010000 == 0b00000000 {
                        if covered_code_blocks[168] == 0 { new_block_reached = true; }
                        covered_code_blocks[168] = 1;
                            if input[53] & 0b01000000 == 0b00000000 {
                            if covered_code_blocks[169] == 0 { new_block_reached = true; }
                            covered_code_blocks[169] = 1;
                        }
                    }
                        if input[72] & 0b11111110 == 0b11011100 {
                        if covered_code_blocks[170] == 0 { new_block_reached = true; }
                        covered_code_blocks[170] = 1;
                            if input[57] & 0b11000000 == 0b11000000 {
                            if covered_code_blocks[171] == 0 { new_block_reached = true; }
                            covered_code_blocks[171] = 1;
                                if input[18] & 0b10000000 == 0b00000000 {
                                if covered_code_blocks[172] == 0 { new_block_reached = true; }
                                covered_code_blocks[172] = 1;
                                    if input[117] & 0b11100000 == 0b01000000 {
                                    if covered_code_blocks[173] == 0 { new_block_reached = true; }
                                    covered_code_blocks[173] = 1;
                                }
                                    if input[123] & 0b00110000 == 0b00010000 {
                                    if covered_code_blocks[174] == 0 { new_block_reached = true; }
                                    covered_code_blocks[174] = 1;
                                }
                                    if input[120] & 0b00000011 == 0b00000011 {
                                    if covered_code_blocks[175] == 0 { new_block_reached = true; }
                                    covered_code_blocks[175] = 1;
                                        if input[119] & 0b01100000 == 0b00100000 {
                                        if covered_code_blocks[176] == 0 { new_block_reached = true; }
                                        covered_code_blocks[176] = 1;
                                    }
                                        if input[60] & 0b00011000 == 0b00000000 {
                                        if covered_code_blocks[177] == 0 { new_block_reached = true; }
                                        covered_code_blocks[177] = 1;
                                    }
                                        if input[58] & 0b01000000 == 0b01000000 {
                                        if covered_code_blocks[178] == 0 { new_block_reached = true; }
                                        covered_code_blocks[178] = 1;
                                            if input[94] & 0b00000011 == 0b00000011 {
                                            if covered_code_blocks[179] == 0 { new_block_reached = true; }
                                            covered_code_blocks[179] = 1;
                                        }
                                            if input[111] & 0b00100000 == 0b00100000 {
                                            if covered_code_blocks[180] == 0 { new_block_reached = true; }
                                            covered_code_blocks[180] = 1;
                                        }
                                            if input[98] & 0b11111100 == 0b10010000 {
                                            if covered_code_blocks[181] == 0 { new_block_reached = true; }
                                            covered_code_blocks[181] = 1;
                                        }
                                    }
                                        if input[0] & 0b11100000 == 0b10000000 {
                                        if covered_code_blocks[182] == 0 { new_block_reached = true; }
                                        covered_code_blocks[182] = 1;
                                            if input[25] & 0b00011111 == 0b00001010 {
                                            if covered_code_blocks[183] == 0 { new_block_reached = true; }
                                            covered_code_blocks[183] = 1;
                                        }
                                    }
                                        if input[77] & 0b00001110 == 0b00000110 {
                                        if covered_code_blocks[184] == 0 { new_block_reached = true; }
                                        covered_code_blocks[184] = 1;
                                    }
                                        if input[58] & 0b10000000 == 0b10000000 {
                                        if covered_code_blocks[185] == 0 { new_block_reached = true; }
                                        covered_code_blocks[185] = 1;
                                    }
                                }
                            }
                        }
                            if input[40] & 0b01100000 == 0b00100000 {
                            if covered_code_blocks[186] == 0 { new_block_reached = true; }
                            covered_code_blocks[186] = 1;
                                if input[126] & 0b00000010 == 0b00000010 {
                                if covered_code_blocks[187] == 0 { new_block_reached = true; }
                                covered_code_blocks[187] = 1;
                            }
                        }
                    }
                        if input[21] & 0b01100000 == 0b00100000 {
                        if covered_code_blocks[188] == 0 { new_block_reached = true; }
                        covered_code_blocks[188] = 1;
                    }
                }
            }
        }
            if input[27] & 0b01100000 == 0b00000000 {
            if covered_code_blocks[189] == 0 { new_block_reached = true; }
            covered_code_blocks[189] = 1;
                if input[32] & 0b00110000 == 0b00100000 {
                if covered_code_blocks[190] == 0 { new_block_reached = true; }
                covered_code_blocks[190] = 1;
                    if input[7] & 0b10000000 == 0b00000000 {
                    if covered_code_blocks[191] == 0 { new_block_reached = true; }
                    covered_code_blocks[191] = 1;
                }
            }
        }
            if input[112] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[192] == 0 { new_block_reached = true; }
            covered_code_blocks[192] = 1;
                if input[104] & 0b00001000 == 0b00001000 {
                if covered_code_blocks[193] == 0 { new_block_reached = true; }
                covered_code_blocks[193] = 1;
            }
        }
            if input[49] & 0b10000000 == 0b00000000 {
            if covered_code_blocks[194] == 0 { new_block_reached = true; }
            covered_code_blocks[194] = 1;
                if input[97] & 0b00000010 == 0b00000000 {
                if covered_code_blocks[195] == 0 { new_block_reached = true; }
                covered_code_blocks[195] = 1;
            }
        }
            if input[15] & 0b11000000 == 0b01000000 {
            if covered_code_blocks[196] == 0 { new_block_reached = true; }
            covered_code_blocks[196] = 1;
                if input[109] & 0b11111000 == 0b11110000 {
                if covered_code_blocks[197] == 0 { new_block_reached = true; }
                covered_code_blocks[197] = 1;
            }
                if input[44] & 0b01100000 == 0b01100000 {
                if covered_code_blocks[198] == 0 { new_block_reached = true; }
                covered_code_blocks[198] = 1;
            }
        }
            if input[81] & 0b01100000 == 0b00000000 {
            if covered_code_blocks[199] == 0 { new_block_reached = true; }
            covered_code_blocks[199] = 1;
        }
            if input[37] & 0b00000111 == 0b00000110 {
            if covered_code_blocks[200] == 0 { new_block_reached = true; }
            covered_code_blocks[200] = 1;
                if input[38] & 0b00100000 == 0b00000000 {
                if covered_code_blocks[201] == 0 { new_block_reached = true; }
                covered_code_blocks[201] = 1;
            }
        }
            if input[125] & 0b00111000 == 0b00000000 {
            if covered_code_blocks[202] == 0 { new_block_reached = true; }
            covered_code_blocks[202] = 1;
                if input[23] & 0b01100000 == 0b00000000 {
                if covered_code_blocks[203] == 0 { new_block_reached = true; }
                covered_code_blocks[203] = 1;
            }
        }
            if input[43] & 0b00000010 == 0b00000010 {
            if covered_code_blocks[204] == 0 { new_block_reached = true; }
            covered_code_blocks[204] = 1;
        }
    }
        if input[32] & 0b00000010 == 0b00000010 {
        if covered_code_blocks[205] == 0 { new_block_reached = true; }
        covered_code_blocks[205] = 1;
            if input[112] & 0b00000011 == 0b00000011 {
            if covered_code_blocks[206] == 0 { new_block_reached = true; }
            covered_code_blocks[206] = 1;
        }
    }
        if input[91] & 0b00000001 == 0b00000001 {
        if covered_code_blocks[207] == 0 { new_block_reached = true; }
        covered_code_blocks[207] = 1;
    }
        if input[7] & 0b00001100 == 0b00001000 {
        if covered_code_blocks[208] == 0 { new_block_reached = true; }
        covered_code_blocks[208] = 1;
    }
        if input[122] & 0b11000000 == 0b01000000 {
        if covered_code_blocks[209] == 0 { new_block_reached = true; }
        covered_code_blocks[209] = 1;
            if input[14] & 0b00001100 == 0b00000100 {
            if covered_code_blocks[210] == 0 { new_block_reached = true; }
            covered_code_blocks[210] = 1;
        }
            if input[13] & 0b01110000 == 0b00010000 {
            if covered_code_blocks[211] == 0 { new_block_reached = true; }
            covered_code_blocks[211] = 1;
        }
    }
        if input[71] & 0b00000100 == 0b00000000 {
        if covered_code_blocks[212] == 0 { new_block_reached = true; }
        covered_code_blocks[212] = 1;
    }
        if input[79] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[213] == 0 { new_block_reached = true; }
        covered_code_blocks[213] = 1;
            if input[80] & 0b00000011 == 0b00000010 {
            if covered_code_blocks[214] == 0 { new_block_reached = true; }
            covered_code_blocks[214] = 1;
        }
            if input[103] & 0b00011000 == 0b00011000 {
            if covered_code_blocks[215] == 0 { new_block_reached = true; }
            covered_code_blocks[215] = 1;
                if input[62] & 0b00000001 == 0b00000001 {
                if covered_code_blocks[216] == 0 { new_block_reached = true; }
                covered_code_blocks[216] = 1;
            }
                if input[65] & 0b00000011 == 0b00000010 {
                if covered_code_blocks[217] == 0 { new_block_reached = true; }
                covered_code_blocks[217] = 1;
            }
        }
            if input[48] & 0b00000110 == 0b00000010 {
            if covered_code_blocks[218] == 0 { new_block_reached = true; }
            covered_code_blocks[218] = 1;
                if input[24] & 0b00100000 == 0b00100000 {
                if covered_code_blocks[219] == 0 { new_block_reached = true; }
                covered_code_blocks[219] = 1;
            }
                if input[24] & 0b00011110 == 0b00001000 {
                if covered_code_blocks[220] == 0 { new_block_reached = true; }
                covered_code_blocks[220] = 1;
            }
                if input[14] & 0b01000000 == 0b01000000 {
                if covered_code_blocks[221] == 0 { new_block_reached = true; }
                covered_code_blocks[221] = 1;
                    if input[83] & 0b10000000 == 0b10000000 {
                    if covered_code_blocks[222] == 0 { new_block_reached = true; }
                    covered_code_blocks[222] = 1;
                        if input[51] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[223] == 0 { new_block_reached = true; }
                        covered_code_blocks[223] = 1;
                    }
                }
            }
        }
    }
        if input[66] & 0b00000011 == 0b00000010 {
        if covered_code_blocks[224] == 0 { new_block_reached = true; }
        covered_code_blocks[224] = 1;
    }
        if input[100] & 0b00011100 == 0b00011100 {
        if covered_code_blocks[225] == 0 { new_block_reached = true; }
        covered_code_blocks[225] = 1;
    }
        if input[20] & 0b00100000 == 0b00100000 {
        if covered_code_blocks[226] == 0 { new_block_reached = true; }
        covered_code_blocks[226] = 1;
    }
        if input[26] & 0b00001000 == 0b00000000 {
        if covered_code_blocks[227] == 0 { new_block_reached = true; }
        covered_code_blocks[227] = 1;
    }
        if input[19] & 0b00011000 == 0b00010000 {
        if covered_code_blocks[228] == 0 { new_block_reached = true; }
        covered_code_blocks[228] = 1;
    }
        if input[57] & 0b00100000 == 0b00000000 {
        if covered_code_blocks[229] == 0 { new_block_reached = true; }
        covered_code_blocks[229] = 1;
    }
        if input[82] & 0b11110000 == 0b00010000 {
        if covered_code_blocks[230] == 0 { new_block_reached = true; }
        covered_code_blocks[230] = 1;
    }
        if input[6] & 0b00001110 == 0b00001010 {
        if covered_code_blocks[231] == 0 { new_block_reached = true; }
        covered_code_blocks[231] = 1;
    }
        if input[32] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[232] == 0 { new_block_reached = true; }
        covered_code_blocks[232] = 1;
    }
        if input[105] & 0b00000110 == 0b00000000 {
        if covered_code_blocks[233] == 0 { new_block_reached = true; }
        covered_code_blocks[233] = 1;
            if input[60] & 0b10000000 == 0b00000000 {
            if covered_code_blocks[234] == 0 { new_block_reached = true; }
            covered_code_blocks[234] = 1;
                if input[12] & 0b00000100 == 0b00000100 {
                if covered_code_blocks[235] == 0 { new_block_reached = true; }
                covered_code_blocks[235] = 1;
            }
        }
            if input[19] & 0b00000111 == 0b00000010 {
            if covered_code_blocks[236] == 0 { new_block_reached = true; }
            covered_code_blocks[236] = 1;
        }
    }
        if input[68] & 0b10000000 == 0b00000000 {
        if covered_code_blocks[237] == 0 { new_block_reached = true; }
        covered_code_blocks[237] = 1;
    }
        if input[53] & 0b00100000 == 0b00000000 {
        if covered_code_blocks[238] == 0 { new_block_reached = true; }
        covered_code_blocks[238] = 1;
    }
        if input[115] & 0b00000100 == 0b00000100 {
        if covered_code_blocks[239] == 0 { new_block_reached = true; }
        covered_code_blocks[239] = 1;
    }
        if input[85] & 0b10000000 == 0b00000000 {
        if covered_code_blocks[240] == 0 { new_block_reached = true; }
        covered_code_blocks[240] = 1;
            if input[35] & 0b11110000 == 0b01100000 {
            if covered_code_blocks[241] == 0 { new_block_reached = true; }
            covered_code_blocks[241] = 1;
        }
            if input[20] & 0b11000000 == 0b11000000 {
            if covered_code_blocks[242] == 0 { new_block_reached = true; }
            covered_code_blocks[242] = 1;
        }
            if input[117] & 0b00000001 == 0b00000000 {
            if covered_code_blocks[243] == 0 { new_block_reached = true; }
            covered_code_blocks[243] = 1;
                if input[70] & 0b00000010 == 0b00000010 {
                if covered_code_blocks[244] == 0 { new_block_reached = true; }
                covered_code_blocks[244] = 1;
            }
        }
            if input[26] & 0b00000111 == 0b00000110 {
            if covered_code_blocks[245] == 0 { new_block_reached = true; }
            covered_code_blocks[245] = 1;
        }
            if input[92] & 0b00010000 == 0b00000000 {
            if covered_code_blocks[246] == 0 { new_block_reached = true; }
            covered_code_blocks[246] = 1;
                if input[85] & 0b01100000 == 0b00000000 {
                if covered_code_blocks[247] == 0 { new_block_reached = true; }
                covered_code_blocks[247] = 1;
            }
        }
    }
        if input[27] & 0b00000110 == 0b00000110 {
        if covered_code_blocks[248] == 0 { new_block_reached = true; }
        covered_code_blocks[248] = 1;
    }
        if input[46] & 0b00000110 == 0b00000110 {
        if covered_code_blocks[249] == 0 { new_block_reached = true; }
        covered_code_blocks[249] = 1;
    }
        if input[70] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[250] == 0 { new_block_reached = true; }
        covered_code_blocks[250] = 1;
    }
        if input[4] & 0b00000011 == 0b00000011 {
        if covered_code_blocks[251] == 0 { new_block_reached = true; }
        covered_code_blocks[251] = 1;
            if input[118] & 0b00000110 == 0b00000000 {
            if covered_code_blocks[252] == 0 { new_block_reached = true; }
            covered_code_blocks[252] = 1;
        }
    }
        if input[17] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[253] == 0 { new_block_reached = true; }
        covered_code_blocks[253] = 1;
            if input[44] & 0b00000010 == 0b00000000 {
            if covered_code_blocks[254] == 0 { new_block_reached = true; }
            covered_code_blocks[254] = 1;
        }
            if input[40] & 0b00011000 == 0b00000000 {
            if covered_code_blocks[255] == 0 { new_block_reached = true; }
            covered_code_blocks[255] = 1;
        }
            if input[21] & 0b00000001 == 0b00000000 {
            if covered_code_blocks[256] == 0 { new_block_reached = true; }
            covered_code_blocks[256] = 1;
        }
            if input[34] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[257] == 0 { new_block_reached = true; }
            covered_code_blocks[257] = 1;
        }
            if input[78] & 0b00000001 == 0b00000000 {
            if covered_code_blocks[258] == 0 { new_block_reached = true; }
            covered_code_blocks[258] = 1;
                if input[104] & 0b00000011 == 0b00000000 {
                if covered_code_blocks[259] == 0 { new_block_reached = true; }
                covered_code_blocks[259] = 1;
            }
        }
            if input[115] & 0b00001000 == 0b00001000 {
            if covered_code_blocks[260] == 0 { new_block_reached = true; }
            covered_code_blocks[260] = 1;
        }
            if input[63] & 0b00000110 == 0b00000100 {
            if covered_code_blocks[261] == 0 { new_block_reached = true; }
            covered_code_blocks[261] = 1;
        }
            if input[3] & 0b01110000 == 0b01000000 {
            if covered_code_blocks[262] == 0 { new_block_reached = true; }
            covered_code_blocks[262] = 1;
        }
    }
        if input[85] & 0b00010000 == 0b00000000 {
        if covered_code_blocks[263] == 0 { new_block_reached = true; }
        covered_code_blocks[263] = 1;
    }
        if input[124] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[264] == 0 { new_block_reached = true; }
        covered_code_blocks[264] = 1;
    }
        if input[10] & 0b00110000 == 0b00010000 {
        if covered_code_blocks[265] == 0 { new_block_reached = true; }
        covered_code_blocks[265] = 1;
            if input[37] & 0b00001000 == 0b00000000 {
            if covered_code_blocks[266] == 0 { new_block_reached = true; }
            covered_code_blocks[266] = 1;
        }
            if input[68] & 0b00001000 == 0b00000000 {
            if covered_code_blocks[267] == 0 { new_block_reached = true; }
            covered_code_blocks[267] = 1;
        }
            if input[62] & 0b00011110 == 0b00000000 {
            if covered_code_blocks[268] == 0 { new_block_reached = true; }
            covered_code_blocks[268] = 1;
                if input[103] & 0b00000001 == 0b00000001 {
                if covered_code_blocks[269] == 0 { new_block_reached = true; }
                covered_code_blocks[269] = 1;
                    if input[106] & 0b00000111 == 0b00000001 {
                    if covered_code_blocks[270] == 0 { new_block_reached = true; }
                    covered_code_blocks[270] = 1;
                }
            }
                if input[69] & 0b11111000 == 0b10010000 {
                if covered_code_blocks[271] == 0 { new_block_reached = true; }
                covered_code_blocks[271] = 1;
            }
        }
            if input[104] & 0b00000100 == 0b00000100 {
            if covered_code_blocks[272] == 0 { new_block_reached = true; }
            covered_code_blocks[272] = 1;
                if input[25] & 0b11000000 == 0b10000000 {
                if covered_code_blocks[273] == 0 { new_block_reached = true; }
                covered_code_blocks[273] = 1;
            }
                if input[63] & 0b10000000 == 0b00000000 {
                if covered_code_blocks[274] == 0 { new_block_reached = true; }
                covered_code_blocks[274] = 1;
                    if input[56] & 0b10000000 == 0b10000000 {
                    if covered_code_blocks[275] == 0 { new_block_reached = true; }
                    covered_code_blocks[275] = 1;
                }
            }
        }
    }
        if input[5] & 0b00000011 == 0b00000001 {
        if covered_code_blocks[276] == 0 { new_block_reached = true; }
        covered_code_blocks[276] = 1;
    }
        if input[14] & 0b00010000 == 0b00000000 {
        if covered_code_blocks[277] == 0 { new_block_reached = true; }
        covered_code_blocks[277] = 1;
    }
        if input[121] & 0b00000100 == 0b00000100 {
        if covered_code_blocks[278] == 0 { new_block_reached = true; }
        covered_code_blocks[278] = 1;
            if input[99] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[279] == 0 { new_block_reached = true; }
            covered_code_blocks[279] = 1;
                if input[76] & 0b00110000 == 0b00000000 {
                if covered_code_blocks[280] == 0 { new_block_reached = true; }
                covered_code_blocks[280] = 1;
                    if input[68] & 0b01100000 == 0b00000000 {
                    if covered_code_blocks[281] == 0 { new_block_reached = true; }
                    covered_code_blocks[281] = 1;
                        if input[71] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[282] == 0 { new_block_reached = true; }
                        covered_code_blocks[282] = 1;
                    }
                }
                    if input[112] & 0b00011100 == 0b00000000 {
                    if covered_code_blocks[283] == 0 { new_block_reached = true; }
                    covered_code_blocks[283] = 1;
                        if input[78] & 0b00000110 == 0b00000100 {
                        if covered_code_blocks[284] == 0 { new_block_reached = true; }
                        covered_code_blocks[284] = 1;
                            if input[78] & 0b11100000 == 0b11000000 {
                            if covered_code_blocks[285] == 0 { new_block_reached = true; }
                            covered_code_blocks[285] = 1;
                        }
                            if input[73] & 0b10000000 == 0b10000000 {
                            if covered_code_blocks[286] == 0 { new_block_reached = true; }
                            covered_code_blocks[286] = 1;
                        }
                            if input[103] & 0b11000000 == 0b10000000 {
                            if covered_code_blocks[287] == 0 { new_block_reached = true; }
                            covered_code_blocks[287] = 1;
                                if input[43] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[288] == 0 { new_block_reached = true; }
                                covered_code_blocks[288] = 1;
                                    if input[52] & 0b00000111 == 0b00000100 {
                                    if covered_code_blocks[289] == 0 { new_block_reached = true; }
                                    covered_code_blocks[289] = 1;
                                }
                            }
                        }
                    }
                        if input[43] & 0b00010000 == 0b00000000 {
                        if covered_code_blocks[290] == 0 { new_block_reached = true; }
                        covered_code_blocks[290] = 1;
                            if input[105] & 0b10000000 == 0b00000000 {
                            if covered_code_blocks[291] == 0 { new_block_reached = true; }
                            covered_code_blocks[291] = 1;
                        }
                            if input[75] & 0b01000000 == 0b00000000 {
                            if covered_code_blocks[292] == 0 { new_block_reached = true; }
                            covered_code_blocks[292] = 1;
                                if input[114] & 0b00000110 == 0b00000000 {
                                if covered_code_blocks[293] == 0 { new_block_reached = true; }
                                covered_code_blocks[293] = 1;
                            }
                                if input[112] & 0b01000000 == 0b01000000 {
                                if covered_code_blocks[294] == 0 { new_block_reached = true; }
                                covered_code_blocks[294] = 1;
                                    if input[79] & 0b00000001 == 0b00000001 {
                                    if covered_code_blocks[295] == 0 { new_block_reached = true; }
                                    covered_code_blocks[295] = 1;
                                        if input[53] & 0b00000100 == 0b00000100 {
                                        if covered_code_blocks[296] == 0 { new_block_reached = true; }
                                        covered_code_blocks[296] = 1;
                                    }
                                }
                                    if input[104] & 0b00100000 == 0b00000000 {
                                    if covered_code_blocks[297] == 0 { new_block_reached = true; }
                                    covered_code_blocks[297] = 1;
                                }
                                    if input[86] & 0b00010000 == 0b00010000 {
                                    if covered_code_blocks[298] == 0 { new_block_reached = true; }
                                    covered_code_blocks[298] = 1;
                                }
                                    if input[31] & 0b10000000 == 0b10000000 {
                                    if covered_code_blocks[299] == 0 { new_block_reached = true; }
                                    covered_code_blocks[299] = 1;
                                }
                            }
                                if input[28] & 0b11100000 == 0b01100000 {
                                if covered_code_blocks[300] == 0 { new_block_reached = true; }
                                covered_code_blocks[300] = 1;
                                    if input[34] & 0b00000111 == 0b00000100 {
                                    if covered_code_blocks[301] == 0 { new_block_reached = true; }
                                    covered_code_blocks[301] = 1;
                                }
                            }
                        }
                            if input[94] & 0b11110000 == 0b00010000 {
                            if covered_code_blocks[302] == 0 { new_block_reached = true; }
                            covered_code_blocks[302] = 1;
                        }
                    }
                        if input[85] & 0b00001110 == 0b00000010 {
                        if covered_code_blocks[303] == 0 { new_block_reached = true; }
                        covered_code_blocks[303] = 1;
                    }
                }
                    if input[27] & 0b10000000 == 0b10000000 {
                    if covered_code_blocks[304] == 0 { new_block_reached = true; }
                    covered_code_blocks[304] = 1;
                        if input[115] & 0b10000000 == 0b10000000 {
                        if covered_code_blocks[305] == 0 { new_block_reached = true; }
                        covered_code_blocks[305] = 1;
                    }
                        if input[89] & 0b00000001 == 0b00000000 {
                        if covered_code_blocks[306] == 0 { new_block_reached = true; }
                        covered_code_blocks[306] = 1;
                            if input[0] & 0b00001000 == 0b00000000 {
                            if covered_code_blocks[307] == 0 { new_block_reached = true; }
                            covered_code_blocks[307] = 1;
                        }
                            if input[123] & 0b10000000 == 0b00000000 {
                            if covered_code_blocks[308] == 0 { new_block_reached = true; }
                            covered_code_blocks[308] = 1;
                                if input[27] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[309] == 0 { new_block_reached = true; }
                                covered_code_blocks[309] = 1;
                                    if input[64] & 0b00000011 == 0b00000001 {
                                    if covered_code_blocks[310] == 0 { new_block_reached = true; }
                                    covered_code_blocks[310] = 1;
                                }
                                    if input[34] & 0b01000000 == 0b01000000 {
                                    if covered_code_blocks[311] == 0 { new_block_reached = true; }
                                    covered_code_blocks[311] = 1;
                                        if input[40] & 0b00000001 == 0b00000000 {
                                        if covered_code_blocks[312] == 0 { new_block_reached = true; }
                                        covered_code_blocks[312] = 1;
                                    }
                                }
                                    if input[97] & 0b00001000 == 0b00001000 {
                                    if covered_code_blocks[313] == 0 { new_block_reached = true; }
                                    covered_code_blocks[313] = 1;
                                        if input[31] & 0b00000100 == 0b00000000 {
                                        if covered_code_blocks[314] == 0 { new_block_reached = true; }
                                        covered_code_blocks[314] = 1;
                                            if input[125] & 0b10000000 == 0b00000000 {
                                            if covered_code_blocks[315] == 0 { new_block_reached = true; }
                                            covered_code_blocks[315] = 1;
                                                if input[29] & 0b00100000 == 0b00000000 {
                                                if covered_code_blocks[316] == 0 { new_block_reached = true; }
                                                covered_code_blocks[316] = 1;
                                            }
                                                if input[71] & 0b00001000 == 0b00000000 {
                                                if covered_code_blocks[317] == 0 { new_block_reached = true; }
                                                covered_code_blocks[317] = 1;
                                            }
                                        }
                                            if input[11] & 0b10000000 == 0b00000000 {
                                            if covered_code_blocks[318] == 0 { new_block_reached = true; }
                                            covered_code_blocks[318] = 1;
                                        }
                                    }
                                        if input[96] & 0b11000000 == 0b11000000 {
                                        if covered_code_blocks[319] == 0 { new_block_reached = true; }
                                        covered_code_blocks[319] = 1;
                                            if input[60] & 0b00000010 == 0b00000000 {
                                            if covered_code_blocks[320] == 0 { new_block_reached = true; }
                                            covered_code_blocks[320] = 1;
                                                if input[1] & 0b00000001 == 0b00000001 {
                                                if covered_code_blocks[321] == 0 { new_block_reached = true; }
                                                covered_code_blocks[321] = 1;
                                            }
                                        }
                                            if input[124] & 0b00010000 == 0b00000000 {
                                            if covered_code_blocks[322] == 0 { new_block_reached = true; }
                                            covered_code_blocks[322] = 1;
                                        }
                                            if input[48] & 0b00010000 == 0b00010000 {
                                            if covered_code_blocks[323] == 0 { new_block_reached = true; }
                                            covered_code_blocks[323] = 1;
                                        }
                                            if input[43] & 0b01000000 == 0b01000000 {
                                            if covered_code_blocks[324] == 0 { new_block_reached = true; }
                                            covered_code_blocks[324] = 1;
                                        }
                                            if input[69] & 0b00000001 == 0b00000001 {
                                            if covered_code_blocks[325] == 0 { new_block_reached = true; }
                                            covered_code_blocks[325] = 1;
                                                if input[38] & 0b11000000 == 0b01000000 {
                                                if covered_code_blocks[326] == 0 { new_block_reached = true; }
                                                covered_code_blocks[326] = 1;
                                            }
                                                if input[46] & 0b00001000 == 0b00001000 {
                                                if covered_code_blocks[327] == 0 { new_block_reached = true; }
                                                covered_code_blocks[327] = 1;
                                            }
                                        }
                                    }
                                        if input[66] & 0b00010000 == 0b00000000 {
                                        if covered_code_blocks[328] == 0 { new_block_reached = true; }
                                        covered_code_blocks[328] = 1;
                                    }
                                }
                            }
                        }
                    }
                        if input[61] & 0b01000000 == 0b01000000 {
                        if covered_code_blocks[329] == 0 { new_block_reached = true; }
                        covered_code_blocks[329] = 1;
                    }
                        if input[27] & 0b00001000 == 0b00000000 {
                        if covered_code_blocks[330] == 0 { new_block_reached = true; }
                        covered_code_blocks[330] = 1;
                            if input[33] & 0b10000000 == 0b10000000 {
                            if covered_code_blocks[331] == 0 { new_block_reached = true; }
                            covered_code_blocks[331] = 1;
                                if input[68] & 0b00000110 == 0b00000100 {
                                if covered_code_blocks[332] == 0 { new_block_reached = true; }
                                covered_code_blocks[332] = 1;
                                    if input[8] & 0b00111000 == 0b00000000 {
                                    if covered_code_blocks[333] == 0 { new_block_reached = true; }
                                    covered_code_blocks[333] = 1;
                                }
                                    if input[29] & 0b11000000 == 0b11000000 {
                                    if covered_code_blocks[334] == 0 { new_block_reached = true; }
                                    covered_code_blocks[334] = 1;
                                }
                                    if input[87] & 0b11000000 == 0b11000000 {
                                    if covered_code_blocks[335] == 0 { new_block_reached = true; }
                                    covered_code_blocks[335] = 1;
                                }
                            }
                                if input[46] & 0b00010000 == 0b00010000 {
                                if covered_code_blocks[336] == 0 { new_block_reached = true; }
                                covered_code_blocks[336] = 1;
                            }
                        }
                    }
                }
                    if input[103] & 0b00000010 == 0b00000010 {
                    if covered_code_blocks[337] == 0 { new_block_reached = true; }
                    covered_code_blocks[337] = 1;
                        if input[66] & 0b10000000 == 0b10000000 {
                        if covered_code_blocks[338] == 0 { new_block_reached = true; }
                        covered_code_blocks[338] = 1;
                            if input[56] & 0b00000001 == 0b00000001 {
                            if covered_code_blocks[339] == 0 { new_block_reached = true; }
                            covered_code_blocks[339] = 1;
                                if input[84] & 0b00000010 == 0b00000000 {
                                if covered_code_blocks[340] == 0 { new_block_reached = true; }
                                covered_code_blocks[340] = 1;
                            }
                                if input[61] & 0b00000001 == 0b00000001 {
                                if covered_code_blocks[341] == 0 { new_block_reached = true; }
                                covered_code_blocks[341] = 1;
                                    if input[91] & 0b00001000 == 0b00000000 {
                                    if covered_code_blocks[342] == 0 { new_block_reached = true; }
                                    covered_code_blocks[342] = 1;
                                        if input[96] & 0b00000010 == 0b00000010 {
                                        if covered_code_blocks[343] == 0 { new_block_reached = true; }
                                        covered_code_blocks[343] = 1;
                                    }
                                }
                                    if input[42] & 0b01000000 == 0b00000000 {
                                    if covered_code_blocks[344] == 0 { new_block_reached = true; }
                                    covered_code_blocks[344] = 1;
                                }
                            }
                        }
                            if input[118] & 0b01100000 == 0b00100000 {
                            if covered_code_blocks[345] == 0 { new_block_reached = true; }
                            covered_code_blocks[345] = 1;
                        }
                    }
                        if input[74] & 0b00000001 == 0b00000000 {
                        if covered_code_blocks[346] == 0 { new_block_reached = true; }
                        covered_code_blocks[346] = 1;
                    }
                        if input[47] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[347] == 0 { new_block_reached = true; }
                        covered_code_blocks[347] = 1;
                    }
                        if input[61] & 0b00000010 == 0b00000000 {
                        if covered_code_blocks[348] == 0 { new_block_reached = true; }
                        covered_code_blocks[348] = 1;
                    }
                        if input[24] & 0b11000000 == 0b00000000 {
                        if covered_code_blocks[349] == 0 { new_block_reached = true; }
                        covered_code_blocks[349] = 1;
                            if input[109] & 0b00000111 == 0b00000010 {
                            if covered_code_blocks[350] == 0 { new_block_reached = true; }
                            covered_code_blocks[350] = 1;
                                if input[86] & 0b10000000 == 0b10000000 {
                                if covered_code_blocks[351] == 0 { new_block_reached = true; }
                                covered_code_blocks[351] = 1;
                                    if input[31] & 0b01000000 == 0b01000000 {
                                    if covered_code_blocks[352] == 0 { new_block_reached = true; }
                                    covered_code_blocks[352] = 1;
                                }
                            }
                                if input[81] & 0b10000000 == 0b10000000 {
                                if covered_code_blocks[353] == 0 { new_block_reached = true; }
                                covered_code_blocks[353] = 1;
                            }
                        }
                    }
                }
                    if input[78] & 0b00001000 == 0b00001000 {
                    if covered_code_blocks[354] == 0 { new_block_reached = true; }
                    covered_code_blocks[354] = 1;
                        if input[7] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[355] == 0 { new_block_reached = true; }
                        covered_code_blocks[355] = 1;
                    }
                        if input[77] & 0b00000001 == 0b00000000 {
                        if covered_code_blocks[356] == 0 { new_block_reached = true; }
                        covered_code_blocks[356] = 1;
                    }
                }
            }
                if input[5] & 0b00000100 == 0b00000000 {
                if covered_code_blocks[357] == 0 { new_block_reached = true; }
                covered_code_blocks[357] = 1;
                    if input[102] & 0b00000011 == 0b00000010 {
                    if covered_code_blocks[358] == 0 { new_block_reached = true; }
                    covered_code_blocks[358] = 1;
                }
                    if input[76] & 0b01000000 == 0b00000000 {
                    if covered_code_blocks[359] == 0 { new_block_reached = true; }
                    covered_code_blocks[359] = 1;
                }
                    if input[8] & 0b01000000 == 0b01000000 {
                    if covered_code_blocks[360] == 0 { new_block_reached = true; }
                    covered_code_blocks[360] = 1;
                        if input[120] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[361] == 0 { new_block_reached = true; }
                        covered_code_blocks[361] = 1;
                            if input[57] & 0b00010000 == 0b00000000 {
                            if covered_code_blocks[362] == 0 { new_block_reached = true; }
                            covered_code_blocks[362] = 1;
                        }
                            if input[123] & 0b00000100 == 0b00000100 {
                            if covered_code_blocks[363] == 0 { new_block_reached = true; }
                            covered_code_blocks[363] = 1;
                                if input[32] & 0b00000001 == 0b00000001 {
                                if covered_code_blocks[364] == 0 { new_block_reached = true; }
                                covered_code_blocks[364] = 1;
                                    if input[42] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[365] == 0 { new_block_reached = true; }
                                    covered_code_blocks[365] = 1;
                                        if input[58] & 0b00010000 == 0b00000000 {
                                        if covered_code_blocks[366] == 0 { new_block_reached = true; }
                                        covered_code_blocks[366] = 1;
                                    }
                                }
                                    if input[15] & 0b00000001 == 0b00000001 {
                                    if covered_code_blocks[367] == 0 { new_block_reached = true; }
                                    covered_code_blocks[367] = 1;
                                        if input[49] & 0b00000001 == 0b00000001 {
                                        if covered_code_blocks[368] == 0 { new_block_reached = true; }
                                        covered_code_blocks[368] = 1;
                                    }
                                        if input[14] & 0b00000010 == 0b00000010 {
                                        if covered_code_blocks[369] == 0 { new_block_reached = true; }
                                        covered_code_blocks[369] = 1;
                                    }
                                }
                                    if input[40] & 0b00000010 == 0b00000010 {
                                    if covered_code_blocks[370] == 0 { new_block_reached = true; }
                                    covered_code_blocks[370] = 1;
                                }
                            }
                        }
                            if input[88] & 0b00001000 == 0b00000000 {
                            if covered_code_blocks[371] == 0 { new_block_reached = true; }
                            covered_code_blocks[371] = 1;
                                if input[91] & 0b01000000 == 0b01000000 {
                                if covered_code_blocks[372] == 0 { new_block_reached = true; }
                                covered_code_blocks[372] = 1;
                                    if input[22] & 0b00000001 == 0b00000000 {
                                    if covered_code_blocks[373] == 0 { new_block_reached = true; }
                                    covered_code_blocks[373] = 1;
                                }
                                    if input[49] & 0b00000010 == 0b00000000 {
                                    if covered_code_blocks[374] == 0 { new_block_reached = true; }
                                    covered_code_blocks[374] = 1;
                                }
                                    if input[39] & 0b00000001 == 0b00000001 {
                                    if covered_code_blocks[375] == 0 { new_block_reached = true; }
                                    covered_code_blocks[375] = 1;
                                }
                                    if input[38] & 0b00011000 == 0b00000000 {
                                    if covered_code_blocks[376] == 0 { new_block_reached = true; }
                                    covered_code_blocks[376] = 1;
                                }
                                    if input[16] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[377] == 0 { new_block_reached = true; }
                                    covered_code_blocks[377] = 1;
                                        if input[10] & 0b01000000 == 0b01000000 {
                                        if covered_code_blocks[378] == 0 { new_block_reached = true; }
                                        covered_code_blocks[378] = 1;
                                    }
                                }
                            }
                        }
                    }
                }
                    if input[95] & 0b00000001 == 0b00000000 {
                    if covered_code_blocks[379] == 0 { new_block_reached = true; }
                    covered_code_blocks[379] = 1;
                }
            }
        }
    }
        if input[28] & 0b00000011 == 0b00000001 {
        if covered_code_blocks[380] == 0 { new_block_reached = true; }
        covered_code_blocks[380] = 1;
            if input[65] & 0b10000000 == 0b00000000 {
            if covered_code_blocks[381] == 0 { new_block_reached = true; }
            covered_code_blocks[381] = 1;
        }
    }
        if input[98] & 0b00000010 == 0b00000010 {
        if covered_code_blocks[382] == 0 { new_block_reached = true; }
        covered_code_blocks[382] = 1;
    }
        if input[44] & 0b00001000 == 0b00000000 {
        if covered_code_blocks[383] == 0 { new_block_reached = true; }
        covered_code_blocks[383] = 1;
    }
        if input[83] & 0b00000001 == 0b00000001 {
        if covered_code_blocks[384] == 0 { new_block_reached = true; }
        covered_code_blocks[384] = 1;
    }
        if input[106] & 0b00001000 == 0b00000000 {
        if covered_code_blocks[385] == 0 { new_block_reached = true; }
        covered_code_blocks[385] = 1;
    }
        if input[15] & 0b00010000 == 0b00010000 {
        if covered_code_blocks[386] == 0 { new_block_reached = true; }
        covered_code_blocks[386] = 1;
            if input[53] & 0b00000011 == 0b00000011 {
            if covered_code_blocks[387] == 0 { new_block_reached = true; }
            covered_code_blocks[387] = 1;
                if input[67] & 0b00100000 == 0b00000000 {
                if covered_code_blocks[388] == 0 { new_block_reached = true; }
                covered_code_blocks[388] = 1;
                    if input[115] & 0b00000010 == 0b00000010 {
                    if covered_code_blocks[389] == 0 { new_block_reached = true; }
                    covered_code_blocks[389] = 1;
                }
                    if input[101] & 0b10000000 == 0b00000000 {
                    if covered_code_blocks[390] == 0 { new_block_reached = true; }
                    covered_code_blocks[390] = 1;
                }
                    if input[52] & 0b00001000 == 0b00000000 {
                    if covered_code_blocks[391] == 0 { new_block_reached = true; }
                    covered_code_blocks[391] = 1;
                        if input[21] & 0b00010000 == 0b00010000 {
                        if covered_code_blocks[392] == 0 { new_block_reached = true; }
                        covered_code_blocks[392] = 1;
                    }
                }
            }
        }
    }
        if input[117] & 0b00011000 == 0b00010000 {
        if covered_code_blocks[393] == 0 { new_block_reached = true; }
        covered_code_blocks[393] = 1;
    }
        if input[100] & 0b00000011 == 0b00000011 {
        if covered_code_blocks[394] == 0 { new_block_reached = true; }
        covered_code_blocks[394] = 1;
            if input[81] & 0b00010000 == 0b00000000 {
            if covered_code_blocks[395] == 0 { new_block_reached = true; }
            covered_code_blocks[395] = 1;
                if input[36] & 0b00000011 == 0b00000001 {
                if covered_code_blocks[396] == 0 { new_block_reached = true; }
                covered_code_blocks[396] = 1;
            }
        }
            if input[97] & 0b00000100 == 0b00000100 {
            if covered_code_blocks[397] == 0 { new_block_reached = true; }
            covered_code_blocks[397] = 1;
        }
            if input[37] & 0b00010000 == 0b00000000 {
            if covered_code_blocks[398] == 0 { new_block_reached = true; }
            covered_code_blocks[398] = 1;
        }
            if input[103] & 0b00100000 == 0b00000000 {
            if covered_code_blocks[399] == 0 { new_block_reached = true; }
            covered_code_blocks[399] = 1;
                if input[26] & 0b00010000 == 0b00010000 {
                if covered_code_blocks[400] == 0 { new_block_reached = true; }
                covered_code_blocks[400] = 1;
                    if input[89] & 0b10000000 == 0b10000000 {
                    if covered_code_blocks[401] == 0 { new_block_reached = true; }
                    covered_code_blocks[401] = 1;
                }
                    if input[70] & 0b00010000 == 0b00010000 {
                    if covered_code_blocks[402] == 0 { new_block_reached = true; }
                    covered_code_blocks[402] = 1;
                        if input[121] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[403] == 0 { new_block_reached = true; }
                        covered_code_blocks[403] = 1;
                            if input[27] & 0b00010000 == 0b00000000 {
                            if covered_code_blocks[404] == 0 { new_block_reached = true; }
                            covered_code_blocks[404] = 1;
                        }
                    }
                        if input[50] & 0b10000000 == 0b10000000 {
                        if covered_code_blocks[405] == 0 { new_block_reached = true; }
                        covered_code_blocks[405] = 1;
                    }
                }
                    if input[12] & 0b00000010 == 0b00000000 {
                    if covered_code_blocks[406] == 0 { new_block_reached = true; }
                    covered_code_blocks[406] = 1;
                }
            }
        }
    }
        if input[110] & 0b00000001 == 0b00000000 {
        if covered_code_blocks[407] == 0 { new_block_reached = true; }
        covered_code_blocks[407] = 1;
            if input[16] & 0b00000001 == 0b00000000 {
            if covered_code_blocks[408] == 0 { new_block_reached = true; }
            covered_code_blocks[408] = 1;
                if input[74] & 0b00010000 == 0b00000000 {
                if covered_code_blocks[409] == 0 { new_block_reached = true; }
                covered_code_blocks[409] = 1;
                    if input[94] & 0b00000100 == 0b00000000 {
                    if covered_code_blocks[410] == 0 { new_block_reached = true; }
                    covered_code_blocks[410] = 1;
                        if input[42] & 0b00100000 == 0b00000000 {
                        if covered_code_blocks[411] == 0 { new_block_reached = true; }
                        covered_code_blocks[411] = 1;
                    }
                        if input[4] & 0b01000000 == 0b00000000 {
                        if covered_code_blocks[412] == 0 { new_block_reached = true; }
                        covered_code_blocks[412] = 1;
                            if input[125] & 0b00000001 == 0b00000001 {
                            if covered_code_blocks[413] == 0 { new_block_reached = true; }
                            covered_code_blocks[413] = 1;
                        }
                            if input[114] & 0b00001000 == 0b00000000 {
                            if covered_code_blocks[414] == 0 { new_block_reached = true; }
                            covered_code_blocks[414] = 1;
                        }
                    }
                        if input[32] & 0b00001100 == 0b00001000 {
                        if covered_code_blocks[415] == 0 { new_block_reached = true; }
                        covered_code_blocks[415] = 1;
                    }
                        if input[96] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[416] == 0 { new_block_reached = true; }
                        covered_code_blocks[416] = 1;
                            if input[119] & 0b10000000 == 0b10000000 {
                            if covered_code_blocks[417] == 0 { new_block_reached = true; }
                            covered_code_blocks[417] = 1;
                        }
                    }
                }
            }
                if input[63] & 0b00000001 == 0b00000001 {
                if covered_code_blocks[418] == 0 { new_block_reached = true; }
                covered_code_blocks[418] = 1;
            }
                if input[66] & 0b00001000 == 0b00000000 {
                if covered_code_blocks[419] == 0 { new_block_reached = true; }
                covered_code_blocks[419] = 1;
            }
        }
            if input[70] & 0b01000000 == 0b00000000 {
            if covered_code_blocks[420] == 0 { new_block_reached = true; }
            covered_code_blocks[420] = 1;
        }
            if input[60] & 0b00000100 == 0b00000100 {
            if covered_code_blocks[421] == 0 { new_block_reached = true; }
            covered_code_blocks[421] = 1;
        }
            if input[14] & 0b00000001 == 0b00000001 {
            if covered_code_blocks[422] == 0 { new_block_reached = true; }
            covered_code_blocks[422] = 1;
                if input[126] & 0b10000000 == 0b10000000 {
                if covered_code_blocks[423] == 0 { new_block_reached = true; }
                covered_code_blocks[423] = 1;
                    if input[59] & 0b01000000 == 0b01000000 {
                    if covered_code_blocks[424] == 0 { new_block_reached = true; }
                    covered_code_blocks[424] = 1;
                        if input[60] & 0b01000000 == 0b01000000 {
                        if covered_code_blocks[425] == 0 { new_block_reached = true; }
                        covered_code_blocks[425] = 1;
                    }
                        if input[41] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[426] == 0 { new_block_reached = true; }
                        covered_code_blocks[426] = 1;
                    }
                }
            }
                if input[86] & 0b00100000 == 0b00100000 {
                if covered_code_blocks[427] == 0 { new_block_reached = true; }
                covered_code_blocks[427] = 1;
                    if input[87] & 0b00000010 == 0b00000010 {
                    if covered_code_blocks[428] == 0 { new_block_reached = true; }
                    covered_code_blocks[428] = 1;
                        if input[120] & 0b01000000 == 0b01000000 {
                        if covered_code_blocks[429] == 0 { new_block_reached = true; }
                        covered_code_blocks[429] = 1;
                    }
                        if input[4] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[430] == 0 { new_block_reached = true; }
                        covered_code_blocks[430] = 1;
                    }
                }
                    if input[43] & 0b00000100 == 0b00000000 {
                    if covered_code_blocks[431] == 0 { new_block_reached = true; }
                    covered_code_blocks[431] = 1;
                }
            }
        }
    }
        if input[15] & 0b00000010 == 0b00000000 {
        if covered_code_blocks[432] == 0 { new_block_reached = true; }
        covered_code_blocks[432] = 1;
            if input[7] & 0b00000010 == 0b00000000 {
            if covered_code_blocks[433] == 0 { new_block_reached = true; }
            covered_code_blocks[433] = 1;
                if input[122] & 0b00000010 == 0b00000010 {
                if covered_code_blocks[434] == 0 { new_block_reached = true; }
                covered_code_blocks[434] = 1;
                    if input[102] & 0b00001000 == 0b00000000 {
                    if covered_code_blocks[435] == 0 { new_block_reached = true; }
                    covered_code_blocks[435] = 1;
                }
                    if input[75] & 0b10000000 == 0b00000000 {
                    if covered_code_blocks[436] == 0 { new_block_reached = true; }
                    covered_code_blocks[436] = 1;
                        if input[76] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[437] == 0 { new_block_reached = true; }
                        covered_code_blocks[437] = 1;
                            if input[4] & 0b00001000 == 0b00001000 {
                            if covered_code_blocks[438] == 0 { new_block_reached = true; }
                            covered_code_blocks[438] = 1;
                                if input[13] & 0b00001000 == 0b00001000 {
                                if covered_code_blocks[439] == 0 { new_block_reached = true; }
                                covered_code_blocks[439] = 1;
                            }
                                if input[123] & 0b01000000 == 0b01000000 {
                                if covered_code_blocks[440] == 0 { new_block_reached = true; }
                                covered_code_blocks[440] = 1;
                            }
                                if input[6] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[441] == 0 { new_block_reached = true; }
                                covered_code_blocks[441] = 1;
                                    if input[51] & 0b00000001 == 0b00000000 {
                                    if covered_code_blocks[442] == 0 { new_block_reached = true; }
                                    covered_code_blocks[442] = 1;
                                }
                                    if input[0] & 0b00000100 == 0b00000000 {
                                    if covered_code_blocks[443] == 0 { new_block_reached = true; }
                                    covered_code_blocks[443] = 1;
                                }
                                    if input[72] & 0b00000001 == 0b00000001 {
                                    if covered_code_blocks[444] == 0 { new_block_reached = true; }
                                    covered_code_blocks[444] = 1;
                                        if input[70] & 0b00001000 == 0b00000000 {
                                        if covered_code_blocks[445] == 0 { new_block_reached = true; }
                                        covered_code_blocks[445] = 1;
                                    }
                                }
                            }
                                if input[87] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[446] == 0 { new_block_reached = true; }
                                covered_code_blocks[446] = 1;
                                    if input[10] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[447] == 0 { new_block_reached = true; }
                                    covered_code_blocks[447] = 1;
                                        if input[67] & 0b00000110 == 0b00000110 {
                                        if covered_code_blocks[448] == 0 { new_block_reached = true; }
                                        covered_code_blocks[448] = 1;
                                    }
                                }
                            }
                                if input[125] & 0b01000000 == 0b01000000 {
                                if covered_code_blocks[449] == 0 { new_block_reached = true; }
                                covered_code_blocks[449] = 1;
                            }
                                if input[23] & 0b00000011 == 0b00000001 {
                                if covered_code_blocks[450] == 0 { new_block_reached = true; }
                                covered_code_blocks[450] = 1;
                            }
                        }
                            if input[100] & 0b10000000 == 0b00000000 {
                            if covered_code_blocks[451] == 0 { new_block_reached = true; }
                            covered_code_blocks[451] = 1;
                        }
                    }
                        if input[55] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[452] == 0 { new_block_reached = true; }
                        covered_code_blocks[452] = 1;
                            if input[126] & 0b00000100 == 0b00000100 {
                            if covered_code_blocks[453] == 0 { new_block_reached = true; }
                            covered_code_blocks[453] = 1;
                                if input[94] & 0b00001000 == 0b00001000 {
                                if covered_code_blocks[454] == 0 { new_block_reached = true; }
                                covered_code_blocks[454] = 1;
                            }
                                if input[29] & 0b00000001 == 0b00000001 {
                                if covered_code_blocks[455] == 0 { new_block_reached = true; }
                                covered_code_blocks[455] = 1;
                            }
                                if input[120] & 0b00100000 == 0b00100000 {
                                if covered_code_blocks[456] == 0 { new_block_reached = true; }
                                covered_code_blocks[456] = 1;
                            }
                        }
                            if input[79] & 0b00000010 == 0b00000010 {
                            if covered_code_blocks[457] == 0 { new_block_reached = true; }
                            covered_code_blocks[457] = 1;
                        }
                    }
                        if input[48] & 0b00000001 == 0b00000000 {
                        if covered_code_blocks[458] == 0 { new_block_reached = true; }
                        covered_code_blocks[458] = 1;
                            if input[116] & 0b01100000 == 0b00100000 {
                            if covered_code_blocks[459] == 0 { new_block_reached = true; }
                            covered_code_blocks[459] = 1;
                        }
                    }
                }
            }
                if input[8] & 0b10000000 == 0b00000000 {
                if covered_code_blocks[460] == 0 { new_block_reached = true; }
                covered_code_blocks[460] = 1;
            }
                if input[106] & 0b00110000 == 0b00000000 {
                if covered_code_blocks[461] == 0 { new_block_reached = true; }
                covered_code_blocks[461] = 1;
            }
                if input[70] & 0b00000100 == 0b00000000 {
                if covered_code_blocks[462] == 0 { new_block_reached = true; }
                covered_code_blocks[462] = 1;
                    if input[123] & 0b00001000 == 0b00000000 {
                    if covered_code_blocks[463] == 0 { new_block_reached = true; }
                    covered_code_blocks[463] = 1;
                }
                    if input[67] & 0b00000001 == 0b00000001 {
                    if covered_code_blocks[464] == 0 { new_block_reached = true; }
                    covered_code_blocks[464] = 1;
                }
            }
                if input[123] & 0b00000001 == 0b00000000 {
                if covered_code_blocks[465] == 0 { new_block_reached = true; }
                covered_code_blocks[465] = 1;
                    if input[118] & 0b00000001 == 0b00000000 {
                    if covered_code_blocks[466] == 0 { new_block_reached = true; }
                    covered_code_blocks[466] = 1;
                }
            }
        }
            if input[102] & 0b01000000 == 0b00000000 {
            if covered_code_blocks[467] == 0 { new_block_reached = true; }
            covered_code_blocks[467] = 1;
                if input[90] & 0b00000011 == 0b00000011 {
                if covered_code_blocks[468] == 0 { new_block_reached = true; }
                covered_code_blocks[468] = 1;
            }
                if input[108] & 0b00000010 == 0b00000000 {
                if covered_code_blocks[469] == 0 { new_block_reached = true; }
                covered_code_blocks[469] = 1;
            }
        }
            if input[113] & 0b00001000 == 0b00000000 {
            if covered_code_blocks[470] == 0 { new_block_reached = true; }
            covered_code_blocks[470] = 1;
                if input[104] & 0b00010000 == 0b00000000 {
                if covered_code_blocks[471] == 0 { new_block_reached = true; }
                covered_code_blocks[471] = 1;
            }
                if input[112] & 0b00100000 == 0b00100000 {
                if covered_code_blocks[472] == 0 { new_block_reached = true; }
                covered_code_blocks[472] = 1;
                    if input[88] & 0b00010000 == 0b00000000 {
                    if covered_code_blocks[473] == 0 { new_block_reached = true; }
                    covered_code_blocks[473] = 1;
                }
            }
        }
    }
        if input[75] & 0b00100000 == 0b00100000 {
        if covered_code_blocks[474] == 0 { new_block_reached = true; }
        covered_code_blocks[474] = 1;
    }
        if input[57] & 0b00000001 == 0b00000000 {
        if covered_code_blocks[475] == 0 { new_block_reached = true; }
        covered_code_blocks[475] = 1;
            if input[102] & 0b10000000 == 0b10000000 {
            if covered_code_blocks[476] == 0 { new_block_reached = true; }
            covered_code_blocks[476] = 1;
        }
    }
        if input[121] & 0b11000000 == 0b11000000 {
        if covered_code_blocks[477] == 0 { new_block_reached = true; }
        covered_code_blocks[477] = 1;
            if input[43] & 0b00001000 == 0b00001000 {
            if covered_code_blocks[478] == 0 { new_block_reached = true; }
            covered_code_blocks[478] = 1;
                if input[110] & 0b01000000 == 0b00000000 {
                if covered_code_blocks[479] == 0 { new_block_reached = true; }
                covered_code_blocks[479] = 1;
                    if input[32] & 0b01000000 == 0b01000000 {
                    if covered_code_blocks[480] == 0 { new_block_reached = true; }
                    covered_code_blocks[480] = 1;
                        if input[91] & 0b00000110 == 0b00000010 {
                        if covered_code_blocks[481] == 0 { new_block_reached = true; }
                        covered_code_blocks[481] = 1;
                            if input[48] & 0b00001000 == 0b00000000 {
                            if covered_code_blocks[482] == 0 { new_block_reached = true; }
                            covered_code_blocks[482] = 1;
                                if input[24] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[483] == 0 { new_block_reached = true; }
                                covered_code_blocks[483] = 1;
                            }
                        }
                            if input[42] & 0b00010000 == 0b00010000 {
                            if covered_code_blocks[484] == 0 { new_block_reached = true; }
                            covered_code_blocks[484] = 1;
                        }
                            if input[107] & 0b10000000 == 0b00000000 {
                            if covered_code_blocks[485] == 0 { new_block_reached = true; }
                            covered_code_blocks[485] = 1;
                        }
                            if input[76] & 0b00000001 == 0b00000001 {
                            if covered_code_blocks[486] == 0 { new_block_reached = true; }
                            covered_code_blocks[486] = 1;
                                if input[119] & 0b00010000 == 0b00000000 {
                                if covered_code_blocks[487] == 0 { new_block_reached = true; }
                                covered_code_blocks[487] = 1;
                            }
                                if input[53] & 0b00001000 == 0b00000000 {
                                if covered_code_blocks[488] == 0 { new_block_reached = true; }
                                covered_code_blocks[488] = 1;
                            }
                                if input[124] & 0b00100000 == 0b00000000 {
                                if covered_code_blocks[489] == 0 { new_block_reached = true; }
                                covered_code_blocks[489] = 1;
                            }
                                if input[68] & 0b00010000 == 0b00000000 {
                                if covered_code_blocks[490] == 0 { new_block_reached = true; }
                                covered_code_blocks[490] = 1;
                            }
                                if input[6] & 0b00100000 == 0b00100000 {
                                if covered_code_blocks[491] == 0 { new_block_reached = true; }
                                covered_code_blocks[491] = 1;
                                    if input[43] & 0b10000000 == 0b10000000 {
                                    if covered_code_blocks[492] == 0 { new_block_reached = true; }
                                    covered_code_blocks[492] = 1;
                                }
                            }
                                if input[82] & 0b00001000 == 0b00001000 {
                                if covered_code_blocks[493] == 0 { new_block_reached = true; }
                                covered_code_blocks[493] = 1;
                            }
                                if input[75] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[494] == 0 { new_block_reached = true; }
                                covered_code_blocks[494] = 1;
                            }
                                if input[15] & 0b00001000 == 0b00000000 {
                                if covered_code_blocks[495] == 0 { new_block_reached = true; }
                                covered_code_blocks[495] = 1;
                            }
                                if input[84] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[496] == 0 { new_block_reached = true; }
                                covered_code_blocks[496] = 1;
                            }
                                if input[105] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[497] == 0 { new_block_reached = true; }
                                covered_code_blocks[497] = 1;
                                    if input[127] & 0b00000010 == 0b00000010 {
                                    if covered_code_blocks[498] == 0 { new_block_reached = true; }
                                    covered_code_blocks[498] = 1;
                                }
                                    if input[107] & 0b00000001 == 0b00000000 {
                                    if covered_code_blocks[499] == 0 { new_block_reached = true; }
                                    covered_code_blocks[499] = 1;
                                }
                                    if input[46] & 0b11000000 == 0b00000000 {
                                    if covered_code_blocks[500] == 0 { new_block_reached = true; }
                                    covered_code_blocks[500] = 1;
                                        if input[118] & 0b10000000 == 0b00000000 {
                                        if covered_code_blocks[501] == 0 { new_block_reached = true; }
                                        covered_code_blocks[501] = 1;
                                            if input[15] & 0b00000100 == 0b00000100 {
                                            if covered_code_blocks[502] == 0 { new_block_reached = true; }
                                            covered_code_blocks[502] = 1;
                                                if input[6] & 0b10000000 == 0b10000000 {
                                                if covered_code_blocks[503] == 0 { new_block_reached = true; }
                                                covered_code_blocks[503] = 1;
                                                    if input[108] & 0b00000001 == 0b00000000 {
                                                    if covered_code_blocks[504] == 0 { new_block_reached = true; }
                                                    covered_code_blocks[504] = 1;
                                                        if input[113] & 0b00000100 == 0b00000000 {
                                                        if covered_code_blocks[505] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[505] = 1;
                                                    }
                                                }
                                            }
                                        }
                                            if input[126] & 0b00000001 == 0b00000001 {
                                            if covered_code_blocks[506] == 0 { new_block_reached = true; }
                                            covered_code_blocks[506] = 1;
                                        }
                                            if input[8] & 0b00000001 == 0b00000001 {
                                            if covered_code_blocks[507] == 0 { new_block_reached = true; }
                                            covered_code_blocks[507] = 1;
                                        }
                                    }
                                        if input[96] & 0b00100000 == 0b00100000 {
                                        if covered_code_blocks[508] == 0 { new_block_reached = true; }
                                        covered_code_blocks[508] = 1;
                                    }
                                        if input[49] & 0b00000100 == 0b00000100 {
                                        if covered_code_blocks[509] == 0 { new_block_reached = true; }
                                        covered_code_blocks[509] = 1;
                                            if input[13] & 0b10000000 == 0b00000000 {
                                            if covered_code_blocks[510] == 0 { new_block_reached = true; }
                                            covered_code_blocks[510] = 1;
                                                if input[114] & 0b00000001 == 0b00000000 {
                                                if covered_code_blocks[511] == 0 { new_block_reached = true; }
                                                covered_code_blocks[511] = 1;
                                            }
                                        }
                                            if input[3] & 0b00000010 == 0b00000010 {
                                            if covered_code_blocks[512] == 0 { new_block_reached = true; }
                                            covered_code_blocks[512] = 1;
                                                if input[21] & 0b10000000 == 0b10000000 {
                                                if covered_code_blocks[513] == 0 { new_block_reached = true; }
                                                covered_code_blocks[513] = 1;
                                            }
                                        }
                                            if input[9] & 0b10000000 == 0b00000000 {
                                            if covered_code_blocks[514] == 0 { new_block_reached = true; }
                                            covered_code_blocks[514] = 1;
                                                if input[57] & 0b00000010 == 0b00000010 {
                                                if covered_code_blocks[515] == 0 { new_block_reached = true; }
                                                covered_code_blocks[515] = 1;
                                                    if input[61] & 0b10000000 == 0b10000000 {
                                                    if covered_code_blocks[516] == 0 { new_block_reached = true; }
                                                    covered_code_blocks[516] = 1;
                                                        if input[110] & 0b00100000 == 0b00000000 {
                                                        if covered_code_blocks[517] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[517] = 1;
                                                    }
                                                        if input[45] & 0b10000000 == 0b10000000 {
                                                        if covered_code_blocks[518] == 0 { new_block_reached = true; }
                                                        covered_code_blocks[518] = 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                    if input[111] & 0b00000001 == 0b00000000 {
                                    if covered_code_blocks[519] == 0 { new_block_reached = true; }
                                    covered_code_blocks[519] = 1;
                                }
                            }
                                if input[105] & 0b00010000 == 0b00000000 {
                                if covered_code_blocks[520] == 0 { new_block_reached = true; }
                                covered_code_blocks[520] = 1;
                            }
                                if input[6] & 0b00010000 == 0b00010000 {
                                if covered_code_blocks[521] == 0 { new_block_reached = true; }
                                covered_code_blocks[521] = 1;
                                    if input[79] & 0b00000100 == 0b00000100 {
                                    if covered_code_blocks[522] == 0 { new_block_reached = true; }
                                    covered_code_blocks[522] = 1;
                                }
                                    if input[40] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[523] == 0 { new_block_reached = true; }
                                    covered_code_blocks[523] = 1;
                                        if input[119] & 0b00001000 == 0b00001000 {
                                        if covered_code_blocks[524] == 0 { new_block_reached = true; }
                                        covered_code_blocks[524] = 1;
                                    }
                                }
                            }
                                if input[99] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[525] == 0 { new_block_reached = true; }
                                covered_code_blocks[525] = 1;
                                    if input[37] & 0b01000000 == 0b01000000 {
                                    if covered_code_blocks[526] == 0 { new_block_reached = true; }
                                    covered_code_blocks[526] = 1;
                                }
                                    if input[19] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[527] == 0 { new_block_reached = true; }
                                    covered_code_blocks[527] = 1;
                                }
                                    if input[92] & 0b10000000 == 0b00000000 {
                                    if covered_code_blocks[528] == 0 { new_block_reached = true; }
                                    covered_code_blocks[528] = 1;
                                }
                                    if input[41] & 0b01000000 == 0b00000000 {
                                    if covered_code_blocks[529] == 0 { new_block_reached = true; }
                                    covered_code_blocks[529] = 1;
                                }
                            }
                        }
                            if input[66] & 0b00000100 == 0b00000000 {
                            if covered_code_blocks[530] == 0 { new_block_reached = true; }
                            covered_code_blocks[530] = 1;
                                if input[80] & 0b10000000 == 0b00000000 {
                                if covered_code_blocks[531] == 0 { new_block_reached = true; }
                                covered_code_blocks[531] = 1;
                            }
                                if input[121] & 0b00001000 == 0b00001000 {
                                if covered_code_blocks[532] == 0 { new_block_reached = true; }
                                covered_code_blocks[532] = 1;
                            }
                                if input[46] & 0b00000001 == 0b00000000 {
                                if covered_code_blocks[533] == 0 { new_block_reached = true; }
                                covered_code_blocks[533] = 1;
                            }
                        }
                    }
                        if input[97] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[534] == 0 { new_block_reached = true; }
                        covered_code_blocks[534] = 1;
                    }
                        if input[111] & 0b10000000 == 0b00000000 {
                        if covered_code_blocks[535] == 0 { new_block_reached = true; }
                        covered_code_blocks[535] = 1;
                    }
                        if input[44] & 0b00010000 == 0b00010000 {
                        if covered_code_blocks[536] == 0 { new_block_reached = true; }
                        covered_code_blocks[536] = 1;
                    }
                        if input[70] & 0b00000001 == 0b00000001 {
                        if covered_code_blocks[537] == 0 { new_block_reached = true; }
                        covered_code_blocks[537] = 1;
                    }
                        if input[19] & 0b00100000 == 0b00000000 {
                        if covered_code_blocks[538] == 0 { new_block_reached = true; }
                        covered_code_blocks[538] = 1;
                            if input[111] & 0b00010000 == 0b00010000 {
                            if covered_code_blocks[539] == 0 { new_block_reached = true; }
                            covered_code_blocks[539] = 1;
                                if input[67] & 0b01000000 == 0b01000000 {
                                if covered_code_blocks[540] == 0 { new_block_reached = true; }
                                covered_code_blocks[540] = 1;
                            }
                        }
                            if input[116] & 0b10000000 == 0b10000000 {
                            if covered_code_blocks[541] == 0 { new_block_reached = true; }
                            covered_code_blocks[541] = 1;
                        }
                            if input[26] & 0b00100000 == 0b00100000 {
                            if covered_code_blocks[542] == 0 { new_block_reached = true; }
                            covered_code_blocks[542] = 1;
                                if input[99] & 0b01000000 == 0b00000000 {
                                if covered_code_blocks[543] == 0 { new_block_reached = true; }
                                covered_code_blocks[543] = 1;
                                    if input[119] & 0b00000001 == 0b00000000 {
                                    if covered_code_blocks[544] == 0 { new_block_reached = true; }
                                    covered_code_blocks[544] = 1;
                                        if input[3] & 0b00000001 == 0b00000001 {
                                        if covered_code_blocks[545] == 0 { new_block_reached = true; }
                                        covered_code_blocks[545] = 1;
                                    }
                                }
                                    if input[116] & 0b00010000 == 0b00000000 {
                                    if covered_code_blocks[546] == 0 { new_block_reached = true; }
                                    covered_code_blocks[546] = 1;
                                }
                                    if input[63] & 0b01000000 == 0b01000000 {
                                    if covered_code_blocks[547] == 0 { new_block_reached = true; }
                                    covered_code_blocks[547] = 1;
                                        if input[39] & 0b10000000 == 0b00000000 {
                                        if covered_code_blocks[548] == 0 { new_block_reached = true; }
                                        covered_code_blocks[548] = 1;
                                    }
                                }
                                    if input[16] & 0b01000000 == 0b01000000 {
                                    if covered_code_blocks[549] == 0 { new_block_reached = true; }
                                    covered_code_blocks[549] = 1;
                                }
                            }
                        }
                    }
                }
                    if input[93] & 0b00000001 == 0b00000001 {
                    if covered_code_blocks[550] == 0 { new_block_reached = true; }
                    covered_code_blocks[550] = 1;
                }
            }
                if input[95] & 0b10000000 == 0b00000000 {
                if covered_code_blocks[551] == 0 { new_block_reached = true; }
                covered_code_blocks[551] = 1;
            }
        }
    }
        if input[23] & 0b10000000 == 0b10000000 {
        if covered_code_blocks[552] == 0 { new_block_reached = true; }
        covered_code_blocks[552] = 1;
            if input[35] & 0b00000001 == 0b00000000 {
            if covered_code_blocks[553] == 0 { new_block_reached = true; }
            covered_code_blocks[553] = 1;
                if input[42] & 0b00000001 == 0b00000001 {
                if covered_code_blocks[554] == 0 { new_block_reached = true; }
                covered_code_blocks[554] = 1;
            }
                if input[60] & 0b00000001 == 0b00000001 {
                if covered_code_blocks[555] == 0 { new_block_reached = true; }
                covered_code_blocks[555] = 1;
            }
                if input[58] & 0b00100000 == 0b00000000 {
                if covered_code_blocks[556] == 0 { new_block_reached = true; }
                covered_code_blocks[556] = 1;
            }
        }
    }
        if input[10] & 0b00000001 == 0b00000001 {
        if covered_code_blocks[557] == 0 { new_block_reached = true; }
        covered_code_blocks[557] = 1;
            if input[18] & 0b01000000 == 0b00000000 {
            if covered_code_blocks[558] == 0 { new_block_reached = true; }
            covered_code_blocks[558] = 1;
        }
            if input[86] & 0b01000000 == 0b01000000 {
            if covered_code_blocks[559] == 0 { new_block_reached = true; }
            covered_code_blocks[559] = 1;
        }
    }
    new_block_reached
}
