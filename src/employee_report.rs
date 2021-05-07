#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Employee<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Employee<'a> {
    fn is_legal(&self) -> bool {
        self.age >= 18
    }
}

fn setup_employees<'a>() -> Vec<Employee<'a>> {
    let employee_one = Employee {
        name: "Max",
        age: 17,
    };
    let employee_two = Employee {
        name: "Sepp",
        age: 18,
    };
    let employee_three = Employee {
        name: "Nina",
        age: 15,
    };
    let employee_four = Employee {
        name: "Mike",
        age: 51,
    };
    let employee_five = Employee {
        name: "John",
        age: 29,
    };

    vec![
        employee_one,
        employee_two,
        employee_three,
        employee_four,
        employee_five,
    ]
}

fn find_legal_employees<'a>() -> Vec<Employee<'a>> {
    let mut employees = setup_employees();
    employees.sort_by(|a, b| a.name.cmp(&b.name));

    employees
        .iter_mut()
        .filter(|employee| employee.is_legal())
        .map(|employee| {
            // let mut clone = employee.clone();
            // clone.name = clone.name.to_uppercase().as_str();
            // return clone;
            employee.name = employee.name.to_uppercase().as_str();
            return employee.to_owned();
        })
        .collect::<Vec<Employee>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retrieves_list_of_legal_employees() {
        let employee_two = Employee {
            name: "Sepp",
            age: 18,
        };

        let employee_four = Employee {
            name: "Mike",
            age: 51,
        };
        let employee_five = Employee {
            name: "John",
            age: 29,
        };

        let expected = vec![employee_five, employee_four, employee_two];

        let actual = find_legal_employees();
        assert_eq!(expected, actual);
    }

    #[test]
    fn orders_list_alphabetically() {
        let employee_five = Employee {
            name: "John",
            age: 29,
        };

        let employee_four = Employee {
            name: "Mike",
            age: 51,
        };
        let employee_two = Employee {
            name: "Sepp",
            age: 18,
        };

        let expected = vec![employee_five, employee_four, employee_two];
        let actual = find_legal_employees();

        assert_eq!(actual, expected);
    }
}
