use std::borrow::Borrow;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::rc::{Rc, Weak};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Value {
    Item(u32),
    List(Rc<Packet>),
}

impl Value {
    fn in_order(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Item(left_single_value), Value::Item(right_single_value)) => {
                // Not in order because left side value is larger than right side value.
                left_single_value <= right_single_value
            }
            /*
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            [[2,[3,[4,[5,6,7]]]],8,9, 1]
            */
            (Value::Item(_), Value::List(right_list_value)) => {
                // Recurse through in case multiple first items are Lists.
                // We need a single value for comparison.
                if let Some(inner_r) = right_list_value.items.borrow().get(0) {
                    Value::in_order(left, inner_r)
                } else {
                    // Left side has value so not in order.
                    false
                }
            }
            (Value::List(left_list_value), Value::Item(_)) => {
                // Recurse through in case multiple first items are Lists.
                // We need a single value for comparison.
                if let Some(inner_l) = left_list_value.items.borrow().get(0) {
                    Value::in_order(inner_l, right)
                } else {
                    // Right side has value so in order.
                    true
                }
            }
            (Value::List(left_list_value), Value::List(right_list_value)) => {
                for inner_items in left_list_value
                    .items
                    .borrow()
                    .iter()
                    .zip_longest(right_list_value.items.borrow().iter())
                {
                    match inner_items {
                        itertools::EitherOrBoth::Both(inner_l, inner_r) => {
                            if !Value::in_order(inner_l, inner_r) {
                                return false;
                            }
                        }
                        itertools::EitherOrBoth::Left(_) => {
                            return false;
                        }
                        itertools::EitherOrBoth::Right(_) => {
                            return true;
                        }
                    }
                }

                true
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Packet {
    parent: Option<Weak<Packet>>,
    items: RefCell<Vec<Rc<Value>>>,
}

impl Packet {
    fn new(str_list: &str) -> Rc<Packet> {
        // Ignore 1st and last character. [...]
        let str_list = str_list.get(1..(str_list.len() - 1)).unwrap();

        // Set root of list of packets.
        let root = Rc::new(Packet {
            parent: None,
            items: RefCell::new(vec![]),
        });

        let current: RefCell<Rc<Packet>> = RefCell::new(root.clone());
        for c in str_list.chars() {
            match c {
                '[' => {
                    let child = Rc::new(Packet {
                        parent: Some(Rc::downgrade(&*current.borrow())),
                        items: RefCell::new(vec![]),
                    });
                    current
                        .borrow()
                        .items
                        .borrow_mut()
                        .push(Rc::new(Value::List(child.clone())));

                    // Set child to current.
                    *current.borrow_mut() = child;
                }
                ',' => {}
                ']' => {
                    let parent_packet = current.borrow().parent.clone();
                    if let Some(Some(parent_packet)) = parent_packet.map(|parent| parent.upgrade())
                    {
                        *current.borrow_mut() = parent_packet.clone()
                    } else {
                        println!("Can't get parent.")
                    }
                }
                _ => {
                    if let Some(parsed_c) = c.to_digit(10) {
                        current
                            .borrow()
                            .items
                            .borrow_mut()
                            .push(Rc::new(Value::Item(parsed_c)))
                    } else {
                        println!("Can't parse {}", c)
                    }
                }
            }
        }

        root
    }

    fn get(&self, n: usize) -> Option<Rc<Value>> {
        self.items.borrow().get(n).cloned()
    }
}

pub fn distress_signal(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;

    let mut in_order_idx: Vec<usize> = vec![];

    'group_loop: for (i, packet_grp) in contents.split("\n\r\n").enumerate() {
        println!("Group: {i}");
        if let Some((packet_1, packet_2)) = packet_grp
            .lines()
            .map(|packet| Packet::new(packet))
            .collect_tuple::<(Rc<Packet>, Rc<Packet>)>()
        {
            // println!("{:?}", packet_1);
            // println!("{:?}", packet_2);
            for item in packet_1
                .items
                .borrow()
                .iter()
                .zip_longest(packet_2.items.borrow().iter())
            {
                match item {
                    itertools::EitherOrBoth::Both(left_item, right_item) => {
                        println!("\tLeft: {:?} - Right {:?}", left_item, right_item);
                        if !Value::in_order(left_item, right_item) {
                            continue 'group_loop;
                        }
                    }
                    itertools::EitherOrBoth::Left(_) => {
                        // Not in right order. Correct if left has less than right in # of items.
                        continue 'group_loop;
                    }
                    itertools::EitherOrBoth::Right(_) => {
                        println!("\tLeft ran out of items.");
                        in_order_idx.push(i + 1);
                        continue 'group_loop;
                    }
                }
            }
            // If no issue in comparisons and doesn't continue early, is in order.
            in_order_idx.push(i + 1);
        }
    }

    Ok(in_order_idx.iter().sum())
}

#[test]
fn test_distress_signal() {
    let test_file = "data/test_day_13_1.txt";
    let exp_sum_ord_indices: usize = 13;
    let res = distress_signal(test_file).unwrap();
    assert_eq!(res, exp_sum_ord_indices);
}
