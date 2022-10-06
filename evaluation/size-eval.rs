//! This rust file was generated from the json.json grammar in the fzero_fuzzer repository.
//!
//! It was modified to enable evaluation by
//! 1. Changing main() to track the sizes of the generated output
//! 2. Replacing a hardcoded depth with the constant DEPTH_LIMIT
//!
//! Compile it with:
//! rustc -O -g size-eval.rs -C target-cpu=native -C lto=yes -C codegen-units=1 -o size-eval

#![allow(unused)]
use std::cell::Cell;
use std::time::Instant;

const DEPTH_LIMIT: usize = 512;

fn main() {
    let mut fuzzer = Fuzzer {
        seed:  Cell::new(0x34cc028e11b4f89c),
        buf:   Vec::new(),
    };
    
    let mut sizes: Vec<usize> = vec![0; 1024 * 1024 + 1];
    let mut generated = 0;

    for iters in 1u64..sizes.len() as u64 + 1 {
        fuzzer.buf.clear();
        fuzzer.fragment_22(0);
        generated += fuzzer.buf.len();
        sizes[iters as usize - 1] = fuzzer.buf.len();
    }

    sizes.sort();
    let median = sizes[sizes.len() / 2];
    let avg = generated / sizes.len();
    let min = sizes[0];
    let max = sizes[sizes.len() - 1];

    println!("depth: {} | samples: {} | min: {} | median: {} | avg: {} | max: {}", DEPTH_LIMIT, sizes.len(), min, median, avg, max);
}

struct Fuzzer {
    seed:  Cell<usize>,
    buf:   Vec<u8>,
}

impl Fuzzer {
    fn rand(&self) -> usize {
        let mut seed = self.seed.get();
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 43;
        self.seed.set(seed);
        seed
    }
    fn fragment_0(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_33(depth + 1),
            1 => self.fragment_37(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_1(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_2(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 94 {
            0 => self.fragment_43(depth + 1),
            1 => self.fragment_45(depth + 1),
            2 => self.fragment_47(depth + 1),
            3 => self.fragment_49(depth + 1),
            4 => self.fragment_51(depth + 1),
            5 => self.fragment_53(depth + 1),
            6 => self.fragment_55(depth + 1),
            7 => self.fragment_57(depth + 1),
            8 => self.fragment_59(depth + 1),
            9 => self.fragment_61(depth + 1),
            10 => self.fragment_63(depth + 1),
            11 => self.fragment_65(depth + 1),
            12 => self.fragment_67(depth + 1),
            13 => self.fragment_69(depth + 1),
            14 => self.fragment_71(depth + 1),
            15 => self.fragment_73(depth + 1),
            16 => self.fragment_75(depth + 1),
            17 => self.fragment_77(depth + 1),
            18 => self.fragment_79(depth + 1),
            19 => self.fragment_81(depth + 1),
            20 => self.fragment_83(depth + 1),
            21 => self.fragment_85(depth + 1),
            22 => self.fragment_87(depth + 1),
            23 => self.fragment_89(depth + 1),
            24 => self.fragment_91(depth + 1),
            25 => self.fragment_93(depth + 1),
            26 => self.fragment_95(depth + 1),
            27 => self.fragment_97(depth + 1),
            28 => self.fragment_99(depth + 1),
            29 => self.fragment_101(depth + 1),
            30 => self.fragment_103(depth + 1),
            31 => self.fragment_105(depth + 1),
            32 => self.fragment_107(depth + 1),
            33 => self.fragment_109(depth + 1),
            34 => self.fragment_111(depth + 1),
            35 => self.fragment_113(depth + 1),
            36 => self.fragment_115(depth + 1),
            37 => self.fragment_117(depth + 1),
            38 => self.fragment_119(depth + 1),
            39 => self.fragment_121(depth + 1),
            40 => self.fragment_123(depth + 1),
            41 => self.fragment_125(depth + 1),
            42 => self.fragment_127(depth + 1),
            43 => self.fragment_129(depth + 1),
            44 => self.fragment_131(depth + 1),
            45 => self.fragment_133(depth + 1),
            46 => self.fragment_135(depth + 1),
            47 => self.fragment_137(depth + 1),
            48 => self.fragment_139(depth + 1),
            49 => self.fragment_141(depth + 1),
            50 => self.fragment_143(depth + 1),
            51 => self.fragment_145(depth + 1),
            52 => self.fragment_147(depth + 1),
            53 => self.fragment_149(depth + 1),
            54 => self.fragment_151(depth + 1),
            55 => self.fragment_153(depth + 1),
            56 => self.fragment_155(depth + 1),
            57 => self.fragment_157(depth + 1),
            58 => self.fragment_159(depth + 1),
            59 => self.fragment_161(depth + 1),
            60 => self.fragment_163(depth + 1),
            61 => self.fragment_165(depth + 1),
            62 => self.fragment_167(depth + 1),
            63 => self.fragment_169(depth + 1),
            64 => self.fragment_171(depth + 1),
            65 => self.fragment_173(depth + 1),
            66 => self.fragment_175(depth + 1),
            67 => self.fragment_177(depth + 1),
            68 => self.fragment_179(depth + 1),
            69 => self.fragment_181(depth + 1),
            70 => self.fragment_183(depth + 1),
            71 => self.fragment_185(depth + 1),
            72 => self.fragment_187(depth + 1),
            73 => self.fragment_189(depth + 1),
            74 => self.fragment_191(depth + 1),
            75 => self.fragment_193(depth + 1),
            76 => self.fragment_195(depth + 1),
            77 => self.fragment_197(depth + 1),
            78 => self.fragment_199(depth + 1),
            79 => self.fragment_201(depth + 1),
            80 => self.fragment_203(depth + 1),
            81 => self.fragment_205(depth + 1),
            82 => self.fragment_207(depth + 1),
            83 => self.fragment_209(depth + 1),
            84 => self.fragment_211(depth + 1),
            85 => self.fragment_213(depth + 1),
            86 => self.fragment_215(depth + 1),
            87 => self.fragment_217(depth + 1),
            88 => self.fragment_219(depth + 1),
            89 => self.fragment_221(depth + 1),
            90 => self.fragment_223(depth + 1),
            91 => self.fragment_225(depth + 1),
            92 => self.fragment_227(depth + 1),
            93 => self.fragment_229(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_3(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_4(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_5(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_6(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_7(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_8(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_247(depth + 1);
        self.fragment_248(depth + 1);
    }
    fn fragment_9(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_250(depth + 1);
        self.fragment_251(depth + 1);
    }
    fn fragment_10(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 7 {
            0 => self.fragment_254(depth + 1),
            1 => self.fragment_256(depth + 1),
            2 => self.fragment_258(depth + 1),
            3 => self.fragment_260(depth + 1),
            4 => self.fragment_262(depth + 1),
            5 => self.fragment_264(depth + 1),
            6 => self.fragment_266(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_11(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 3 {
            0 => self.fragment_267(depth + 1),
            1 => self.fragment_271(depth + 1),
            2 => self.fragment_275(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_12(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_276(depth + 1),
            1 => self.fragment_279(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_13(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 4 {
            0 => self.fragment_281(depth + 1),
            1 => self.fragment_284(depth + 1),
            2 => self.fragment_287(depth + 1),
            3 => self.fragment_291(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_14(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_15(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_294(depth + 1);
        self.fragment_295(depth + 1);
        self.fragment_296(depth + 1);
        self.fragment_297(depth + 1);
        self.fragment_298(depth + 1);
    }
    fn fragment_16(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_300(depth + 1);
        self.fragment_301(depth + 1);
    }
    fn fragment_17(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_303(depth + 1);
        self.fragment_304(depth + 1);
        self.fragment_305(depth + 1);
    }
    fn fragment_18(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_310(depth + 1),
            1 => self.fragment_314(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_19(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 9 {
            0 => self.fragment_316(depth + 1),
            1 => self.fragment_318(depth + 1),
            2 => self.fragment_320(depth + 1),
            3 => self.fragment_322(depth + 1),
            4 => self.fragment_324(depth + 1),
            5 => self.fragment_326(depth + 1),
            6 => self.fragment_328(depth + 1),
            7 => self.fragment_330(depth + 1),
            8 => self.fragment_332(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_20(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 3 {
            0 => self.fragment_333(depth + 1),
            1 => self.fragment_335(depth + 1),
            2 => self.fragment_337(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_21(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 4 {
            0 => self.fragment_339(depth + 1),
            1 => self.fragment_341(depth + 1),
            2 => self.fragment_343(depth + 1),
            3 => self.fragment_345(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_22(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_23(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_348(depth + 1);
        self.fragment_349(depth + 1);
        self.fragment_350(depth + 1);
    }
    fn fragment_24(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_352(depth + 1),
            1 => self.fragment_355(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_25(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_356(depth + 1);
        self.fragment_357(depth + 1);
    }
    fn fragment_26(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_359(depth + 1),
            1 => self.fragment_362(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_27(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_363(depth + 1);
        self.fragment_364(depth + 1);
    }
    fn fragment_28(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 7 {
            0 => self.fragment_367(depth + 1),
            1 => self.fragment_369(depth + 1),
            2 => self.fragment_371(depth + 1),
            3 => self.fragment_373(depth + 1),
            4 => self.fragment_375(depth + 1),
            5 => self.fragment_377(depth + 1),
            6 => self.fragment_379(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_29(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_30(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([91].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_31(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_32(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([93].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_33(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_30(depth + 1);
        self.fragment_31(depth + 1);
        self.fragment_32(depth + 1);
    }
    fn fragment_34(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([91].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_35(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_247(depth + 1);
        self.fragment_248(depth + 1);
    }
    fn fragment_36(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([93].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_37(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_34(depth + 1);
        self.fragment_35(depth + 1);
        self.fragment_36(depth + 1);
    }
    fn fragment_38(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_39(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 94 {
            0 => self.fragment_43(depth + 1),
            1 => self.fragment_45(depth + 1),
            2 => self.fragment_47(depth + 1),
            3 => self.fragment_49(depth + 1),
            4 => self.fragment_51(depth + 1),
            5 => self.fragment_53(depth + 1),
            6 => self.fragment_55(depth + 1),
            7 => self.fragment_57(depth + 1),
            8 => self.fragment_59(depth + 1),
            9 => self.fragment_61(depth + 1),
            10 => self.fragment_63(depth + 1),
            11 => self.fragment_65(depth + 1),
            12 => self.fragment_67(depth + 1),
            13 => self.fragment_69(depth + 1),
            14 => self.fragment_71(depth + 1),
            15 => self.fragment_73(depth + 1),
            16 => self.fragment_75(depth + 1),
            17 => self.fragment_77(depth + 1),
            18 => self.fragment_79(depth + 1),
            19 => self.fragment_81(depth + 1),
            20 => self.fragment_83(depth + 1),
            21 => self.fragment_85(depth + 1),
            22 => self.fragment_87(depth + 1),
            23 => self.fragment_89(depth + 1),
            24 => self.fragment_91(depth + 1),
            25 => self.fragment_93(depth + 1),
            26 => self.fragment_95(depth + 1),
            27 => self.fragment_97(depth + 1),
            28 => self.fragment_99(depth + 1),
            29 => self.fragment_101(depth + 1),
            30 => self.fragment_103(depth + 1),
            31 => self.fragment_105(depth + 1),
            32 => self.fragment_107(depth + 1),
            33 => self.fragment_109(depth + 1),
            34 => self.fragment_111(depth + 1),
            35 => self.fragment_113(depth + 1),
            36 => self.fragment_115(depth + 1),
            37 => self.fragment_117(depth + 1),
            38 => self.fragment_119(depth + 1),
            39 => self.fragment_121(depth + 1),
            40 => self.fragment_123(depth + 1),
            41 => self.fragment_125(depth + 1),
            42 => self.fragment_127(depth + 1),
            43 => self.fragment_129(depth + 1),
            44 => self.fragment_131(depth + 1),
            45 => self.fragment_133(depth + 1),
            46 => self.fragment_135(depth + 1),
            47 => self.fragment_137(depth + 1),
            48 => self.fragment_139(depth + 1),
            49 => self.fragment_141(depth + 1),
            50 => self.fragment_143(depth + 1),
            51 => self.fragment_145(depth + 1),
            52 => self.fragment_147(depth + 1),
            53 => self.fragment_149(depth + 1),
            54 => self.fragment_151(depth + 1),
            55 => self.fragment_153(depth + 1),
            56 => self.fragment_155(depth + 1),
            57 => self.fragment_157(depth + 1),
            58 => self.fragment_159(depth + 1),
            59 => self.fragment_161(depth + 1),
            60 => self.fragment_163(depth + 1),
            61 => self.fragment_165(depth + 1),
            62 => self.fragment_167(depth + 1),
            63 => self.fragment_169(depth + 1),
            64 => self.fragment_171(depth + 1),
            65 => self.fragment_173(depth + 1),
            66 => self.fragment_175(depth + 1),
            67 => self.fragment_177(depth + 1),
            68 => self.fragment_179(depth + 1),
            69 => self.fragment_181(depth + 1),
            70 => self.fragment_183(depth + 1),
            71 => self.fragment_185(depth + 1),
            72 => self.fragment_187(depth + 1),
            73 => self.fragment_189(depth + 1),
            74 => self.fragment_191(depth + 1),
            75 => self.fragment_193(depth + 1),
            76 => self.fragment_195(depth + 1),
            77 => self.fragment_197(depth + 1),
            78 => self.fragment_199(depth + 1),
            79 => self.fragment_201(depth + 1),
            80 => self.fragment_203(depth + 1),
            81 => self.fragment_205(depth + 1),
            82 => self.fragment_207(depth + 1),
            83 => self.fragment_209(depth + 1),
            84 => self.fragment_211(depth + 1),
            85 => self.fragment_213(depth + 1),
            86 => self.fragment_215(depth + 1),
            87 => self.fragment_217(depth + 1),
            88 => self.fragment_219(depth + 1),
            89 => self.fragment_221(depth + 1),
            90 => self.fragment_223(depth + 1),
            91 => self.fragment_225(depth + 1),
            92 => self.fragment_227(depth + 1),
            93 => self.fragment_229(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_40(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_41(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_39(depth + 1);
        self.fragment_40(depth + 1);
    }
    fn fragment_42(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([48].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_43(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([48].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_44(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([49].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_45(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([49].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_46(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([50].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_47(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([50].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_48(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([51].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_49(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([51].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_50(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([52].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_51(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([52].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_52(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([53].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_53(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([53].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_54(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([54].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_55(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([54].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_56(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([55].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_57(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([55].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_58(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([56].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_59(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([56].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_60(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([57].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_61(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([57].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_62(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([97].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_63(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([97].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_64(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([98].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_65(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([98].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_66(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([99].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_67(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([99].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_68(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([100].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_69(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([100].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_70(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_71(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_72(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_73(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_74(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([103].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_75(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([103].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_76(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([104].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_77(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([104].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_78(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([105].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_79(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([105].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_80(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([106].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_81(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([106].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_82(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([107].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_83(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([107].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_84(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([108].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_85(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([108].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_86(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([109].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_87(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([109].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_88(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_89(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_90(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([111].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_91(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([111].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_92(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([112].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_93(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([112].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_94(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([113].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_95(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([113].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_96(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([114].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_97(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([114].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_98(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([115].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_99(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([115].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_100(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_101(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_102(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([117].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_103(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([117].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_104(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([118].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_105(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([118].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_106(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([119].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_107(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([119].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_108(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([120].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_109(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([120].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_110(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([121].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_111(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([121].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_112(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([122].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_113(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([122].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_114(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([65].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_115(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([65].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_116(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([66].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_117(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([66].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_118(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([67].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_119(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([67].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_120(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([68].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_121(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([68].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_122(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([69].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_123(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([69].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_124(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([70].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_125(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([70].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_126(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([71].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_127(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([71].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_128(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([72].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_129(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([72].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_130(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([73].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_131(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([73].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_132(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([74].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_133(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([74].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_134(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([75].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_135(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([75].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_136(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([76].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_137(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([76].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_138(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([77].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_139(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([77].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_140(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([78].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_141(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([78].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_142(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([79].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_143(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([79].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_144(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([80].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_145(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([80].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_146(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([81].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_147(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([81].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_148(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([82].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_149(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([82].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_150(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([83].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_151(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([83].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_152(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([84].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_153(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([84].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_154(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([85].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_155(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([85].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_156(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([86].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_157(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([86].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_158(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([87].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_159(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([87].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_160(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([88].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_161(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([88].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_162(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([89].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_163(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([89].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_164(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([90].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_165(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([90].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_166(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([33].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_167(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([33].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_168(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([35].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_169(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([35].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_170(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([36].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_171(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([36].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_172(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([37].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_173(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([37].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_174(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([38].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_175(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([38].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_176(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_177(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_178(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([40].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_179(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([40].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_180(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([41].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_181(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([41].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_182(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([42].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_183(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([42].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_184(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([43].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_185(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([43].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_186(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([44].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_187(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([44].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_188(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_189(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_190(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([46].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_191(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([46].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_192(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([47].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_193(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([47].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_194(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([58].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_195(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([58].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_196(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([59].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_197(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([59].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_198(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([60].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_199(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([60].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_200(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([61].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_201(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([61].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_202(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([62].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_203(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([62].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_204(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([63].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_205(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([63].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_206(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([64].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_207(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([64].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_208(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([91].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_209(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([91].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_210(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([93].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_211(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([93].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_212(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([94].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_213(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([94].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_214(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([95].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_215(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([95].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_216(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([96].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_217(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([96].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_218(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([123].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_219(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([123].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_220(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([124].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_221(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([124].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_222(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([125].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_223(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([125].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_224(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([126].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_225(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([126].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_226(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([32].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_227(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([32].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_228(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_250(depth + 1);
        self.fragment_251(depth + 1);
    }
    fn fragment_229(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_250(depth + 1);
        self.fragment_251(depth + 1);
    }
    fn fragment_230(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_231(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_232(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_233(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_234(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_235(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_236(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_234(depth + 1);
        self.fragment_235(depth + 1);
    }
    fn fragment_237(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([48].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_238(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([48].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_239(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 9 {
            0 => self.fragment_316(depth + 1),
            1 => self.fragment_318(depth + 1),
            2 => self.fragment_320(depth + 1),
            3 => self.fragment_322(depth + 1),
            4 => self.fragment_324(depth + 1),
            5 => self.fragment_326(depth + 1),
            6 => self.fragment_328(depth + 1),
            7 => self.fragment_330(depth + 1),
            8 => self.fragment_332(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_240(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 9 {
            0 => self.fragment_316(depth + 1),
            1 => self.fragment_318(depth + 1),
            2 => self.fragment_320(depth + 1),
            3 => self.fragment_322(depth + 1),
            4 => self.fragment_324(depth + 1),
            5 => self.fragment_326(depth + 1),
            6 => self.fragment_328(depth + 1),
            7 => self.fragment_330(depth + 1),
            8 => self.fragment_332(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_241(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_242(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_243(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_244(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 7 {
            0 => self.fragment_367(depth + 1),
            1 => self.fragment_369(depth + 1),
            2 => self.fragment_371(depth + 1),
            3 => self.fragment_373(depth + 1),
            4 => self.fragment_375(depth + 1),
            5 => self.fragment_377(depth + 1),
            6 => self.fragment_379(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_245(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_246(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_247(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_248(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_352(depth + 1),
            1 => self.fragment_355(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_249(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_247(depth + 1);
        self.fragment_248(depth + 1);
    }
    fn fragment_250(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([92].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_251(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 7 {
            0 => self.fragment_254(depth + 1),
            1 => self.fragment_256(depth + 1),
            2 => self.fragment_258(depth + 1),
            3 => self.fragment_260(depth + 1),
            4 => self.fragment_262(depth + 1),
            5 => self.fragment_264(depth + 1),
            6 => self.fragment_266(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_252(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_250(depth + 1);
        self.fragment_251(depth + 1);
    }
    fn fragment_253(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([92].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_254(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([92].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_255(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([98].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_256(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([98].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_257(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_258(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_259(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_260(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_261(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([114].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_262(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([114].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_263(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_264(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_265(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_266(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_267(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_268(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([69].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_269(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 3 {
            0 => self.fragment_333(depth + 1),
            1 => self.fragment_335(depth + 1),
            2 => self.fragment_337(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_270(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_271(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_268(depth + 1);
        self.fragment_269(depth + 1);
        self.fragment_270(depth + 1);
    }
    fn fragment_272(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_273(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 3 {
            0 => self.fragment_333(depth + 1),
            1 => self.fragment_335(depth + 1),
            2 => self.fragment_337(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_274(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_275(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_272(depth + 1);
        self.fragment_273(depth + 1);
        self.fragment_274(depth + 1);
    }
    fn fragment_276(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_277(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([46].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_278(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_279(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_277(depth + 1);
        self.fragment_278(depth + 1);
    }
    fn fragment_280(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_281(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_238(depth + 1),
            1 => self.fragment_240(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_282(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 9 {
            0 => self.fragment_316(depth + 1),
            1 => self.fragment_318(depth + 1),
            2 => self.fragment_320(depth + 1),
            3 => self.fragment_322(depth + 1),
            4 => self.fragment_324(depth + 1),
            5 => self.fragment_326(depth + 1),
            6 => self.fragment_328(depth + 1),
            7 => self.fragment_330(depth + 1),
            8 => self.fragment_332(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_283(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_284(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_282(depth + 1);
        self.fragment_283(depth + 1);
    }
    fn fragment_285(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_286(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_287(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_285(depth + 1);
        self.fragment_286(depth + 1);
    }
    fn fragment_288(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_289(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 9 {
            0 => self.fragment_316(depth + 1),
            1 => self.fragment_318(depth + 1),
            2 => self.fragment_320(depth + 1),
            3 => self.fragment_322(depth + 1),
            4 => self.fragment_324(depth + 1),
            5 => self.fragment_326(depth + 1),
            6 => self.fragment_328(depth + 1),
            7 => self.fragment_330(depth + 1),
            8 => self.fragment_332(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_290(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_233(depth + 1),
            1 => self.fragment_236(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_291(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_288(depth + 1);
        self.fragment_289(depth + 1);
        self.fragment_290(depth + 1);
    }
    fn fragment_292(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_293(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_294(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_295(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_348(depth + 1);
        self.fragment_349(depth + 1);
        self.fragment_350(depth + 1);
    }
    fn fragment_296(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_297(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([58].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_298(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_299(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_294(depth + 1);
        self.fragment_295(depth + 1);
        self.fragment_296(depth + 1);
        self.fragment_297(depth + 1);
        self.fragment_298(depth + 1);
    }
    fn fragment_300(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_294(depth + 1);
        self.fragment_295(depth + 1);
        self.fragment_296(depth + 1);
        self.fragment_297(depth + 1);
        self.fragment_298(depth + 1);
    }
    fn fragment_301(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_359(depth + 1),
            1 => self.fragment_362(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_302(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_300(depth + 1);
        self.fragment_301(depth + 1);
    }
    fn fragment_303(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 4 {
            0 => self.fragment_281(depth + 1),
            1 => self.fragment_284(depth + 1),
            2 => self.fragment_287(depth + 1),
            3 => self.fragment_291(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_304(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_276(depth + 1),
            1 => self.fragment_279(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_305(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 3 {
            0 => self.fragment_267(depth + 1),
            1 => self.fragment_271(depth + 1),
            2 => self.fragment_275(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_306(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_303(depth + 1);
        self.fragment_304(depth + 1);
        self.fragment_305(depth + 1);
    }
    fn fragment_307(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([123].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_308(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_309(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([125].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_310(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_307(depth + 1);
        self.fragment_308(depth + 1);
        self.fragment_309(depth + 1);
    }
    fn fragment_311(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([123].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_312(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_300(depth + 1);
        self.fragment_301(depth + 1);
    }
    fn fragment_313(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([125].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_314(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_311(depth + 1);
        self.fragment_312(depth + 1);
        self.fragment_313(depth + 1);
    }
    fn fragment_315(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([49].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_316(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([49].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_317(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([50].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_318(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([50].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_319(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([51].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_320(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([51].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_321(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([52].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_322(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([52].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_323(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([53].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_324(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([53].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_325(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([54].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_326(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([54].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_327(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([55].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_328(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([55].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_329(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([56].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_330(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([56].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_331(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([57].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_332(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([57].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_333(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_334(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([43].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_335(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([43].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_336(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_337(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([45].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_338(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([32].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_339(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([32].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_340(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([10].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_341(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([10].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_342(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([9].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_343(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([9].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_344(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([13].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_345(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([13].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_346(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_347(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_243(depth + 1);
        self.fragment_244(depth + 1);
        self.fragment_245(depth + 1);
    }
    fn fragment_348(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_349(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_38(depth + 1),
            1 => self.fragment_41(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_350(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([34].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_351(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_348(depth + 1);
        self.fragment_349(depth + 1);
        self.fragment_350(depth + 1);
    }
    fn fragment_352(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_353(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_356(depth + 1);
        self.fragment_357(depth + 1);
    }
    fn fragment_354(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_352(depth + 1),
            1 => self.fragment_355(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_355(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_353(depth + 1);
        self.fragment_354(depth + 1);
    }
    fn fragment_356(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([44].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_357(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_247(depth + 1);
        self.fragment_248(depth + 1);
    }
    fn fragment_358(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_356(depth + 1);
        self.fragment_357(depth + 1);
    }
    fn fragment_359(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
    fn fragment_360(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_363(depth + 1);
        self.fragment_364(depth + 1);
    }
    fn fragment_361(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_359(depth + 1),
            1 => self.fragment_362(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_362(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_360(depth + 1);
        self.fragment_361(depth + 1);
    }
    fn fragment_363(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 1;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([44].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 1);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_364(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_300(depth + 1);
        self.fragment_301(depth + 1);
    }
    fn fragment_365(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_363(depth + 1);
        self.fragment_364(depth + 1);
    }
    fn fragment_366(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_310(depth + 1),
            1 => self.fragment_314(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_367(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_310(depth + 1),
            1 => self.fragment_314(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_368(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_33(depth + 1),
            1 => self.fragment_37(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_369(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_33(depth + 1),
            1 => self.fragment_37(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_370(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_348(depth + 1);
        self.fragment_349(depth + 1);
        self.fragment_350(depth + 1);
    }
    fn fragment_371(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_348(depth + 1);
        self.fragment_349(depth + 1);
        self.fragment_350(depth + 1);
    }
    fn fragment_372(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_303(depth + 1);
        self.fragment_304(depth + 1);
        self.fragment_305(depth + 1);
    }
    fn fragment_373(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_303(depth + 1);
        self.fragment_304(depth + 1);
        self.fragment_305(depth + 1);
    }
    fn fragment_374(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 4;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116, 114, 117, 101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 4);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_375(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 4;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([116, 114, 117, 101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 4);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_376(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 5;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102, 97, 108, 115, 101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 5);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_377(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 5;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([102, 97, 108, 115, 101].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 5);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_378(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 4;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110, 117, 108, 108].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 4);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_379(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }

            unsafe {
                let old_size = self.buf.len();
                let new_size = old_size + 4;

                if new_size > self.buf.capacity() {
                    self.buf.reserve(new_size - old_size);
                }

                std::ptr::copy_nonoverlapping([110, 117, 108, 108].as_ptr(), self.buf.as_mut_ptr().offset(old_size as isize), 4);
                self.buf.set_len(new_size);
            }
        }
    fn fragment_380(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 4 {
            0 => self.fragment_339(depth + 1),
            1 => self.fragment_341(depth + 1),
            2 => self.fragment_343(depth + 1),
            3 => self.fragment_345(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_381(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        match self.rand() % 2 {
            0 => self.fragment_382(depth + 1),
            1 => self.fragment_383(depth + 1),
            _ => unreachable!(),
        }
    }
    fn fragment_382(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
        self.fragment_380(depth + 1);
        self.fragment_381(depth + 1);
    }
    fn fragment_383(&mut self, depth: usize) {
        if depth >= DEPTH_LIMIT { return; }
    }
}
