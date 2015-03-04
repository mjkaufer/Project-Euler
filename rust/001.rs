fn main() {
    
    let mut sum = 0;

    let mut counter = 0;

    while counter < 1000{
        if counter % 3 == 0 && counter % 5 != 0{
            sum+= counter;
        }

        counter+=3;
    }

    counter = 0;

    while counter < 1000{
        if counter % 5 == 0{
            sum+= counter;
        }

        counter+=5;
    }

    println!("Sum of all digits divisible by 3 or 5 under 1000 is {}", sum);
} 