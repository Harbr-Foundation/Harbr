-- Create users table first since repositories will reference it
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR,
    avatar_url VARCHAR,
    bio TEXT,
    location VARCHAR,
    website VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create repositories table with owner reference
CREATE TABLE repositories (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    is_private BOOLEAN NOT NULL DEFAULT false,
    stars BIGINT NOT NULL DEFAULT 0,
    forks BIGINT NOT NULL DEFAULT 0,
    main_language VARCHAR NOT NULL,
    last_update TIMESTAMP NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(owner_id, name)
);

-- Create repository topics table
CREATE TABLE repository_topics (
    id INTEGER NOT NULL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    topic VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, topic)
);

-- Create repository collaborators junction table
CREATE TABLE repository_collaborators (
    id INTEGER NOT NULL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR NOT NULL CHECK (role IN ('READ', 'WRITE', 'ADMIN')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, user_id)
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);

CREATE INDEX idx_repositories_name ON repositories(name);
CREATE INDEX idx_repositories_owner ON repositories(owner_id);
CREATE INDEX idx_repositories_main_language ON repositories(main_language);
CREATE INDEX idx_repositories_stars ON repositories(stars DESC);
CREATE INDEX idx_repositories_last_update ON repositories(last_update DESC);

CREATE INDEX idx_repository_topics_topic ON repository_topics(topic);

CREATE INDEX idx_repository_collaborators_user ON repository_collaborators(user_id);
CREATE INDEX idx_repository_collaborators_repo_role ON repository_collaborators(repository_id, role);