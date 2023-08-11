fn main() {
 dbg!(std::mem::size_of::<i32>());
 dbg!(std::mem::size_of::<&i32>());
 dbg!(std::mem::size_of::<[i32; 4]>());
 dbg!(std::mem::size_of::<&[i32; 4]>());
 dbg!(std::mem::size_of::<&[i32]>());
}
