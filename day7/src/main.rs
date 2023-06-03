use std::collections::HashMap;

use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}
impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size as u64, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("sample_input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => curr = tree.get(&curr)?.parent().unwrap().clone(),
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {}
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();

    dbg!(sum);

    Ok(())
}
