DROP INDEX IF EXISTS idx_repository_collaborators_repo_role;
DROP INDEX IF EXISTS idx_repository_collaborators_user;
DROP INDEX IF EXISTS idx_repository_topics_topic;
DROP INDEX IF EXISTS idx_repositories_last_update;
DROP INDEX IF EXISTS idx_repositories_stars;
DROP INDEX IF EXISTS idx_repositories_main_language;
DROP INDEX IF EXISTS idx_repositories_owner;
DROP INDEX IF EXISTS idx_repositories_name;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_username;

DROP TABLE IF EXISTS repository_collaborators;
DROP TABLE IF EXISTS repository_topics;
DROP TABLE IF EXISTS repositories;
DROP TABLE IF EXISTS users;