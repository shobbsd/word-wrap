#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Employee {
    name: String,
    age: u32,
}

impl Employee {
    fn is_legal(&self) -> bool {
        self.age >= 18
    }
}

enum Order {
    Ascending,
    Descending,
}

fn setup_employees() -> Vec<Employee> {
    let employee_one = Employee {
        name: "Max".to_owned(),
        age: 17,
    };
    let employee_two = Employee {
        name: "Sepp".to_owned(),
        age: 18,
    };
    let employee_three = Employee {
        name: "Nina".to_owned(),
        age: 15,
    };
    let employee_four = Employee {
        name: "Mike".to_owned(),
        age: 51,
    };
    let employee_five = Employee {
        name: "John".to_owned(),
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

fn find_legal_employees(order: Order) -> Vec<Employee> {
    let mut employees = setup_employees();

    match order {
        Order::Ascending => {
            employees.sort_by(|a, b| a.name.cmp(&b.name));
        }
        Order::Descending => {
            employees.sort_by(|a, b| b.name.cmp(&a.name));
        }
    };

    employees
        .iter_mut()
        .filter(|employee| employee.is_legal())
        .map(|employee| {
            employee.name = employee.name.to_uppercase();
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
            name: "Sepp".to_owned(),
            age: 18,
        };

        let employee_four = Employee {
            name: "Mike".to_owned(),
            age: 51,
        };
        let employee_five = Employee {
            name: "John".to_owned(),
            age: 29,
        };

        let expected = vec![employee_five, employee_four, employee_two]
            .iter_mut()
            .map(|employee| {
                employee.name = employee.name.to_uppercase();
                employee.clone()
            })
            .collect::<Vec<Employee>>();

        let actual = find_legal_employees(Order::Ascending);
        assert_eq!(expected, actual);
    }

    #[test]
    fn orders_list_alphabetically() {
        let employee_five = Employee {
            name: "John".to_owned(),
            age: 29,
        };

        let employee_four = Employee {
            name: "Mike".to_owned(),
            age: 51,
        };
        let employee_two = Employee {
            name: "Sepp".to_owned(),
            age: 18,
        };

        let expected = vec![employee_five, employee_four, employee_two]
            .iter_mut()
            .map(|employee| {
                employee.name = employee.name.to_uppercase();
                employee.clone()
            })
            .collect::<Vec<Employee>>();
        let actual = find_legal_employees(Order::Ascending);

        assert_eq!(actual, expected);
    }

    #[test]
    fn orders_capitalises_names() {
        let employee_five = Employee {
            name: "JOHN".to_owned(),
            age: 29,
        };

        let employee_four = Employee {
            name: "MIKE".to_owned(),
            age: 51,
        };
        let employee_two = Employee {
            name: "SEPP".to_owned(),
            age: 18,
        };

        let expected = vec![employee_five, employee_four, employee_two];
        let actual = find_legal_employees(Order::Ascending);

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_order_descending() {
        let employee_five = Employee {
            name: "JOHN".to_owned(),
            age: 29,
        };

        let employee_four = Employee {
            name: "MIKE".to_owned(),
            age: 51,
        };
        let employee_two = Employee {
            name: "SEPP".to_owned(),
            age: 18,
        };

        let expected = vec![employee_two, employee_four, employee_five];
        let actual = find_legal_employees(Order::Descending);

        assert_eq!(actual, expected);
    }
}
