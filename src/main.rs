use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Game Starting...");
    let mut point = 0.00;
    loop {
        let mut rng = rand::thread_rng();
        let mut dif_lvl = rng.gen_range(1..5);

        println!("Diff LvL.{}", dif_lvl);

        if dif_lvl == 1 {
            let number1 = rng.gen_range(0..100);
            let number2 = rng.gen_range(0..100);
            let quest = number1 + number2;

            println!("Question is : {} + {}", number1, number2);

            let mut ans = String::new();
            stdin().read_line(&mut ans).expect("Error");
            let answer: i32 = ans.trim().parse().expect("Parse error");

            if answer == quest {
                point += 1.0;
            } else {
                println!("Wrong answer");
            }

            println!("New point {}", point);
        } else if dif_lvl == 2 {
            let number1 = rng.gen_range(100..200);
            let number2 = rng.gen_range(100..200);
            let quest = number1 + number2;

            println!("Question is : {} + {}", number1, number2);

            let mut ans = String::new();
            stdin().read_line(&mut ans).expect("Error");
            let answer: i32 = ans.trim().parse().expect("Parse error");

            if answer == quest {
                point += 2.0;
            } else {
                println!("Wrong answer");
            }

            println!("New point {}", point);
        } else if dif_lvl == 3 {
            let number1 = rng.gen_range(100..500);
            let number2 = rng.gen_range(100..500);
            let number3 = rng.gen_range(100..500);
            let quest = number1 + number2 + number3;

            println!("Question is : {} + {} + {}", number1, number2, number3);

            let mut ans = String::new();
            stdin().read_line(&mut ans).expect("Error");
            let answer: i32 = ans.trim().parse().expect("Parse error");

            if answer == quest {
                point += 10.3;
            } else {
                println!("Wrong answer");
            }

            println!("New point {}", point);
        } else if dif_lvl == 4 {
            let number1 = rng.gen_range(1..10);
            let number2 = rng.gen_range(1..10);
            let number3 = rng.gen_range(1..10);
            let number4 = rng.gen_range(1..10);
            let quest = number1 * number2 * number3 * number4;

            println!(
                "Question is : {} * {} * {} * {}",
                number1, number2, number3, number4
            );

            let mut ans = String::new();
            stdin().read_line(&mut ans).expect("Error");
            let answer: i32 = ans.trim().parse().expect("Parse error");

            if answer == quest {
                point += 43.2;
            } else {
                println!("Wrong answer");
            }

            println!("New point {}", point);
        }
    }
}
