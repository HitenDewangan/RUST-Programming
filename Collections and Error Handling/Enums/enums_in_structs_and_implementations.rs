enum JobStatus {
    Applied,
    Interviewing,
    Offered,
    Rejected,
}

struct Candidate {
    name: String,
    status: JobStatus,
}

impl Candidate {
    fn new(name: String, status: JobStatus) -> Candidate {
        Candidate { name, status }
    }

    fn get_status(&self) -> &str {
        match self.status {
            JobStatus::Applied => "Applied",
            JobStatus::Interviewing => "Interviewing",
            JobStatus::Offered => "Offered",
            JobStatus::Rejected => "Rejected",
        }
    }

    fn update_status(&mut self, new_status: JobStatus) {
        self.status = new_status;
    }
}

fn main() {
    let mut candidate = Candidate::new(String::from("Alice"), JobStatus::Applied);
    println!("{} is currently {}", candidate.name, candidate.get_status());

    candidate.update_status(JobStatus::Interviewing);
    println!("{} is currently {}", candidate.name, candidate.get_status());

    candidate.update_status(JobStatus::Offered);
    println!("{} is currently {}", candidate.name, candidate.get_status());
}