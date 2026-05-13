CREATE TABLE reviews (
    id UUID PRIMARY KEY,
    book_id UUID NOT NULL,
    user_id UUID NOT NULL,
    rating SMALLINT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX reviews_book_id_idx ON reviews (book_id);
