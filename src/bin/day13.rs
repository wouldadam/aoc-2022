use std::{cell::RefCell, rc::Rc};

/// Input is pairs of lists that represent a packet.
/// Pairs are line separated.
/// Compare each pair of packets with the rules:
/// Both ints: lower should come first.
/// Both list: Compare each item in list. Left should run out of items first.
/// One list: convert int to list and treat as above.
/// Part A:
/// Print the some of the indices (1-indexed) of correctly ordered packet pairs.
/// Part B:
/// Sort the packets and insert two extras [[2]] [[6]].
/// Print the sort indices of the two extra packets multiplied together.

#[derive(Debug)]
enum ListItem {
    Value(u32),
    List(Vec<RcListItem>, bool),
}

type RcListItem = Rc<RefCell<ListItem>>;

impl ListItem {
    fn list() -> RcListItem {
        Rc::new(RefCell::new(ListItem::List(vec![], false)))
    }

    fn value(value: u32) -> RcListItem {
        Rc::new(RefCell::new(ListItem::Value(value)))
    }

    fn add(&mut self, item: RcListItem) {
        match self {
            ListItem::Value(_) => panic!("Can't add an item to a value"),
            ListItem::List(ref mut list, _) => list.push(item),
        }
    }

    fn last_list(&mut self) -> RcListItem {
        match self {
            ListItem::Value(_) => panic!("List isn't a list"),
            ListItem::List(ref mut list, _) => list.last().unwrap().clone(),
        }
    }

    fn is_divider(&self) -> bool {
        match self {
            ListItem::Value(_) => false,
            ListItem::List(_, is_divider) => *is_divider,
        }
    }

    fn parse(s: &str, is_divider: bool) -> RcListItem {
        let chars = s.chars().collect::<Vec<_>>();
        let top = Rc::new(RefCell::new(ListItem::List(vec![], is_divider)));
        let mut stack = vec![top.clone()];

        let mut pos: usize = 1;

        while pos < chars.len() {
            let current = stack.last().unwrap().clone();
            if chars[pos] == '[' {
                current.borrow_mut().add(ListItem::list());
                stack.push(current.borrow_mut().last_list());

                pos += 1;
            } else if chars[pos] == ']' {
                stack.pop();
                pos += 1;
            } else if chars[pos] == ',' {
                pos += 1;
            } else {
                let end = chars[pos + 1..]
                    .iter()
                    .position(|c| *c == ',' || *c == ']')
                    .unwrap()
                    + pos
                    + 1;

                let x = &chars[pos..end].iter().collect::<String>();
                let value = x.parse::<u32>().unwrap();

                current.borrow_mut().add(ListItem::value(value));
                pos = end;
            }
        }

        top
    }

    #[allow(dead_code)]
    fn to_str(&self) -> String {
        match self {
            ListItem::Value(v) => v.to_string(),
            ListItem::List(l, _) => {
                let mut s = String::from("[");
                for i in l.iter() {
                    s += &i.borrow().to_str();
                    s += ","
                }
                s += "]";
                s
            }
        }
    }
}

fn ordering(left: RcListItem, right: RcListItem) -> std::cmp::Ordering {
    match (&*left.borrow(), &*right.borrow()) {
        (ListItem::Value(lv), ListItem::Value(rv)) => lv.cmp(rv),
        (ListItem::Value(lv), ListItem::List(_, _)) => {
            let wrapper = Rc::new(RefCell::new(ListItem::List(
                vec![ListItem::value(*lv)],
                false,
            )));
            ordering(wrapper, right.clone())
        }
        (ListItem::List(_, _), ListItem::Value(rv)) => {
            let wrapper = Rc::new(RefCell::new(ListItem::List(
                vec![ListItem::value(*rv)],
                false,
            )));
            ordering(left.clone(), wrapper)
        }
        (ListItem::List(ll, _), ListItem::List(rl, _)) => {
            for idx in 0..ll.len() {
                // Right side ran out of items
                if idx >= rl.len() {
                    return std::cmp::Ordering::Greater;
                }

                match ordering(ll[idx].clone(), rl[idx].clone()) {
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                }
            }

            // Left side ran out of items
            if ll.len() < rl.len() {
                return std::cmp::Ordering::Less;
            }

            // Inconclusive
            std::cmp::Ordering::Equal
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day13.txt");

    // Sum the indices of correctly ordered packet pairs
    let packet_pairs = input
        .split("\n\n")
        .map(|pair| pair.lines().collect::<Vec<_>>())
        .map(|pair| {
            (
                ListItem::parse(pair[0], false),
                ListItem::parse(pair[1], false),
            )
        })
        .collect::<Vec<_>>();

    let indices_sum = packet_pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| (idx + 1, ordering(pair.0.clone(), pair.1.clone())))
        .filter(|pair| pair.1 != std::cmp::Ordering::Greater)
        .map(|pair| pair.0)
        .sum::<usize>();

    println!("Indices sum of correctly ordered packets {}", indices_sum);

    // Sort the packets and add dividers
    let div_a = ListItem::parse("[[2]]", true);
    let div_b = ListItem::parse("[[6]]", true);

    let mut packets = packet_pairs
        .iter()
        .flat_map(|pair| [pair.0.clone(), pair.1.clone()])
        .collect::<Vec<_>>();
    packets.push(div_a);
    packets.push(div_b);

    packets.sort_by(|a, b| ordering(a.clone(), b.clone()));

    let divider_sum = packets
        .iter()
        .enumerate()
        .map(|p| (p.0 + 1, p.1))
        .filter(|p| p.1.borrow().is_divider())
        .map(|p| p.0)
        .product::<usize>();

    println!("Divider indices sum {}", divider_sum);
}
