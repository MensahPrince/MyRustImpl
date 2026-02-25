use std::io;

enum Urgency {
    Low, 
    High, 
    Medium, 
    Critical
}

struct Patient{
    name: String,
    age: String,
    urgency: Urgency,
}

impl Patient {
    fn describe(&self){
        println!("name: {}, age: {}", self.name, self.age);
    

        match self.urgency {
            Urgency::Low => println!("Urgency Low"),
            Urgency::Medium => println!("Urgency Medium"),
            Urgency::High => println!("Urgency: High"),
            Urgency::Critical => println!("Urgency: Critical"),
        }
    }
}

fn main() {
    let mut patients: Vec<Patient> = Vec::new();

       
   //Variables to hold patient details
    let mut p_name = String::new();
    let mut p_age = String::new();

    println!("Enter Patients name: ");
    io::stdin().read_line(&mut p_name).expect("Failed to read Patient name");
     if p_name.is_empty() {
        println!("Cannot accept an empty field");
        return;
     }
    println!("Enter Patients age: ");
    io::stdin().read_line(&mut p_age).expect("Failed to read Patient age");
    if p_age.is_empty() {
        println!("Cannot accept an empty field");
        return;
    } 
    let patient_details = Patient {
        name: p_name,
        age: p_age,
        urgency: Urgency::Low,
    };

    patients.push(patient_details);
    


    for patient in &patients {
        patient.describe(); 
    }
}
