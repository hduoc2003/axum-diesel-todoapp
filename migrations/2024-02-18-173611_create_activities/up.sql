CREATE TABLE activities (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  task_id BIGINT NOT NULL,
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
  CONSTRAINT fk_activity_user
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT fk_activity_task
    FOREIGN KEY (task_id)
    REFERENCES tasks (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);

CREATE INDEX idx_activity_user ON activities (user_id);
CREATE INDEX idx_activity_task ON activities (task_id);
