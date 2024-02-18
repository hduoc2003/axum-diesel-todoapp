CREATE TABLE comments (
  id BIGSERIAL PRIMARY KEY,
  task_id BIGINT NOT NULL,
  activity_id BIGINT DEFAULT NULL,
  title VARCHAR(100) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT NULL,
  content TEXT DEFAULT NULL,
  CONSTRAINT fk_comment_task
    FOREIGN KEY (task_id)
    REFERENCES tasks (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT fk_comment_activity
    FOREIGN KEY (activity_id)
    REFERENCES activities (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);

CREATE INDEX idx_comment_task ON comments (task_id);
CREATE INDEX idx_comment_activity ON comments (activity_id);
