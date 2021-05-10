#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Employee {
    name: String,
    age: u32,
}

impl Employee {
    fn is_legal(&self) -> bool {
        self.age >= 18
    }

    fn scream_name(&mut self) -> Self {
        Self {
            name: self.name.to_uppercase(),
            age: self.age,
        }
    }
}

enum Order {
    Ascending,
    Descending,
    Unordered,
}

enum NameCaseType {
    Screaming,
    Sentence,
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

fn sort_employees(order: Order, employees: &mut Vec<Employee>) {
    match order {
        Order::Ascending => {
            employees.sort_by(|a, b| a.name.cmp(&b.name));
        }
        Order::Descending => {
            employees.sort_by(|a, b| b.name.cmp(&a.name));
        }
        _ => (),
    }
}

fn find_legal_employees(order: Order, name_case_type: NameCaseType) -> Vec<Employee> {
    let mut employees = setup_employees();

    sort_employees(order, &mut employees);

    employees
        .iter_mut()
        .filter(|employee| employee.is_legal())
        .map(|employee| match name_case_type {
            NameCaseType::Screaming => {
                return employee.scream_name();
            }
            NameCaseType::Sentence => {
                return employee.clone();
            }
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

        let expected = vec![employee_two, employee_four, employee_five];

        let actual = find_legal_employees(Order::Unordered, NameCaseType::Sentence);
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

        let expected = vec![employee_five, employee_four, employee_two];
        let actual = find_legal_employees(Order::Ascending, NameCaseType::Sentence);

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
        let actual = find_legal_employees(Order::Ascending, NameCaseType::Screaming);

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_order_descending() {
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

        let expected = vec![employee_two, employee_four, employee_five];
        let actual = find_legal_employees(Order::Descending, NameCaseType::Sentence);

        assert_eq!(actual, expected);
    }
}
