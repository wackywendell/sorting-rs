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
use std::{os,result};

use std::from_str::FromStr;

use std::io::{File,BufferedReader,IoResult};
use collections::{TreeMap,TreeSet};
use std::rand;
use std::rand::Rng;

fn get_counts<K : Ord, I : Iterator<K>>(list : I) -> TreeMap<K, uint> {
	let mut counter : TreeMap<K, uint> = TreeMap::new();
	for key in list {
		// TODO: if key is not in the tree, this looks for its position twice
		//       there used to be an 'insert_or_update_with' method
		match counter.find_mut(&key){
			None => {counter.insert(key, 1);},
			Some(value) => {*value += 1;}
		}
	}
	counter
}

//* Build a TreeMap of "substring" : "number of occurrences"
fn toHashes(wordlist : &[String], sublens : uint) -> TreeMap<String, uint> {
	let iter = wordlist.iter().take_while(|k| {
		let kslice = k.as_slice();
		if kslice.char_len() == 0 {false}
		else if kslice.contains("\'") {false}
		else if kslice.find(|c : char|{!c.is_lowercase()}).is_some() {false}
		else {true}
	}).flat_map(|k| {
		let kslice = k.as_slice();
		let fullword = (["^", kslice.trim_chars(&[' ', '\t', '\r', '\n']), "$"]).concat();
        let fullchars : Vec<char> = fullword.as_slice().chars().collect();
		fullchars.as_slice().windows(sublens).map(|chars| {
			std::str::from_chars(chars)
		})
	});
	get_counts(iter)
}

struct WordBuilder {
    subs : TreeMap<String, uint>,
    list : Vec<String>,
    sublens : uint,
    wordset : TreeSet<String>,
    wordlens : Vec<uint>
}

struct WordIter<'a> {
    p : &'a mut WordBuilder
}

impl WordBuilder {
    fn new(list : Vec<String>, sublens : uint) -> WordBuilder {
        let mut h = TreeSet::new();
        let mut wlens : Vec<uint> = Vec::new();
        for w in list.iter() {
            h.insert(w.to_owned());
            let wordlen = w.len();
            for _ in range(wlens.len(), wordlen+1){
                wlens.push(0);
            }
            let n = wlens.get_mut(wordlen);
            *n += 1;
        }
        
        WordBuilder {
            subs : toHashes(list.as_slice(), sublens), 
            list : list, 
            sublens : sublens,
            wordset : h,
            wordlens : wlens
        }
    }
    
    fn word(&mut self) -> Option<String> {
        let mut s : String = String::from_str("^");
        
        loop {
            let mut fullsum = 0u;
            let mut endsum = 0u;
            let possibilities : Vec<(&str, uint)> = self.subs.iter().filter_map(
                |(k,v)| {
                    /* the beginning of k and the end of s must match for
                     * k to be a possibility
                     * if s is long, then the first (klength - 1) letters
                     * of k must match the last (klength - 1) letters of s
                     * otherwise, the first (slength) characters
                    */
                    let slen = s.as_slice().char_len();
                    let kslice = k.as_slice();
                    let klen = kslice.char_len();
                    let kcut = if slen < klen - 1 {slen} else {klen - 1};
                    if s.as_slice().ends_with(kslice.slice_chars(0, kcut)){
                        fullsum += *v;
                        if kslice.ends_with("$") {endsum += *v;}
                        Some((kslice,*v))
                    } else {None}
                }
            ).collect();
            if fullsum == 0 {
                fail!("s: \"{}\"", s);
            }
            
            let endprob = if self.wordlens.len() > s.len() {
                let wordlenslice : &[uint] = self.wordlens.slice(s.len()-1, self.wordlens.len());
                let c = wordlenslice[0];
                let l = wordlenslice.iter().fold(0, |a,&b|{a+b});
                (c as f64) / (l as f64)
            } else {
                //~ println!("Too long: {}", s);
                return None;
            };
            
            //~ println!("s: {}", s);
            //~ println!("lens: {}, {}", curlens, lensum);
            //~ print!("wordlens: ");
            //~ for &w in self.wordlens.iter() {
                //~ print!(" {}", w);
            //~ }
            //~ println("");
            //~ println!("sums: {}, {}", endsum, fullsum);
            
            let randnum = rand::task_rng().gen_range(0.0, 1.0);
            //~ let endtime = match (endsum, fullsum-endsum) {
                //~ (0,0) => {return None;},
                //~ (0,_) => {false},
                //~ (_,0) => {true},
                //~ _ => (randnum < endprob)
            //~ };
            
            let endtime = randnum < endprob;
            if (endtime && (endsum == 0)) || ((!endtime) && (fullsum-endsum==0)) {
                //~ println!("Failed to end: {}", s);
                return None;
            }
            
            //~ println!("endtime: {} {} : ({},{}) {}", endtime, randnum < endprob,
                //~ endsum, fullsum, if(endtime){endsum} else {fullsum - endsum});
            let randnum = rand::task_rng().gen_range(0.0, 
                (if endtime {endsum} else {fullsum - endsum} as f64));
            
            let mut psum = 0;
            
            for &(k,v) in possibilities.iter() {
                if endtime ^ k.ends_with("$") {continue;};
                
                if randnum < ((psum + v) as f64) {
                    let slen = s.as_slice().char_len();
                    let klen = k.char_len();
                    let kcut = if slen < klen - 1 {slen} else {klen - 1};
                    //~ let olds = s.to_owned();
                    s.push_str(k.slice_chars(kcut,klen));
                    break;
                }
                psum += v;
            }
            //~ s = possibilities[0].to_owned().first();
            let slen = s.as_slice().char_len();
            if s.as_slice().slice_chars(slen-1, slen) == "$" {
                return Some(s.as_slice().slice_chars(1, slen-1).to_owned());
            }
            //~ if (breakpoint > 0 && slen - 2 >= breakpoint) {
                //~ return None;
            //~ }
        };
    }
    
    fn iter<'a>(&'a mut self) -> WordIter<'a> {
        WordIter{p:self}
    }
}

impl<'a> Iterator<String> for WordIter<'a> {
    fn next(&mut self) -> Option<String> {
        loop {
            let optw = self.p.word();
            match optw {
                None => {},
                Some(w) => {
                    if !self.p.wordset.contains(&w)
                        {return Some(w)};
                }
            };
        }
    }
}

fn print_usage(program: &str, _opts: &[getopts::OptGroup]) {
    println!("Usage: {} [-n NUMBER] [DICTFILE]", program);
    println!("-n\tNUMBER\tLength of Markov chain");
    println!("\tDICTFILE\tDictionary to use for generation");
}

fn get_val<T : FromStr>(program : &str, opts : &[getopts::OptGroup],
            matches : &getopts::Matches, optstr : &str, def : T) -> T{
    match matches.opt_str(optstr) {
        None => {def},
        Some(s) => match s {
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
    let args : Vec<String> = os::args();
    let program = args.get(0).as_slice();
    
    let opts = [
        getopts::optflag("h", "help", "Print this help message"),
        getopts::optopt("n", "", "Length of Markov chain", "NUMBER"),
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
    
    for w in wb.iter(){
        println!("{}",w);
    }
}
