macro_rules vec {
    ($x:expr) => {{
        let mut temp_vec = Vec::new();
        temp_vec.push($x);
        temp_vec
    }};
}

macro_rules multi_arg_vec{
    ($($x:expr), *) => { // `$($x:expr),*` の`*`は0回以上の繰り返しを意味する
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! vec {
    ($x:ty) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}