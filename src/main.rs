use datastructures_in_rust::LeetCodeParser;

fn main() {
    let mut lt = LeetCodeParser::new();
    let range: Vec<i32> = lt.next_arg_as_vec_of_nums::<i32>().unwrap();
    let lower: i32 = lt.next_arg_as_num::<i32>().unwrap();
    let upper: i32 = lt.next_arg_as_num::<i32>().unwrap();
    println!("{}", Solution::count_range_sum(range, lower, upper));
}
