#[macro_export]
macro_rules! ja {
    ($result:expr) => {
        $result.unwrap()
    }
} 
#[macro_export]
macro_rules! nom {
    ($iter:expr) => {
        $iter.collect()
    }
}
#[macro_export]
macro_rules! such_table {
    ($table:expr => $type:ty) => {
        nom!($table.lines()
             .map(|l| nom!(l.split_whitespace()
                           .map(|n| ja!(n.parse::<$type>())))
             )
        )
    }
}
#[macro_export]
macro_rules! num_col {
    ($list:expr => $type:ty) => {
        nom!($list.lines().map(|l| ja!(l.parse::<$type>())))
    }
}
#[macro_export]
macro_rules! num_row {
    ($row:expr => $type:ty) => {
        nom!($row.split_whitespace().map(|l| ja!(l.parse::<$type>())))
    }
}
