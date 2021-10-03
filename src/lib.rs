#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub age: u32,
}

pub fn staff_system(_employees: Vec<Employee>) -> Vec<Employee> {
    todo!()
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_epmloyees_olter_than_18() {
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
}
