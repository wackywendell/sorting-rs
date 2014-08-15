#![warn(non_camel_case_types)]
#![warn(unnecessary_qualification)]
#![warn(non_uppercase_statics)]
#![warn(missing_doc)]

/*! Creates fake words.
 *  First, it uses a predefined list of words to generate a markov chain
 *  for word prefixes; it then probabilistically follows this to generate
 *  fake words.
 */
extern crate getopts;
extern crate collections;
extern crate rand;
use std::{os,result};

use std::from_str::FromStr;

use std::io::{File,BufferedReader,IoResult};
use collections::{HashMap,HashSet};
use rand::Rng;

//* Build a hashmap of "substring" : "number of occurrences"
fn toHashes(wordlist : &[String], sublens : uint) -> HashMap<String, uint> {
    let mut h : HashMap<String, uint> = HashMap::new();
    //h.insert_or_update_with(~"abc", 0, |_,n|{*n+=1;});
    for k in wordlist.iter() {
        if k.char_len() == 0 {continue};
        if k.contains("\'") {continue};
        if k.find(|c : char|{!c.is_lowercase()}).is_some() {continue};
        let fullword = (["^", k.trim().to_owned(), "$"]).concat();
        let fullchars : Vec<char> = fullword.chars().collect();
        for chars in fullchars.as_slice().windows(sublens){
            let _ = h.insert_or_update_with(
                    std::str::from_chars(chars), 1, |_,n|{*n+=1;});
        }
    }
    h
}

struct WordBuilder {
    subs : HashMap<String, uint>,
    list : Vec<String>,
    sublens : uint,
    wordset : HashSet<String>
}

struct WordIter<'a> {
    p : &'a mut WordBuilder,
    wmin : uint,
    wmax : uint,
}

impl WordBuilder {
    fn new(list : Vec<String>, sublens : uint) -> WordBuilder {
        let mut h = HashSet::with_capacity(list.len());
        for w in list.iter() {
            h.insert(w.to_owned());
        }
        WordBuilder {
            subs : toHashes(list.as_slice(), sublens), 
            list : list, 
            sublens : sublens,
            wordset : h
        }
    }
    
    fn word(&mut self, breakpoint : uint) -> Option<String> {
        let mut s = ~"^";
        
        loop {
            let mut fullsum = 0u;
            let possibilities : Vec<(&str, uint)> = self.subs.iter().filter_map(
                |(k,v)| {
                    let slen = s.char_len();
                    let klen = k.char_len();
                    let kcut = if slen < klen - 1 {slen} else {klen - 1};
                    if s.ends_with(k.slice_chars(0, kcut)){
                        fullsum += *v;
                        Some((k.as_slice(),*v))
                    } else {None}
                }
            ).collect();
            if fullsum == 0 {
                fail!("s: \"{}\"", s);
            }
            let randnum = rand::task_rng().gen_range(0u, fullsum);
            let mut psum = 0;
            //~ println!("Possibilities for '{}':", s);
            //~ for &(k,v) in possibilities.iter() {
                //~ println!("{}: {}", k, v);
            //~ }
            for &(k,v) in possibilities.iter() {
                if randnum < psum + v {
                    let slen = s.char_len();
                    let klen = k.char_len();
                    let kcut = if slen < klen - 1 {slen} else {klen - 1};
                    //~ let olds = s.to_owned();
                    s = s + k.slice_chars(kcut,klen);
                    //~ println!("({} / {} / {}: {}) {} -> {}", randnum, psum, fullsum, k, olds, s);
                    break;
                }
                //~ println!("({} / {} / {}: {} [{} {}])", randnum, psum, fullsum, k, psum, v);
                psum += v;
            }
            //~ s = possibilities[0].to_owned().first();
            let slen = s.char_len();
            if s.slice_chars(slen-1, slen) == "$" {
                return Some(s.slice_chars(1, slen-1).to_owned());
            }
            if breakpoint > 0 && slen - 2 >= breakpoint {
                return None;
            }
        };
    }
    
    fn iter<'a>(&'a mut self, wmin : uint, wmax : uint) -> WordIter<'a> {
        WordIter{p:self, wmin:wmin, wmax:wmax}
    }
}

impl<'a> Iterator<String> for WordIter<'a> {
    fn next(&mut self) -> Option<String> {
        loop {
            let optw = self.p.word(self.wmax);
            match optw {
                None => {},
                Some(w) => {
                    if (!self.p.wordset.contains_equiv(&w)) && (w.len() >= self.wmin)
                        {return Some(w)};
                }
            };
        }
    }
}

fn print_usage(program: &str, _opts: &[getopts::OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-n\tNUMBER\tLength of Markov chain");
    println!("-l\tNUMBER\tMinimum word length");
    println!("-h\tNUMBER\tMaximum word length");
}

fn get_val<T : FromStr>(program : &str, opts : &[getopts::OptGroup],
            matches : &getopts::Matches, optstr : &str, def : T) -> T{
    match matches.opt_str(optstr) {
        None => {def},
        Some(s) => match from_str(s) {
            None => {
                print_usage(program, opts);
                fail!("Could not transform option {}", optstr);
            }
            Some(n) => n
        }
    }
}

fn or_fail<T>(v : IoResult<T>) -> T {
    match v {
        Ok(m) => { m }
        Err(f) => { fail!("Error unwrapping, due to previous error: {}", f) }
    }
}

fn main(){
    let args = Vec::from_slice(os::args());
    let program = args.get(0).to_owned();
    
    let opts = [
        getopts::optflag("", "help", "Print this help message"),
        getopts::optopt("n", "", "Length of Markov chain", "NUMBER"),
        getopts::optopt("l", "", "Minimum word length", "NUMBER"),
        getopts::optopt("h", "", "Maximum word length", "NUMBER"),
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    if matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }
    
    let subsetn : uint = get_val(program, opts, &matches, "n", 4u);
    let wordmin : uint = get_val(program, opts, &matches, "l", 5u);
    let wordmax : uint = get_val(program, opts, &matches, "h", 10u);
    
    let pathstr = match matches.free.as_slice() {
            [] => &"/usr/share/dict/words",
            [ref s] => (*s).as_slice(),
            _ => fail!("too many words!")
        };
    
    let path = Path::new(pathstr);
    //~ let path = Path::new("fakewords-test.txt");
    let mut file = BufferedReader::new(File::open(&path));
    let lines_opt: IoResult<Vec<String> > = result::collect(
        file.lines().map(|orl| {
            orl.map(|l| {l.trim().to_owned()})
            })
        );
    let lines = or_fail(lines_opt);
    let mut wb = WordBuilder::new(lines, subsetn);
    
    for w in wb.iter(wordmin,wordmax){
        println!("{}", w);
    }
}
