#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub age: u32,
}

pub fn staff_system(employees: Vec<Employee>) -> Vec<Employee> {
    let mut filtered_employees = employees
        .into_iter()
        .filter(|e| e.age >= 18)
        .collect::<Vec<_>>();
    filtered_employees.sort_by(|a, b| a.name.cmp(&b.name));

    filtered_employees
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_epmloyees_older_than_18() {
        let employees = vec![
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
        ];

        let result = staff_system(employees.clone());

        assert_eq!(result, vec![employees[1].clone(), employees[3].clone()]);
    }

    #[test]
    fn return_epmloyees_sorted_by_names() {
        let employees = vec![
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
        ];

        let result = staff_system(employees.clone());

        assert_eq!(result, vec![employees[3].clone(), employees[1].clone()]);
    }
}
