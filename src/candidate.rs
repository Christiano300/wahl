use std::fmt::Debug;

pub struct Candidate {
    pub name: String,
    pub points: u16,
    pub first_votes: u16,
}

impl Debug for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}: {} / {}", &self.name, self.points, self.first_votes)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Candidate {
    pub fn new(name: String) -> Self {
        Self {
            name,
            points: 0,
            first_votes: 0,
        }
    }

    pub fn first_vote(&mut self) {
        self.points += 2;
        self.first_votes += 1;
    }

    pub fn second_vote(&mut self) {
        self.points += 1;
    }
}
