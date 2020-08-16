
use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "basic",  about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long, default_value="666")]
    speed: f64,

    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    #[structopt(short, long)]
    level: Vec<String>,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(Debug, StructOpt, Copy, Clone)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Git {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "i")]
        interactive: bool,

        #[structopt(short = "p")]
        patch: bool
    },

    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,

        #[structopt(long = "all")]
        all: bool
    },

    #[structopt(name = "commit")]
    Commit {
        #[structopt(short = "a")]
        all: bool
    }
}

// 类型桥
struct GitIntoIterator {
    index: usize,
}

// 实现自定义打印信息
impl fmt::Display for Git {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Git data={:?}", self)
    }
}

// 迭代器转化
impl IntoIterator for Git {
    type Item = Git;
    type IntoIter = GitIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        GitInto
use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "basic",  about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long, default_value="666")]
    speed: f64,

    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    #[structopt(short, long)]
    level: Vec<String>,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(Debug, StructOpt, Copy, Clone)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Git {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "i")]
        interactive: bool,

        #[structopt(short = "p")]
        patch: bool
    },

    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,

        #[structopt(long = "all")]
        all: bool
    },

    #[structopt(name = "commit")]
    Commit {
        #[structopt(short = "a")]
        all: bool
    }
}

// 类型桥
struct GitIntoIterator {
    index: usize,
}

//自定义打印信息
impl fmt::Display for Git {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Git data={:?}", self)
    }
}

// 迭代器转化
impl IntoIterator for Git {
    type Item = Git;
    type IntoIter = GitIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        GitIntoIterator {
            index: 0,
        }
    }
}

// 为Into实现迭代器
impl Iterator for GitIntoIterator {
    type Item = Git;

    fn next(&mut self) -> Option<Git> {
        let result = match self.index {
            0 => Some(Git::Add{ interactive: false, patch: false}),
            1 => Some(Git::Fetch{ dry_run: false, all: false }),
            2 => Some(Git::Commit{ all: false }),
            _ => return None,
        };

        self.index += 1;
        return result;
    }
}


fn main() {
   
    let opt = Git::from_args();
    println!("opt data={}", format!("{}", opt));

    // 转移所有权
    let opt2 = opt;
    println!("opt2 data={}", format!("{}", opt2));

    // 模式匹配
    match opt2 {
        Git::Add{interactive, patch} => {
            println!("Add, interactive={:?}, patch={:?}", interactive, patch);
        },
        Git::Fetch{dry_run, all} => {
            println!("Fetch, dry_run={:?}, all={:?}", dry_run, all);
        },
        _ => {
            println!("{:?}", "other cmd");
        }
    }

    // 迭代器输出
    for cmd in optt {
        println!("iterator, {}", cmd)
    }
} {
            index: 0,
        }
    }
}

// 为Into实现迭代器
impl Iterator for GitIntoIterator {
    type Item = Git;

    fn next(&mut self) -> Option<Git> {
        let result = match self.index {
            0 => Some(Git::Add{ interactive: false, patch: false}),
            1 => Some(Git::Fetch{ dry_run: false, all: false }),
            2 => Some(Git::Commit{ all: false }),
            _ => return None,
        };

        self.index += 1;
        return result;
    }
}


fn main() {
   
    let opt = Git::from_args();
    println!("opt data={}", format!("{}", opt));

    // 转移所有权, opt也失去对引用内存的所有权
    let optt = opt;
    println!("optt data={}", format!("{}", optt));

    // 模式匹配
    match optt {
        Git::Add{interactive, patch} => {
            println!("Add, interactive={:?}, patch={:?}", interactive, patch);
        },
        Git::Fetch{dry_run, all} => {
            println!("Fetch, dry_run={:?}, all={:?}", dry_run, all);
        },
        _ => {
            println!("{:?}", "other cmd");
        }
    }

    // 迭代器输出
    for cmd in optt {
        println!("iterator, {}", cmd)
    }
}
