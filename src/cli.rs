#[derive(Debug)]
pub struct Args {
    pub mode: Mode
}

#[derive(Debug)]
pub enum Mode {
    Help,
    Create(CreateOptions),
    Remote(String),
    List,
    Edit(EditOptions)
}

#[derive(Debug)]
pub struct CreateOptions {
    pub name: String,
    pub priority: u8,
    pub tags: Option<Box<[String]>>,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct EditOptions {

}

#[derive(Debug)]
pub enum ParseError {
    UnkownCommand(String),
    MissingCommand(String),
    InvalidInput(String),
}

impl Args {
    pub fn from_args<I>(mut args: I) -> Result<Self, ParseError>
    where I: Iterator<Item = String>
    {
        args.next().expect("If this failed something is seriously fucked up");

        let cmd = match args.next() {
            Some(v) => v,
            None    => return Ok(Self {mode: Mode::Help}),
        };

        match &cmd[..] {
            "-c" | "--create" => Ok(Self { mode: Mode::Create(CreateOptions::from_args(args)?) }),

            "-h" | "--help" => Ok(Self { mode: Mode::Help  }),

            "-l" | "--list" => Ok(Self { mode: Mode::List }),

            "-r" | "--remove" => Ok(Self { mode: Mode::Remote (
                args.next().ok_or(ParseError::MissingCommand("-c [name] - Please specify the name of the issue to be removed".into()))?
            )}),

            "-e" | "--edit" => todo!(),

            _ => Err(ParseError::UnkownCommand(cmd))
        }
    }
}

impl CreateOptions {
    pub fn from_args<I>(mut args: I) -> Result<Self, ParseError>
    where I: Iterator<Item = String> 
    {
        let mut name = None;
        let mut priority = None;
        let mut tags = None;
        let mut description = None;

        let mut cmd = args.next();
        'outer: while cmd.is_some() {
            
            match &cmd.as_mut().unwrap()[..] {
                // Parse priority
                "-p" | "--priority" => {
                    let v = args.next().ok_or(ParseError::MissingCommand("-p [n] - Please insert a number".into()))?;
                    let n = v.parse::<u8>().map_err(|_| 
                        ParseError::InvalidInput("Could not parse to a u8 number".into())
                    )?;

                    priority = Some(n)
                },

                // Parse tags
                "-t" | "--tags" => {
                    let mut tag_bfr = Vec::with_capacity(10);

                    while let Some(v) = args.next() {
                        match &v[..] {
                            "-p" | "--priority" | "-d" | "--description" => {
                                tags = Some(tag_bfr.into_boxed_slice());
                                cmd = Some(v);
                                continue 'outer;
                            },
                            _ => {
                                tag_bfr.push(v);
                            }
                        }
                    }

                    tags = Some(tag_bfr.into_boxed_slice())
                },

                // Parse description
                "-d" | "--description" => {
                    description = Some(args.next().unwrap_or(String::new()));
                }

                // Parse the name of the issue
                _ if name.is_none() => {
                    name = Some(cmd.unwrap());
                },
                
                _ => { return Err(ParseError::UnkownCommand(cmd.unwrap())) }
            }
            
            cmd = args.next();
        }

        
        Ok(Self {
            name: name.ok_or(
                ParseError::MissingCommand("Please select a name for the issue".into())
            )?,

            priority: priority.unwrap_or(127),
            tags,
            description
        })
    }
}