// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
enum Position {
    MaintenanceCrews,
    MarketingDepartment,
    Manager,
    LineSupervisor,
    Kitchen,
    AssemblyTechnic,
}
// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    em_type: Position,
    employed: bool
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_determine(employee: &Employee) -> Result<(), String> {
    if employee.employed {
        match employee.em_type {
            Position::MaintenanceCrews => Ok(()),
            Position::MarketingDepartment => Ok(()),
            Position::Manager => Ok(()),
            _ => Err("Invalid position".to_owned())
        }
    } else { Err("Terminated".to_owned()) }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access(employee: &Employee) -> Result<(), String> {
    try_determine(employee)?;
    println!("access OK");
    Ok(())
}

fn main() {
    let manager = Employee{em_type: Position::Manager, employed: false};

    if let Err(e) = print_access(&manager) {
        println!("access denied {:?}", e)
    }
}