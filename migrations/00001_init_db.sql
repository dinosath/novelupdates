



CREATE EXTENSION IF NOT EXISTS pgcrypto;



CREATE OR REPLACE FUNCTION update_last_updated_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS public.artist (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    bio VARCHAR
    ,
    name VARCHAR
    ,
    native_name VARCHAR
    ,
    novels VARCHAR
    ,
    website VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.artist;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.artist
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.author (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    bio VARCHAR
    ,
    country VARCHAR
    ,
    name VARCHAR
    ,
    native_name VARCHAR
    ,
    novels VARCHAR
    ,
    website VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.author;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.author
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.chapter (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    content_url VARCHAR
    ,
    is_locked BOOLEAN
    ,
    language VARCHAR
    ,
    novels VARCHAR
    ,
    number VARCHAR
    ,
    part INTEGER
    ,
    release_date VARCHAR
    ,
    source VARCHAR
    ,
    title VARCHAR
    ,
    views INTEGER
    ,
    volume INTEGER
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.chapter;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.chapter
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
        CREATE TYPE status AS ENUM ('active', 'inactive', 'disbanded');
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS public.group (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    description VARCHAR
    ,
    discord_url VARCHAR
    ,
    founded_date VARCHAR
    ,
    language VARCHAR
    ,
    logo_url VARCHAR
    ,
    member_count INTEGER
    ,
    name VARCHAR
    ,
    patreon_url VARCHAR
    ,
    sources VARCHAR
    ,
    status VARCHAR
    ,
    website VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.group;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.group
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status-origin') THEN
        CREATE TYPE status-origin AS ENUM ('ongoing', 'completed', 'hiatus', 'cancelled');
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS public.novel (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    alternative_names VARCHAR
    ,
    artists VARCHAR
    ,
    authors VARCHAR
    ,
    average_rating VARCHAR
    ,
    chapters VARCHAR
    ,
    completely_translated BOOLEAN
    ,
    country_of_origin VARCHAR
    ,
    cover_image_url VARCHAR
    ,
    default_name VARCHAR
    ,
    description VARCHAR
    ,
    english_publisher VARCHAR
    ,
    genres VARCHAR
    ,
    licensed BOOLEAN
    ,
    native_name VARCHAR
    ,
    original_language VARCHAR
    ,
    publishers VARCHAR
    ,
    rating_count INTEGER
    ,
    reading_lists VARCHAR
    ,
    release_frequency VARCHAR
    ,
    reviews VARCHAR
    ,
    sources VARCHAR
    ,
    status_origin VARCHAR
    ,
    tags VARCHAR
    ,
    total_chapters INTEGER
    ,
    type VARCHAR
    ,
    views INTEGER
    ,
    year INTEGER
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.novel;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.novel
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.publisher (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    country VARCHAR
    ,
    description VARCHAR
    ,
    name VARCHAR
    ,
    novels VARCHAR
    ,
    website VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.publisher;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.publisher
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
        CREATE TYPE status AS ENUM ('reading', 'completed', 'plan-to-read', 'on-hold', 'dropped');
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS public.reading_list (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    completed_date VARCHAR
    ,
    current_chapter INTEGER
    ,
    last_read VARCHAR
    ,
    notes VARCHAR
    ,
    novel VARCHAR
    ,
    personal_rating VARCHAR
    ,
    started_date VARCHAR
    ,
    status VARCHAR
    ,
    user VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.reading_list;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.reading_list
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.review (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    content VARCHAR
    ,
    helpful_count INTEGER
    ,
    novel VARCHAR
    ,
    rating VARCHAR
    ,
    spoiler BOOLEAN
    ,
    title VARCHAR
    ,
    user VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.review;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.review
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
        CREATE TYPE status AS ENUM ('active', 'inactive', 'completed', 'dropped');
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS public.source (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    chapter_count INTEGER
    ,
    chapters VARCHAR
    ,
    completely_translated BOOLEAN
    ,
    group VARCHAR
    ,
    is_official BOOLEAN
    ,
    language VARCHAR
    ,
    name VARCHAR
    ,
    novel VARCHAR
    ,
    status VARCHAR
    ,
    url VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.source;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.source
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'category') THEN
        CREATE TYPE category AS ENUM ('genre', 'theme', 'plot', 'character', 'setting', 'other');
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS public.tag (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    category VARCHAR
    ,
    description VARCHAR
    ,
    name VARCHAR
    ,
    novels VARCHAR
    ,
    slug VARCHAR
    ,
    usage_count INTEGER
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.tag;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.tag
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.type (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    description VARCHAR
    ,
    name VARCHAR
    ,
    novels VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.type;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.type
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

CREATE TABLE IF NOT EXISTS public.user (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    ,
    avatar_url VARCHAR
    ,
    bio VARCHAR
    ,
    display_name VARCHAR
    ,
    email VARCHAR
    ,
    joined_date VARCHAR
    ,
    last_active VARCHAR
    ,
    password_hash VARCHAR
    ,
    reading_lists VARCHAR
    ,
    reviews VARCHAR
    ,
    username VARCHAR
    );

DROP TRIGGER IF EXISTS set_last_updated ON public.user;
CREATE TRIGGER set_last_updated
    BEFORE UPDATE ON public.user
    FOR EACH ROW EXECUTE FUNCTION update_last_updated_column();

-- m2m junction tables (composite primary keys of the two foreign keys)
CREATE TABLE IF NOT EXISTS Artist_Novel (
    artist_id INTEGER NOT NULL REFERENCES public.artist(id) ON DELETE CASCADE,
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    PRIMARY KEY (artist_id, novel_id)
);
CREATE INDEX IF NOT EXISTS idx_Artist_Novel_novel_id_artist_id ON Artist_Novel(novel_id,artist_id);
CREATE TABLE IF NOT EXISTS Author_Novel (
    author_id INTEGER NOT NULL REFERENCES public.author(id) ON DELETE CASCADE,
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    PRIMARY KEY (author_id, novel_id)
);
CREATE INDEX IF NOT EXISTS idx_Author_Novel_novel_id_author_id ON Author_Novel(novel_id,author_id);
CREATE TABLE IF NOT EXISTS Chapter_Novel (
    chapter_id INTEGER NOT NULL REFERENCES public.chapter(id) ON DELETE CASCADE,
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    PRIMARY KEY (chapter_id, novel_id)
);
CREATE INDEX IF NOT EXISTS idx_Chapter_Novel_novel_id_chapter_id ON Chapter_Novel(novel_id,chapter_id);
CREATE TABLE IF NOT EXISTS Novel_Publisher (
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    publisher_id INTEGER NOT NULL REFERENCES public.publisher(id) ON DELETE CASCADE,
    PRIMARY KEY (novel_id, publisher_id)
);
CREATE INDEX IF NOT EXISTS idx_Novel_Publisher_publisher_id_novel_id ON Novel_Publisher(publisher_id,novel_id);
CREATE TABLE IF NOT EXISTS Novel_ReadingList (
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    reading_list_id INTEGER NOT NULL REFERENCES public.reading_list(id) ON DELETE CASCADE,
    PRIMARY KEY (novel_id, reading_list_id)
);
CREATE INDEX IF NOT EXISTS idx_Novel_ReadingList_reading_list_id_novel_id ON Novel_ReadingList(reading_list_id,novel_id);
CREATE TABLE IF NOT EXISTS Novel_Tag (
    novel_id INTEGER NOT NULL REFERENCES public.novel(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES public.tag(id) ON DELETE CASCADE,
    PRIMARY KEY (novel_id, tag_id)
);
CREATE INDEX IF NOT EXISTS idx_Novel_Tag_tag_id_novel_id ON Novel_Tag(tag_id,novel_id);
