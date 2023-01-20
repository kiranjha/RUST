fn main() {
    let num = vec![-1, 1, 0, -3, 3]; //1, 2, 3, 4  //-1,1,0,0,3
    let result = product_except_self(num);
    println!("Result: {:?}",result);
}
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut output: Vec<i32> = vec![1; len];
        let mut product = 1;
        let mut zero_count = 0;
        let mut product_non_zero = 1;
         
        for i in 0..len {
            product *= nums[i];
            if nums[i] == 0 {
                zero_count += 1;
            }
            if zero_count < 2 {
                if nums[i] == 0 {
                    continue;
                } else { 
                    product_non_zero *= nums[i];
                }
            }
        }
        println!("Product: {product}");
        for i in 0..len {
            if nums[i] == 0 {
                if zero_count < 2 {
                    output[i] = product_non_zero;
                } else {
                    output[i] = 0;
                }
            } else {
                output[i] = product / nums[i];
            }
        }
        output
}
// fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let len = nums.len();
//         let mut output: Vec<i32> = vec![1; len];
//         let mut product = 1;
//         for i in 0..len {
//             output[i] = product;
//             product *= nums[i];
//         }
//         product = 1;
//         for i in (0..len).rev() {
//             output[i] *= product;
//             product *= nums[i];
//         }
//         output
// }