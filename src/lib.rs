#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub age: u32,
}

pub fn staff_system(mut employees: Vec<Employee>) -> Vec<Employee> {
    employees.sort_by(|a, b| b.name.cmp(&a.name));

    employees
        .into_iter()
        .filter(|e| e.age >= 18)
        .map(|e| Employee {
            name: e.name.to_uppercase(),
            age: e.age,
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod should {
    use super::*;

    fn get_initial_employee_list() -> Vec<Employee> {
        vec![
            Employee {
                name: "Max".into(),
                age: 17,
            },
            Employee {
                name: "Sepp".into(),
                age: 18,
            },
            Employee {
                name: "Nina".into(),
                age: 15,
            },
            Employee {
                name: "Mike".into(),
                age: 51,
            },
        ]
    }

    #[test]
    fn return_employees_older_than_18() {
        let employees = get_initial_employee_list();

        let result = staff_system(employees);

        assert!(result.iter().all(|r| r.age >= 18));
    }

    #[test]
    fn return_employees_sorted_by_names_in_descent() {
        let employees = get_initial_employee_list();

        let result = staff_system(employees);

        assert!(result.windows(2).all(|e| { e[0].name > e[1].name }));
    }

    #[test]
    fn return_employees_with_capitalized_names() {
        let employees = get_initial_employee_list();

        let result = staff_system(employees);

        assert!(result
            .into_iter()
            .all(|e| { e.name.chars().all(char::is_uppercase) }));
    }
}
