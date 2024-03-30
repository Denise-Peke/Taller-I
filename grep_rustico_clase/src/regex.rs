use std::collections::VecDeque;



#[derive(Debug)]
pub struct RegexStep {
    value: RegexValue,
    repetition: RegexRep,
}

#[derive(Debug)]
pub struct Regex {
    steps: Vec<RegexStep>
}

#[derive(Debug)]
enum RegexValue {
    Literal(char), // literal
    Wildcard, // comodin
    //CharacterClass(RegexClase), //la clase de caracter
}

impl RegexValue {
    pub fn matches(&self, value:&str) -> usize{
        match self {
            RegexValue::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8() //Cantidad consumida en el input                     
                }else{
                    0 
                }
            },
            RegexValue::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8() //Cantidad consumida en el input                     
                }else{
                    0 
                }
            },
        }
    }
}

#[derive(Debug)]
enum RegexRep{
    Any,
    Exact(usize),
    Range(Option<usize>, Option<usize>),
}

impl Regex {

    pub fn new(expression: &str) -> Result<Self, &str>{

        let mut steps: Vec<RegexStep> = Vec::new();

        let mut chars_iter = expression.chars();
        while let Some(c) = chars_iter.next(){
            let step = match c {
                '.' => Some(RegexStep) { 
                        repetition: RegexRep::Exact(1), 
                        value: RegexValue::Wildcard
                    },
                'a' ..= 'z' => Some(RegexStep {
                    repetition: RegexRep::Exact(1),
                    value: RegexValue::Literal(c),
                    }),
                '*' => {
                    if let Some(last) = steps.last_mut(){
                        last.repetition = RegexRep::Any;
                    } else {
                        return Err("Se encontro un asterisco inesperado")
                    }
                },
                '\\' => match chars_iter.next(){
                    Some(literal) => Some(RegexStep { value: RegexValue::Literal(literal), repetition: RegexRep::Exact(1) }),
                    None => return Err("Se enconotro un caracter inesperado")
                },

                _ => return Err("Se encontró un caracter inesperado")
            
            };      
            };
      
        Ok(Regex {steps})
    }

    pub fn test(self, value:&str) -> Result<bool, &str>{
        if !value.is_ascii(){
            return Err("el input no es ASCII");
        }
       let mut queue = VecDeque::from(self.steps);

       let mut index = 0;

       while let Some(step) =  queue.pop_front(){
           match step.repetition {
            RegexRep::Any => {
                let mut keep_matching = true;
                while keep_matching {
                    let match_size = step.value.matches(&value[index..]);
                    if match_size != 0 {
                        index += match_size;
                    }else {
                        keep_matching = false;
                    }
                }
            }, //no se puede usar todo!
            RegexRep::Exact(n) => {
                for _ in [1,n] {
                    let size = step.value.matches(&value[index..]);

                    if size == 0 {
                        return Ok(false);
                    }
                    index += size;
                }
            },
            RegexRep::Range(_, _) => todo!(),
            }
       }
       Ok(true)
    }
}


