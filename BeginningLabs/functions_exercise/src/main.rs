fn main() {
    let grades: [i32; 10] = [85, 92, 78, 65, 95, 88, 72, 90, 68, 81];
    
    println!("=== Analyse Student Grades ==");
    
    let average: f64 = average_grade(&grades);
    let max: i32 = max_grade(&grades);
    
}

fn average_grade(grade: &[i32]) -> f64 {
    let sum: i32 = grade.iter().sum(); 
    let average: f64 = sum as f64 / grade.len() as f64;
    average
}

fn max_grade(grade: &[i32]) -> i32 {
    let max: i32 = *grade.iter().max().unwrap_or(&0);
    max
}

//fn min_grade(grade: &[i32]) -> i32 {}

//fn count_approved(grade: &[i32]) -> usize {}

//fn count_failed(grade: &[i32]) -> usize {}

//fn letter_note(grade: i32) -> char {}



