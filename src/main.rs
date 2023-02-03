// use auth_service::movies::play;
fn main() {
    // println!("Inside main of test module");
    // play(String::from("Garam Masala"));
    let nums = vec![2,1,3,4];
    let length = nums.len();
    // solution(nums);
    let mut left = Vec::new();
    left[0] = 1;
    // let mut right = Vec::new();
    // right[nums.len()-1] = 1;
    // let mut answer = Vec::new();
    for (i, v) in [1..length].iter().enumerate() {
        left[i] = left[i-1]*nums[i-1];
        println!("{} at {:?}",i, v);
    }
}
// fn solution(nums: Vec<i32>){
//     // let mut left = Vec::new();
//     // left[0] = 1;
//     // let mut right = Vec::new();
//     // right[nums.len()-1] = 1;
//     // // let mut answer = Vec::new();
//     // for (i, v) in nums.iter().enumerate() {
//     //     left[i] = left[i-1]*nums[i-1];
//     //     println!("{} at {}",i, v);
//     // }
// }
