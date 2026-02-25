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
            Urgency::Low => println!("Urgency: Low"),
            Urgency::Medium => println!("Urgency: Medium"),
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
    let mut p_decision = String::new();

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

    println!("Enter Patients condition.");
    println!("Enter (1) Low , (2) Medium, (3) High, (4) Critical");
    io::stdin().read_line(&mut p_decision).expect("Failed to read Urgency scale");
        
    let p_urgency = loop {
       match p_decision.trim() {
        "1" => break Urgency::Low,
        "2" => break Urgency::Medium,
        "3" => break Urgency::High,
        "4" => break Urgency::Critical,
        _ =>{
                println!("Enter either 1, 2, 3, or 4 as specified above to choose a scale.");
                p_decision.clear();
                io::stdin().read_line(&mut p_decision).expect("Failed to read Urgency scale");
        }
    }
 
    };
    

    let patient_details = Patient {
        name: p_name.trim().to_string(),
        age: p_age.trim().to_string(),
        urgency: p_urgency,
    };

    patients.push(patient_details);
    


    for patient in &patients {
        patient.describe(); 
    }
}
