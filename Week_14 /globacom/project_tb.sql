--
-- PostgreSQL database dump
--

\restrict M2yt1Wib7mUPCx5dGBQifTaWZnSMbB4LGuZdMeMwrA9ihxcfdzQrhKtI1khkZ2H

-- Dumped from database version 14.20 (Homebrew)
-- Dumped by pg_dump version 14.20 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: project; Type: TABLE; Schema: public; Owner: chukwurahcharles
--

CREATE TABLE public.project (
    p_no integer NOT NULL,
    p_name text NOT NULL,
    p_duration character(50) NOT NULL,
    p_manageid integer NOT NULL
);


ALTER TABLE public.project OWNER TO chukwurahcharles;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: chukwurahcharles
--

COPY public.project (p_no, p_name, p_duration, p_manageid) FROM stdin;
11	A	9 months                                          	102
22	B	14 months                                         	97
33	C	16 months                                         	120
44	D	25 months                                         	108
55	E	9 months                                          	107
11	A	9 months                                          	102
22	B	14 months                                         	97
33	C	16 months                                         	120
44	D	25 months                                         	108
55	E	9 months                                          	107
\.


--
-- PostgreSQL database dump complete
--

\unrestrict M2yt1Wib7mUPCx5dGBQifTaWZnSMbB4LGuZdMeMwrA9ihxcfdzQrhKtI1khkZ2H

