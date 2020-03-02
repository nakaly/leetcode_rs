

fn main() {
    // let mut input = vec![2,0,2,1,1,0];
    // print!("> ");
    // sort_colors(&mut input);
    // for x in input.iter() {
    //     print!("{} ", x);
    // }
    // println!("");
    let  input = vec![1,1,1,0];
    println!("result {}", can_jump(input))
    
}

pub fn sort_colors(nums: &mut Vec<i32>) {

    let mut num_of_zero: usize = 0;
    let mut num_of_one: usize = 0;

    for x in nums.iter() {
        if 0 == *x {
            num_of_zero += 1
        } else if 1 == *x {
            num_of_one += 1
        } 
    }
    for i in 0..nums.len() {
        if i < num_of_zero {
            nums[i] = 0
        } else if i < num_of_zero + num_of_one {
            nums[i] = 1
        } else {
            nums[i] = 2
        }
    }
}
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut indeces :Vec<i32> = (1..nums[0]+1).collect();
    if nums.len() == 1 {
        return true
    } 
    while !indeces.is_empty() {
        let current_pos = indeces[0];
        let furthest:i32 = current_pos + nums[current_pos as usize] ;
        // println!("indeces: {:?}, current: {}, furthest: {}",indeces,  current_pos, furthest);
        if furthest >= (nums.len() - 1) as i32 {
            return true
        }
        if furthest > indeces[indeces.len()-1] {
            let addition :Vec<i32> = (current_pos+1..furthest+1).collect();
            indeces.extend(addition.iter().copied());
            //println!("addition: {:?}", addition);
        }
        indeces.remove(0);

    }
    return false

}

