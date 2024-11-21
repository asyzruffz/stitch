use std::fmt;
use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::phrase::Phrase;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Statement {
    Noun {
        name: Rc<str>,
        super_type: Option<Datatype>,
        body: Rc<[Statement]>,
    },
    Verb {
        name: Rc<str>,
        hence_type: Option<Datatype>,
        subject_type: Option<Datatype>,
        object_types: Rc<[Statement]>,
        body: Rc<[Statement]>,
    },
    Adjective {
        name: Rc<str>,
        subject_type: Datatype,
        body: Rc<[Statement]>,
    },
    So {
        name: Rc<str>,
        datatype: Datatype,
        initializer: Option<Phrase>,
    },
    Phrase(Phrase),
    Hence(Phrase),
    //Block(Rc<[Statement]>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Noun { name, super_type: Some(supertype), .. } => write!(f, "noun {name} is {supertype} {{..}}"),
            Statement::Noun { name, super_type, .. } => write!(f, "noun {name} is {super_type:?} {{..}}"),
            Statement::Verb { name, hence_type: Some(hencetype), subject_type: Some(subjecttype), object_types, .. } => 
                write!(f, "verb {name} is {hencetype} for {subjecttype} when {} {{..}}",
                    object_types.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")),
            Statement::Verb { name, hence_type, subject_type, object_types, .. } => 
                write!(f, "verb {name} is {hence_type:?} for {subject_type:?} when {} {{..}}",
                    object_types.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")),
            Statement::Adjective { name, subject_type, .. } => write!(f, "adjective {name} for {subject_type}"),
            Statement::So { name, datatype, initializer: Some(init) } => write!(f, "so {name} is {datatype} as {init}"),
            Statement::So { name, datatype, initializer } => write!(f, "so {name} is {datatype} as {initializer:?}"),
            Statement::Phrase(phrase) => write!(f, "{phrase}."),
            Statement::Hence(phrase) => write!(f, "hence {phrase}."),
            //Statement::Block(stmnts) => write!(f, "{{{}\n}}", stmnts.as_ref().iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n\t")),
        }
    }
}
