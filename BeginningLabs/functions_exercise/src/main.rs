enum GradesImputation {
    AVERAGE, 
    NEARBY, 
}

const MAX_GRADE: u32 = 100; 
const MIN_GRADE: u32 = 0;
const STRATEGY_IMPUTATION_GRADES: GradesImputation = GradesImputation::AVERAGE;

fn avergae_imputation(grades: &mut [u32]) -> u32 {
    let sum: u32 = grades.iter()
        .filter(|&&n| n >= MIN_GRADE && n <= MAX_GRADE)
        .sum(); 
    let count: u32 = grades.iter()
        .filter(|&&n| n >= MIN_GRADE && n <= MAX_GRADE)
        .count();
    
}

fn max_min_grades_validation(grade: &mut[u32]) -> usize {
    let mut changes: usize = 0;
    
    for g in grade {
        if *g < MIN_GRADE || *g > MAX_GRADE {
            println!("Your value {g}");
            *g = 0;
            
            
            *g = avergae_imputation(grade);
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
        println!("New grades: {:?}", grades);
    } else {
        println!("All grades are fine");
    }
    
    println!("=== Analyse Student Grades ==");
    
    //let average: f64 = average_grade(&grades);
    //let max: u32 = max_grade(&grades);
    //let min: u32 = min_grade(&grades);
    //let approved: usize = count_approved(&grades);
}


//fn count_failed(grade: &[u32]) -> usize {}

//fn letter_note(grade: u32) -> char {}



