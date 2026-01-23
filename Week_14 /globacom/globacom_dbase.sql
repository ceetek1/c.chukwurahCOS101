--
-- PostgreSQL database dump
--

\restrict VKyZUJzF0BPk9OteSQkFw5jmbmMqCRr0oqkExKg8Q3JmtEkc07c42ftaiKjYWAD

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
-- Name: customer; Type: TABLE; Schema: public; Owner: chukwurahcharles
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(15) NOT NULL,
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO chukwurahcharles;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: chukwurahcharles
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO chukwurahcharles;

--
-- Name: department; Type: TABLE; Schema: public; Owner: chukwurahcharles
--

CREATE TABLE public.department (
    dept_mid integer NOT NULL,
    dept_no integer NOT NULL,
    dept_name text NOT NULL,
    dept_location character(50) NOT NULL,
    p_no integer NOT NULL
);


ALTER TABLE public.department OWNER TO chukwurahcharles;

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
-- Name: staff; Type: TABLE; Schema: public; Owner: chukwurahcharles
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    no integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO chukwurahcharles;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: chukwurahcharles
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail,com	08055089112	102	5
111	Lilian Jaiya	43	i_jaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	102	1
117	Okafor Mathais	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	a_adeleke@gmail.com	7056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: chukwurahcharles
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	200
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: chukwurahcharles
--

COPY public.department (dept_mid, dept_no, dept_name, dept_location, p_no) FROM stdin;
108	1	Aministration	Ikeja                                             	44
101	2	Account	Egbeda                                            	11
100	3	Packaging	Ajah                                              	44
120	4	Research	V.I                                               	33
97	5	Account	Magodo                                            	22
122	6	Operations	Mile 2                                            	44
107	7	Packaging	Ketu                                              	55
108	1	Aministration	Ikeja                                             	44
101	2	Account	Egbeda                                            	11
100	3	Packaging	Ajah                                              	44
120	4	Research	V.I                                               	33
97	5	Account	Magodo                                            	22
122	6	Operations	Mile 2                                            	44
107	7	Packaging	Ketu                                              	55
\.


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
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: chukwurahcharles
--

COPY public.staff (staff_id, staff_name, no, staff_sal, age, mobile) FROM stdin;
101	Alade joy	2	250000	33	08023089832
100	Mustapha Ali	3	175000	32	08063285831
107	Alokwe Martin	7	380000	48	07090082812
97	Dankade Aminat	5	555000	40	09023688832
108	Josiah Joshua	1	120000	30	08053189131
102	Makinde Mary	2	450000	55	09023487830
120	Adeleke Jane	4	200000	38	07061045862
122	Osahon Mark	6	320000	44	08022289842
117	Suleman Ajayi	3	800000	50	07030089811
104	Kuti Lawal	1	750000	35	09145689842
\.


--
-- Name: customer custumer_pkey; Type: CONSTRAINT; Schema: public; Owner: chukwurahcharles
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT custumer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: chukwurahcharles
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: chukwurahcharles
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict VKyZUJzF0BPk9OteSQkFw5jmbmMqCRr0oqkExKg8Q3JmtEkc07c42ftaiKjYWAD

