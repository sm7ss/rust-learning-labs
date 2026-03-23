const MAX_GRADE: u32 = 100; 
const MIN_GRADE: u32 = 0;
const VALUE_GRADE_IF_MIN_MAX: u32 = 0;

fn max_min_grades_validation(grade: &mut[u32]) -> usize {
    let mut changes: usize = 0;
    
    for g in grade {
        if *g < MIN_GRADE {
            *g = VALUE_GRADE_IF_MIN_MAX;
            println!("Your grade value cannot be less than {} Your new grade: {}", MIN_GRADE, VALUE_GRADE_IF_MIN_MAX);
            changes+=1;
        } else if *g > MAX_GRADE {
            *g = VALUE_GRADE_IF_MIN_MAX;
            println!("Your grade value cannot be greater than {} Your new grade: {}", MAX_GRADE, VALUE_GRADE_IF_MIN_MAX);
            changes+=1;
        }
    }; 
    
    changes
}

fn main() {
    let mut grades: [u32; 10] = [85, 92, 78, 65, 95, 110, 72, 90, 68, 81];
    let change: usize = max_min_grades_validation(&mut grades);
    
    if change > 0 {
        println!("New grades: {:?}", grades);
    } else {
        println!("All grades are fine");
    }
    
    println!("=== Analyse Student Grades ==");
    
    let average: f64 = average_grade(&grades);
    let max: u32 = max_grade(&grades);
    let min: u32 = min_grade(&grades);
    let approved: usize = count_approved(&grades);
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
    let mut count: usize= 0;
    for &g in grade {
        match g {
            70..=100 => {count+=1}, 
            _ => {continue}
        }
    }
    count
}

//fn count_failed(grade: &[u32]) -> usize {}

//fn letter_note(grade: u32) -> char {}



