// Function to check guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 7;

    let mut guess_count = 0;

    // Mutable guess variable (simulate user input)
    let mut guess = 10; // Start with a guess

    loop {
        guess_count += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! It took {} guesses.", guess_count);
            break;
        } else if result == 1 {
            println!("{} is too high", guess);
        } else {
            println!("{} is too low", guess);
        }

        // Simulate changing guess value (modify this as needed for testing)
        guess -= 1; // Adjust guess value (you can change this logic)
    }
}
