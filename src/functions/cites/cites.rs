use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Cites {
    pub phrase: &'static str,
    pub author: &'static str,
    pub cite: &'static str,
}

static CITES: [Cites; 1] = [
    Cites {
        phrase: "It is not the drug, is their prohibition, which destroys society.",
        author: "Jesus Huerta de Soto",
        cite: "Political Economy course at the Rey Juan Carlos University of Madrid. 2009-2010",
    }
];
