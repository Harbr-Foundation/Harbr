struct Repository {
    name: String,
    description: Option<String>,
    is_private: bool,
    stars: u64,
    forks: u64,
    main_language: String,
    last_update: chrono::Utc,
    topics: Vec<String>,
}

type LanguageList = Vec<ProgrammingLanguage>;
struct ProgrammingLanguage {
    name: String,
    value: String,
    color: String,
}

struct User {
    
}