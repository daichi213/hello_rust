fn main() {
    // [データ型; サイズ]
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
    // println!("{}", nums.0);
    
    let nums: [f32; 3] = [1.2, 2.12, 3.142592654];
    println!("{:?}", nums);
    println!("{}", nums[0]);
}