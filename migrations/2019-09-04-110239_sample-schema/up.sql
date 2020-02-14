-- Diff code generated with pgModeler (PostgreSQL Database Modeler)
-- pgModeler version: 0.9.2-beta
-- Diff date: 2019-09-04 20:13:47
-- Source model: sample
-- Database: sample
-- PostgreSQL version: 10.0

-- [ Diff summary ]
-- Dropped objects: 0
-- Created objects: 3
-- Changed objects: 0
-- Truncated tables: 0

SET check_function_bodies = false;
-- ddl-end --

SET search_path=public,pg_catalog;
-- ddl-end --


-- [ Created objects ] --
-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
	id uuid NOT NULL,
	screen_name text NOT NULL,
	created_at timestamp NOT NULL,
	CONSTRAINT pk_users PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.users OWNER TO postgres;
-- ddl-end --

-- object: public.posts | type: TABLE --
-- DROP TABLE IF EXISTS public.posts CASCADE;
CREATE TABLE public.posts (
	id uuid NOT NULL,
	content text NOT NULL,
	id_users uuid NOT NULL,
	is_private boolean NOT NULL,
	CONSTRAINT pk_posts PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.posts OWNER TO postgres;
-- ddl-end --



-- [ Created foreign keys ] --
-- object: users_fk | type: CONSTRAINT --
-- ALTER TABLE public.posts DROP CONSTRAINT IF EXISTS users_fk CASCADE;
ALTER TABLE public.posts ADD CONSTRAINT users_fk FOREIGN KEY (id_users)
REFERENCES public.users (id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

