// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// Example:
// Given nums = [2, 7, 11, 15], target = 9,
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

fn main()
{
    let input: [u16; 4] = [2, 7, 11, 15];
    let target = 9;
    let range = 0..=3;

    for idx in range {
        println!("{:?}", idx);
        search_range(&target, &input[idx], &input[idx..=3]);
    }
}

fn search_range(target: &u16, val: &u16, range: &[u16]) -> u16
{
    let mut idx: u16 = 0;

    let location = loop {
        let comparison = &range[idx as usize];

        if (comparison + val) == *target {
            println!("\t{:?} {:?}", comparison, val);
            break idx;
        }

        if idx < range.len() as u16{
            idx = idx + 1;
        }else{
            // this won't compile
            // break;
        }

    };

    location
}
