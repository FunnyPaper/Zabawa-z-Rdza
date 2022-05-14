enum OddEven {
    Odd(i128),
    Even(i128)
}

impl OddEven {
    // inline if
    fn determine(value: i128) -> OddEven {
        if value % 2 == 0 { OddEven::Even(value) } else { OddEven::Odd(value) }
    }

    // if let
    fn is_even_let(&self) -> bool {
        if let OddEven::Even(_) = self { true } else { false }
    }

    // match
    fn is_even_match(&self) -> bool {
        match self {
            OddEven::Even(_) => true,
            OddEven::Odd(_) => false,
        }
    }

    fn is_odd_let(&self) -> bool { !self.is_even_let() }
    fn is_odd_match(&self) -> bool { !self.is_even_match() }

    // match
    fn say(arr: &[OddEven]) {
            for (i, r) in arr.iter().enumerate() {
                match r {
                    OddEven::Odd(val) => {
                        println!("[{i}] Matched value - {val} - is odd");
                    },
                    OddEven::Even(val) => {
                        println!("[{i}] Matched value - {val} - is even");
                    },
            };
        };
    }
}

fn main() {
    OddEven::say(&[ 
        OddEven::determine(10), 
        OddEven::determine(11),
        OddEven::determine(12),
        OddEven::determine(13),
        OddEven::determine(14),
        ]
    );
}