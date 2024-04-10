-- Your SQL goes here
CREATE TABLE feeds (
    id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id),
	name VARCHAR NOT NULL,
	UNIQUE(user_id,name)
);

CREATE TABLE feedSource (
	id SERIAL PRIMARY KEY,
	feed_id INTEGER NOT NULL REFERENCES feeds(id),
	source_id INTEGER NOT NULL REFERENCES sources(id),
	UNIQUE(feed_id,source_id)
);

CREATE OR REPLACE FUNCTION relevance_score(q text, body text) 
	RETURNS INTEGER
	LANGUAGE plpgsql
	as $$
DECLARE
	score int8 := 0;
	search_word text;
	query_word text;
BEGIN
	FOREACH search_word in array string_to_array(lower(trim(q)),' ')
	LOOP
		FOREACH query_word in array string_to_array(lower(trim(body)),' ')
		LOOP
			IF (search_word=query_word) THEN
				score := score + 30;
				CONTINUE;
			END IF;
			IF (starts_with(query_word,search_word)) THEN
				score := score + 10;
				CONTINUE;
			END IF;
			IF (query_word LIKE '%' || search_word || '%') THEN
				score := score + 1;
				CONTINUE;
			END IF;
		END LOOP;
		score := score + length(search_word);
	END LOOP;
	RETURN score;
END;
$$;