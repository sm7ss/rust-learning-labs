use std::any::type_name;

const MAX_GRADE: i32 = 100; 
const MIN_GRADE: i32 = 0;

fn data_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let grades: [i32; 10] = [85, 92, 78, 65, 95, 88, 72, 90, 68, 81];
    
    
    
    println!("=== Analyse Student Grades ==");
    
    let average: f64 = average_grade(&grades);
    let max: i32 = max_grade(&grades);
    let min: i32 = min_grade(&grades);
    let approved: usize = count_approved(&grades);
    
    
    data_type(&average);
    
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

fn min_grade(grade: &[i32]) -> i32 {
    let min: i32 = *grade.iter().min().unwrap_or(&0);
    min
}

fn count_approved(grade: &[i32]) -> usize {
    let mut count: usize= 0;
    for &g in grade {
        match g {
            70..=100 => {count+=1}, 
            _ => {continue}
        }
    }
    count
}

//fn count_failed(grade: &[i32]) -> usize {}

//fn letter_note(grade: i32) -> char {}



