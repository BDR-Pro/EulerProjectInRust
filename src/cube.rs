use std::collections::HashMap;
use itertools::Itertools; 

//calc time to run

pub fn main() {
    let start = std::time::Instant::now();
    let mut cubes: HashMap<String, Vec<u64>> = HashMap::new();

    // Assuming reasonable bounds after mathematical analysis, replace `10000` with an appropriate upper bound.
    for i in 345u64.. {
        let cube = i.pow(3); // No need for 'as u64', 'i' is already u64.
        let sorted_digits: String = cube.to_string().chars().sorted().collect();

        cubes.entry(sorted_digits).or_insert_with(Vec::new).push(cube);

        // Early stopping condition if we already found what we needed.
        if let Some((_, v)) = cubes.iter().find(|(_, v)| v.len() == 5) {
            println!("The smallest cube for which exactly five permutations of its digits are cubes is: {}", v.iter().min().unwrap());
            let duration = start.elapsed();
            println!("Time to run it: {:?}", duration);
            return; // Stop the loop and end the program once we found our answer.
        }
    }
    // This line only runs if the loop completes without finding an answer
    println!("No such cube found");
}
