enum GradesImputation {
    AVERAGE, 
    NEARBY, 
}

const MAX_GRADE: u32 = 100; 
const MIN_GRADE: u32 = 0;
const APPROVED: u32 = 70;
const STRATEGY_IMPUTATION_GRADES: GradesImputation = GradesImputation::AVERAGE;

fn avergae_imputation(grades: &[u32]) -> u32 {
    let sum: u32 = grades.iter()
        .filter(|&&n| n >= MIN_GRADE && n <= MAX_GRADE)
        .sum(); 
    let count: usize = grades.iter()
        .filter(|&&n| n >= MIN_GRADE && n <= MAX_GRADE)
        .count();
    sum / count as u32
}

fn nearby_imputation(grade_value: u32) -> u32 {
    let mut grade: u32 = 0;
    
    if grade_value > MAX_GRADE {
        println!("The nearest value of the max limit is {}. Your value was {}", MAX_GRADE, grade_value);
        grade = MAX_GRADE;
    } else if grade_value < MIN_GRADE {
        println!("The neareast calue of the min limit is {}. Your value was {}", MIN_GRADE, grade_value);
        grade = MIN_GRADE;
    }
    grade
}

fn max_min_grades_validation(grade: &mut[u32]) -> usize {
    let mut changes: usize = 0;
    let avg = avergae_imputation(&grade);
    
    for g in grade {
        if *g < MIN_GRADE || *g > MAX_GRADE {
            println!("Your value {g}");
            
            *g = match STRATEGY_IMPUTATION_GRADES {
                GradesImputation::NEARBY => nearby_imputation(*g),
                GradesImputation::AVERAGE => avg,
            };
            
            println!("Your new value {g}");
            
            changes += 1
        }
    }; 
    
    changes
}

fn main() {
    let grades: [u32; 10] = [85, 92, 78, 65, 95, 110, 72, 90, 68, 81];
    let mut copy_grades = grades; //this can be hard because its a copy
    
    let change: usize = max_min_grades_validation(&mut copy_grades);
    if change > 0 {
        println!("New grades: {:?}", copy_grades);
    } else {
        println!("All grades are fine");
    }
    
    let average: f64 = average_grade(&copy_grades);
    let max: u32 = max_grade(&copy_grades);
    let min: u32 = min_grade(&copy_grades);
    let approved: usize = count_approved(&copy_grades);
    let not_approved: usize = count_failed(&copy_grades);
    
    println!(r#"
    === Analysis Student Grades ===   
    
        Average Grade:.........{}
        Max Grade:.............{}
        Min Grade:.............{}
        Approved Grades:.......{}
        Not Approved Grades:...{}
    "#, average, max, min, approved, not_approved);
    letter_note(&copy_grades);
}

fn average_grade(grade: &[u32]) -> f64 {
    let sum: u32 = grade.iter().sum(); 
    let average: f64 = sum as f64 / grade.len() as f64;
    average
}

fn max_grade(grade: &[u32]) -> u32 {
    let max: u32 = *grade.iter().max().unwrap_or(&0);
    max
}

fn min_grade(grade: &[u32]) -> u32 {
    let min: u32 = *grade.iter().min().unwrap_or(&0);
    min
}

fn count_approved(grade: &[u32]) -> usize {
    let count: usize = grade.iter()
        .filter(|&&n| n >= APPROVED)
        .count();
    count
}

fn count_failed(grade: &[u32]) -> usize {
    let not_approved: usize = grade.iter()
        .filter(|&&n| n < APPROVED)
        .count();
    not_approved
}

fn letter_note(grade: &[u32]) {
    for g in grade {
        match g {
            90..=100 => println!("Your grade was {}, you're an 'A'", g),
            80..=89 => println!("Your grade was {}, you're a 'B'", g), 
            70..=79 => println!("Your grade was {}, you're a 'C'", g), 
            60..=69 => println!("Your grade was {}, you're a 'D'", g), 
            _ => println!("Your grade was {}, you're a 'F'", g),
        }
    };
}

