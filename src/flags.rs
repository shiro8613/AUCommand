use std::collections::HashMap;

#[derive(Debug)]
pub struct FlagParser {
    flag_map :HashMap<String, String>,
    raw_args :Vec<String>
}

pub struct FlagArguments {
    none_args   :Vec<String>,
    flag_args   :HashMap<String, Flag>
}

#[derive(Clone, Debug)]
pub struct Flag {
    typ :String,
    arg :String
}

pub struct FlagMap {
    m :HashMap<String, Flag>
}

impl FlagParser {
    pub fn new(args :Vec<String>) -> Self {
        FlagParser{
            flag_map: HashMap::new(),
            raw_args: args
        }
    }

    pub fn set_bool_flag(&mut self, flag_name :&str) -> &mut Self {
        self.flag_map.insert(String::from(flag_name), String::from("bool"));
        self
    }

    pub fn set_int_flag(&mut self, flag_name :&str) -> &mut Self {
        self.flag_map.insert(String::from(flag_name), String::from("int"));
        self
    }

    pub fn set_string_flag(&mut self, flag_name :&str) -> &mut Self {
        self.flag_map.insert(String::from(flag_name), String::from("string"));
        self
    }

    pub fn matches(&self) -> FlagArguments {
        let mut count = 0;
        let mut n_args :Vec<String> = Vec::new();
        let mut used_args :Vec<String> = Vec::new();
        let mut op :HashMap<String, Flag> = HashMap::new();

        for flag in &self.raw_args {
            let arg_a = flag.replace("-", "");
            if !flag.starts_with("-") {
                n_args.push(arg_a.clone());
                count += 1;
                continue;
            }

            for arg_c in arg_a.chars() {
                let arg = arg_c.to_string();
                match self.flag_map.get(arg.as_str()) {
                    None => break,
                    Some(x) => {
                        if x.eq("bool") {
                            op.insert(arg, Flag {
                                typ: x.to_string(),
                                arg: String::from("t")
                            });
                        } else {
                            if self.raw_args.len() > count + 1 {
                                let a = self.raw_args.get(count + 1);
                                match a {
                                    None => break,
                                    Some(v) => {
                                        used_args.push(v.clone());
                                        op.insert(arg, Flag {
                                            typ: x.to_string(),
                                            arg: v.to_string()
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                count += 1
            }
        }

        FlagArguments {
            none_args: n_args.iter()
                .filter(|&x| !used_args.contains(x))
                .map(|x| x.clone())
                .collect::<Vec<String>>(),
            flag_args: op
        }
    }

}

impl FlagArguments {
    pub fn none_args(&self) -> &Vec<String> {
        &self.none_args
    }

    pub fn flags(&self) -> FlagMap {
        FlagMap{
            m: self.flag_args.clone()
        }
    }
}

impl Flag {
    pub fn get_bool(&self) -> bool {
        if self.typ.eq("bool") {
            return self.arg.eq("t");
        }
        false
    }

    pub fn get_int(&self) -> usize {
        if self.typ.eq("int") {
            let pint = self.arg.parse::<usize>();
            if !pint.is_err() {
                return pint.unwrap();
            }
        }
        0
    }

    pub fn get_str(&self) -> String {
        if self.typ.eq("string") {
            return self.arg.to_string();
        }
        String::new()
    }
}

impl FlagMap {
    pub fn flag_func<F :Fn(&Flag)>(&self, key :&str, f :F) -> &Self {
        if self.m.get(key).is_some() {
            let flag = self.m.get(key).unwrap();
            f(flag);
        }

        self
    }
}