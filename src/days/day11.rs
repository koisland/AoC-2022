use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::{fs, vec};

use itertools::Itertools;

use super::error::ParserError;

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Old,
    Value(usize),
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug)]
struct Statement {
    operation: Operation,
    operand_1: Operand,
    operand_2: Operand,
}

impl Statement {
    fn result(&self, input: Option<usize>) -> Result<usize, &'static str> {
        if (self.operand_1 == Operand::Old || self.operand_2 == Operand::Old) && input.is_none() {
            return Err("Cannot perform operation without an input value.");
        }
        // Set operands if some input is provided.
        let (mut op_1, mut op_2) = (self.operand_1.clone(), self.operand_2.clone());
        if let Some(input) = input {
            if self.operand_1 == Operand::Old {
                op_1 = Operand::Value(input);
            }
            if self.operand_2 == Operand::Old {
                op_2 = Operand::Value(input);
            }
        }
        if let (Operand::Value(v1), Operand::Value(v2)) = (op_1, op_2) {
            Ok(match self.operation {
                Operation::Add => v1 + v2,
                // Worry level for input item cannot be less than 0.
                Operation::Sub => v1.saturating_sub(v2),
                Operation::Mult => v1 * v2,
                Operation::Div => v1 / v2,
            })
        } else {
            Err("Value for op1 or op2 wasn't properly set.")
        }
    }
}

#[derive(Debug)]
struct ThrowTest {
    modulus: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug)]
struct Monkey {
    num: usize,
    items: Vec<usize>,
    inspected_items: usize,
    statement: Statement,
    throw_test: ThrowTest,
}
impl Monkey {
    pub fn new(behavior: &str) -> Result<Self, ParserError> {
        Monkey::_parse_behavior(behavior)
    }

    /// Parse the 2nd string from line split into two strings into a `usize`.
    /// * **Note**: Will return `Option::None` if multiple delimiters are present.
    ///
    /// Examples:
    /// * `(Monkey 0)`
    /// * `divisible (by 19)`
    /// * `If true: throw to (monkey 2)`
    fn _parse_num_label(delim: &str, line: &str) -> Option<usize> {
        if let Some((_, str_num)) = line.split(delim).collect_tuple::<(&str, &str)>() {
            return str_num.trim().parse::<usize>().ok();
        } else {
            None
        }
    }
    /// Parse monkey items from line.
    ///
    /// Ex. `Starting items: (54, 65, 75, 74)`
    fn _parse_items(line: &str) -> Option<Vec<usize>> {
        if let Some((_, str_items)) = line.split(':').collect_tuple::<(&str, &str)>() {
            return str_items
                .split(',')
                .map(|str_item| str_item.trim().parse::<usize>().ok())
                .collect();
        } else {
            None
        }
    }
    /// Parse monkey operation from line.
    /// * Panics if operation is unknown or values cannot be cast to `usize`.
    ///
    /// Ex. `Operation: new = old + 6`
    fn _parse_operation(line: &str) -> Result<Statement, ParserError> {
        if let Some((_, str_ops)) = line.split('=').collect_tuple::<(&str, &str)>() {
            let (op1, oper, op2) = str_ops
                .trim()
                .split(' ')
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap();

            let operation = match oper {
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Mult,
                "/" => Operation::Div,
                _ => {
                    return Err(ParserError {
                        reason: format!("Unknown operation '{oper}'."),
                    });
                }
            };
            let parsed_op1 = if op1 == "old" {
                Operand::Old
            } else {
                if let Ok(parsed_value) = op1.parse::<usize>() {
                    Operand::Value(parsed_value)
                } else {
                    return Err(ParserError {
                        reason: format!("Operand 1 ({op1}) cannot be converted to uint."),
                    });
                }
            };
            let parsed_op2 = if op2 == "old" {
                Operand::Old
            } else {
                if let Ok(parsed_value) = op2.parse::<usize>() {
                    Operand::Value(parsed_value)
                } else {
                    return Err(ParserError {
                        reason: format!("Operand 2 ({op2}) cannot be converted to uint."),
                    });
                }
            };

            Ok(Statement {
                operation,
                operand_1: parsed_op1,
                operand_2: parsed_op2,
            })
        } else {
            Err(ParserError {
                reason: "Missing '=' delimiter in line.".to_string(),
            })
        }
    }
    /// Parse monkey behavior into a `Monkey`.
    fn _parse_behavior(behavior: &str) -> Result<Monkey, ParserError> {
        let mut num: Option<usize> = None;
        let mut items: Option<Vec<usize>> = None;
        let mut stmt: Option<Statement> = None;

        let mut test_modulus: Option<usize> = None;
        let mut monkey_true: Option<usize> = None;
        let mut monkey_false: Option<usize> = None;

        for line in behavior.lines() {
            let line = line.trim();
            if line.starts_with("Monkey") {
                num = Monkey::_parse_num_label("Monkey", &line.replace(":", ""))
            } else if line.starts_with("Starting items") {
                items = Monkey::_parse_items(line)
            } else if line.starts_with("Operation") {
                stmt = Monkey::_parse_operation(line).ok()
            } else if line.starts_with("Test") {
                test_modulus = Monkey::_parse_num_label("by", line)
            } else if line.starts_with("If true") {
                monkey_true = Monkey::_parse_num_label("monkey", line)
            } else if line.starts_with("If false") {
                monkey_false = Monkey::_parse_num_label("monkey", line)
            }
        }
        if let (Some(div), Some(m_true), Some(m_false)) = (test_modulus, monkey_true, monkey_false)
        {
            if let (Some(n), Some(items), Some(stmt)) = (num, items, stmt) {
                Ok(Monkey {
                    num: n,
                    items,
                    inspected_items: 0,
                    statement: stmt,
                    throw_test: ThrowTest {
                        modulus: div,
                        on_true: m_true,
                        on_false: m_false,
                    },
                })
            } else {
                Err(ParserError {
                    reason: "Missing monkey num, items, or behavior operation.".to_string(),
                })
            }
        } else {
            Err(ParserError {
                reason: "Missing test modulus, true monkey, or false monkey from monkey behavior description.".to_string()
            })
        }
    }
}

#[derive(Debug)]
struct Barrel {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    modulus_multiple: usize
}

impl Barrel {
    fn new(fname: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(fname)?;
        let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];
        let mut moduli: Vec<usize> = vec![];
        for monkey_behavior in contents.split("\n\r\n") {
            let monkey = Rc::new(RefCell::new(Monkey::new(monkey_behavior)?));
            moduli.push(monkey.borrow().throw_test.modulus);
            monkeys.push(monkey)
        }

        // Calculate a multiple to use modular arithmetic on the worry level of an item so that the size of item remains manageable.
        Ok(Barrel { monkeys, modulus_multiple: moduli.iter().product()})
    }
    fn start_round(&mut self) {
        for monkey in self.monkeys.iter() {
            let mut monkey = monkey.borrow_mut();
            let mut items_inspected = 0;
            for item in monkey.items.iter() {
                // Worry level increased as monkey looks at item. Factor monkey's statement into worry level for item.
                let new_item = monkey.statement.result(Some(*item)).unwrap();

                // //  Not damaged so divide worry level by 3.
                // let worry_lvl = new_item / 3;

                // Part 2. Doesn't decrease.
                // Take the mod of the item's worry level against the product of all the monkey moduli.
                // We do this so that:
                //     * Our worry test will still work to select the next monkey.
                //     * We reduce the size of the worry levels of items so our unsigned values don't overflow.
                //     * See https://www.reddit.com/r/adventofcode/comments/zih7gf/2022_day_11_part_2_what_does_it_mean_find_another/j02eicp/.
                let worry_lvl = new_item % self.modulus_multiple;
                
                let next_monkey_idx = if worry_lvl % monkey.throw_test.modulus == 0 {
                    monkey.throw_test.on_true
                } else {
                    monkey.throw_test.on_false
                };
                // Pass item with new worry level to next monkey.
                if let Some(next_monkey) = self.monkeys.get(next_monkey_idx) {
                    next_monkey.borrow_mut().items.push(worry_lvl)
                }
                // Increment the number of inspected items.
                items_inspected += 1;
            }
            // Update monkey's inspected items.
            monkey.inspected_items += items_inspected;
            // After items moved to next item, remove items.
            monkey.items.clear()
        }
    }
    fn monkey_business(&mut self, n_rounds: usize) -> Result<usize, &'static str> {
        // Do n rounds of monkey business.
        for _ in 0..n_rounds {
            self.start_round();
        }
        let n_inspected_items = self.monkeys
            .iter()
            .map(|monkey| monkey.borrow().inspected_items)
            .sorted()
            .rev()
            .collect_vec();

        if let Some(top_two) = n_inspected_items.get(0..2) {
            Ok(top_two.iter().product())
        } else {
            Err("Not enough monkeys.")
        }
    }
}
pub fn monkey_business(fname: &str) -> Result<usize, Box<dyn Error>> {
    let mut barrel_of_monkeys = Barrel::new(fname)?;

    let monke_biz = barrel_of_monkeys.monkey_business(10000)?;
    Ok(monke_biz)
}
