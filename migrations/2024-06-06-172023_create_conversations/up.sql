CREATE TABLE threads (
    id VARCHAR(255) PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    thread_id VARCHAR(255) REFERENCES threads(id) ON DELETE CASCADE,
    content TEXT,
    role VARCHAR NOT NULL DEFAULT 'user',
    active_model VARCHAR NOT NULL,
    active_lab VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);
