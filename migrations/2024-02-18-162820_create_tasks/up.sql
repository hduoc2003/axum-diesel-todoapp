CREATE TYPE status_enum AS ENUM ('Pending', 'InProgress', 'Completed');

CREATE TABLE tasks (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  created_by BIGINT NOT NULL,
  updated_by BIGINT NOT NULL,
  title VARCHAR(512) NOT NULL,
  description VARCHAR(2048),
  status status_enum NOT NULL DEFAULT 'Pending',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT NULL,
  planned_start_date TIMESTAMP DEFAULT NULL,
  planned_end_date TIMESTAMP DEFAULT NULL,
  actual_start_date TIMESTAMP DEFAULT NULL,
  actual_end_date TIMESTAMP DEFAULT NULL,
  content TEXT,
  CONSTRAINT fk_task_user
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);

CREATE INDEX idx_task_user ON tasks (user_id);
