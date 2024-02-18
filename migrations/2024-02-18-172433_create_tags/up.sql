CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(75) NOT NULL,
    slug VARCHAR(100) NOT NULL
);

CREATE TABLE task_tags (
    task_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    CONSTRAINT fk_tt_task
        FOREIGN KEY (task_id)
        REFERENCES tasks (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
    CONSTRAINT fk_tt_tag
        FOREIGN KEY (tag_id)
        REFERENCES tags (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE INDEX idx_tt_task ON task_tags (task_id);
CREATE INDEX idx_tt_tag ON task_tags (tag_id);
