use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Quotes {
    pub phrase: &'static str,
    pub author: &'static str,
    pub quotes: &'static str,
}

static QUOTES: [Quotes; 6] = [
    Quotes {
        phrase: "It is not the drug, is their prohibition, which destroys society.",
        author: "Jesus Huerta de Soto",
        quotes: "Political Economy course at the Rey Juan Carlos University of Madrid. 2009-2010",
    },
    Quotes {
        phrase: "The State is an inherently illegitimate institution of organized aggression, 
        of organized and regularized crime against the persons and properties of its subjects.",
        author: "Murray N. Rothbard",
        quotes: "Ethics of Liberty (1982), p.187"
    },
    Quotes {
        phrase: "If history could prove and teach us anything, it would be that private 
        ownership of the means of production is a necessary requisite of civilization and 
        material well-being.",
        author: "Ludwig H. Edler von Mises",
        quotes: "Socialism: An Economic and Sociological Analysis (1936), p.583" 
    },
    Quotes {
        phrase: "Capitalism, thrift and hard work. There is no other way out of poverty.",
        author: "Miguel Anxo Bastos Boubeta",
        quotes: "Inter-University Seminar on Economics for Politics, UFM, Guatemala, 2012" 
    },
    Quotes {
        phrase: "I propose to call economic means the personal work of each
        individual and exchange equivalent to personal and individual labor for
        the work of others for the satisfaction of needs. On the other hand, the
        misappropriation of the work of other individuals shall be referred to as media
        politicians.",
        author: "Franz Oppenheimer",
        quotes: "The State (1922), p.36" 
    },
    Quotes {
        phrase: "War, trade and piracy is an inseparable trinity.",
        author: "Fausto J. W. von Goethe",
        quotes: "Fausto (1808)" 
    }
];
